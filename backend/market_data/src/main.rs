use backend::shared::kafka_producer::publish_to_kafka;
use backend::shared::mmap_buffer::write_to_mmap;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

const ALPACA_WS_URL: &str = "wss://stream.data.alpaca.markets/v2/iex";
const API_KEY: &str = "PKS83YQBEUPZL111E7NJ";
const SECRET_KEY: &str = "NDSora4h27DyMzn1vgRElYWr40gkDpkZTrzIXwvh";
const KAFKA_TOPIC: &str = "market_data"; // Kafka topic for publishing data

#[tokio::main]
async fn main() {
    // let mmap = get_mmap(); // Use `get_mmap()` directly, no extra wrapping

    let url = Url::parse(ALPACA_WS_URL).expect("Invalid WebSocket URL");
    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to Alpaca WebSocket");

    let (mut write, mut read) = ws_stream.split();

    authenticate_and_subscribe(&mut write).await;

    println!("[MarketData] ✅ Subscribed to Alpaca market feed.");

    while let Some(msg) = read.next().await {
        if let Ok(Message::Text(text)) = msg {
            let text = text.clone();
            // let mmap = Arc::clone(&mmap);

            tokio::spawn(async move {
                process_market_data(&text).await;
            });
        }
    }
}

async fn authenticate_and_subscribe<S>(write: &mut S)
where
    S: SinkExt<Message> + Unpin,
    <S as futures_util::Sink<Message>>::Error: std::fmt::Debug,
{
    let auth_msg = format!(
        r#"{{"action": "auth", "key": "{}", "secret": "{}"}}"#,
        API_KEY, SECRET_KEY
    );
    if let Err(e) = write.send(Message::Text(auth_msg)).await {
        eprintln!("[MarketData] ❌ Auth failed: {:?}", e);
    }

    let subscribe_msg =
        r#"{"action":"subscribe","trades":["AAPL","TSLA"],"quotes":["AAPL","TSLA"]}"#;
    if let Err(e) = write.send(Message::Text(subscribe_msg.into())).await {
        eprintln!("[MarketData] ❌ Subscribe failed: {:?}", e);
    }
}

/// Processes an array of market data and writes it to memory-mapped buffer & Kafka.
async fn process_market_data(text: &str) {
    if let Ok(json_array) = serde_json::from_str::<Vec<Value>>(text) {
        for json_msg in json_array {
            let json_str = json_msg.to_string();

            // Write to Shared Memory-Mapped Buffer (Await async function)
            {
                // let mmap_guard = mmap.lock().await;
                write_to_mmap(&json_str).await;
                println!("[MarketData] ✅ Data written to mmap.");
            }

            // Publish full JSON object to Kafka
            publish_to_kafka(KAFKA_TOPIC, &json_str).await;
        }
    } else {
        eprintln!(
            "[MarketData] ❌ Failed to parse WebSocket message as JSON array: {}",
            text
        );
    }
}
