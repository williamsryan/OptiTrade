use futures_util::stream::StreamExt;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use serde_json::Value;
use std::collections::VecDeque;
use std::time::Duration;
use tokio_postgres::{Client, Error, NoTls};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const BINANCE_WS_URL: &str = "wss://fstream.binance.com/ws/bnbusdt@aggTrade";
const KAFKA_TOPIC: &str = "market_data.binance";

/// Struct for computing VWAP (Volume Weighted Average Price)
struct VWAPCalculator {
    sum_price_volume: f64,
    sum_volume: f64,
}

impl VWAPCalculator {
    fn new() -> Self {
        Self {
            sum_price_volume: 0.0,
            sum_volume: 0.0,
        }
    }

    fn update(&mut self, price: f64, volume: f64) {
        self.sum_price_volume += price * volume;
        self.sum_volume += volume;
    }

    fn get_vwap(&self) -> f64 {
        if self.sum_volume == 0.0 {
            0.0
        } else {
            self.sum_price_volume / self.sum_volume
        }
    }
}

/// Struct for computing Order Book Imbalance
struct OrderBookImbalance {
    bid_volume: f64,
    ask_volume: f64,
}

impl OrderBookImbalance {
    fn new() -> Self {
        Self {
            bid_volume: 0.0,
            ask_volume: 0.0,
        }
    }

    fn update(&mut self, bids: &Vec<(f64, f64)>, asks: &Vec<(f64, f64)>) {
        self.bid_volume = bids.iter().map(|(_, vol)| vol).sum();
        self.ask_volume = asks.iter().map(|(_, vol)| vol).sum();
    }

    fn get_imbalance(&self) -> f64 {
        if self.bid_volume + self.ask_volume == 0.0 {
            0.0
        } else {
            (self.bid_volume - self.ask_volume) / (self.bid_volume + self.ask_volume)
        }
    }
}

async fn connect_db() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=optitrade password=secret dbname=market_data",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS order_book (
            timestamp TIMESTAMPTZ PRIMARY KEY DEFAULT NOW(),
            best_bid FLOAT NOT NULL,
            best_ask FLOAT NOT NULL,
            vwap FLOAT NOT NULL,
            imbalance FLOAT NOT NULL
        )
    ",
        )
        .await?;

    Ok(client)
}

async fn insert_order_book_data(
    client: &Client,
    best_bid: f64,
    best_ask: f64,
    vwap: f64,
    imbalance: f64,
) {
    client
        .execute(
            "INSERT INTO order_book (best_bid, best_ask, vwap, imbalance) VALUES ($1, $2, $3, $4)",
            &[&best_bid, &best_ask, &vwap, &imbalance],
        )
        .await
        .expect("Failed to insert data");
}

pub async fn run_market_data_agent(producer: FutureProducer, db_client: &Client) {
    let (ws_stream, _) = connect_async(BINANCE_WS_URL)
        .await
        .expect("Failed to connect to Binance WebSocket");

    let (_, mut read) = ws_stream.split();
    let mut vwap_calculator = VWAPCalculator::new();
    let mut order_book_imbalance = OrderBookImbalance::new();

    println!("[MarketData] Connected to Binance WebSocket feed.");

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(json_msg) = serde_json::from_str::<Value>(&text) {
                    let bids = json_msg["b"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|b| {
                            (
                                b[0].as_str().unwrap().parse::<f64>().unwrap(),
                                b[1].as_str().unwrap().parse::<f64>().unwrap(),
                            )
                        })
                        .collect::<Vec<_>>();

                    let asks = json_msg["a"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|a| {
                            (
                                a[0].as_str().unwrap().parse::<f64>().unwrap(),
                                a[1].as_str().unwrap().parse::<f64>().unwrap(),
                            )
                        })
                        .collect::<Vec<_>>();

                    // Compute VWAP
                    if let Some(best_bid) = bids.first() {
                        vwap_calculator.update(best_bid.0, best_bid.1);
                    }
                    if let Some(best_ask) = asks.first() {
                        vwap_calculator.update(best_ask.0, best_ask.1);
                    }

                    let vwap = vwap_calculator.get_vwap();
                    println!("[MarketData] VWAP: {:.2}", vwap);

                    // Compute Order Book Imbalance
                    order_book_imbalance.update(&bids, &asks);
                    let imbalance = order_book_imbalance.get_imbalance();
                    println!("[MarketData] Order Book Imbalance: {:.4}", imbalance);

                    // ✅ Insert data into database
                    insert_order_book_data(db_client, bids[0].0, asks[0].0, vwap, imbalance).await;

                    // ✅ Publish to Kafka
                    let record = FutureRecord::to(KAFKA_TOPIC)
                        .payload(&serde_json::to_string(&json_msg).unwrap())
                        .key("order_book");

                    let timeout = Timeout::from(Duration::from_secs(5));
                    let _ = producer.send(record, timeout).await;
                }
            }
            Err(e) => eprintln!("[MarketData] WebSocket Error: {:?}", e),
            _ => {}
        }
    }
}
