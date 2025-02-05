use futures_util::stream::StreamExt;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use serde_json::Value;
use std::time::Duration;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const BINANCE_WS_URL: &str = "wss://fstream.binance.com/ws/bnbusdt@aggTrade";
const KAFKA_TOPIC: &str = "market_data.binance";

pub async fn run_market_data_agent(producer: FutureProducer) {
    let (ws_stream, _) = connect_async(BINANCE_WS_URL)
        .await
        .expect("Failed to connect to Binance WebSocket");

    let (_, mut read) = ws_stream.split();

    println!("[MarketData] Connected to Binance WebSocket feed.");

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(json_msg) = serde_json::from_str::<Value>(&text) {
                    println!("[MarketData] New Order Book Update: {:?}", json_msg);

                    let record = FutureRecord::to(KAFKA_TOPIC)
                        .payload(&text)
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
