mod kafka_producer;
use kafka_producer::publish_to_kafka;

use futures_util::{SinkExt, StreamExt};
use memmap2::MmapMut;
use serde_json::Value;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

const ALPACA_WS_URL: &str = "wss://stream.data.alpaca.markets/v2/iex";
const API_KEY: &str = "PKS83YQBEUPZL111E7NJ";
const SECRET_KEY: &str = "NDSora4h27DyMzn1vgRElYWr40gkDpkZTrzIXwvh";
const KAFKA_TOPIC: &str = "market_data"; // Kafka topic for publishing data
const MMAP_FILE: &str = "/tmp/market_data_mmap"; // Memory-mapped buffer file path
const MMAP_SIZE: usize = 10 * 1024 * 1024; // 10MB buffer for market data

#[tokio::main]
async fn main() {
    let mmap = Arc::new(Mutex::new(setup_mmap_buffer()));

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
            let mmap = Arc::clone(&mmap);

            // println!("[MarketData] 📊 Received market data: {}", text);

            tokio::spawn(async move {
                process_market_data(&text, mmap).await;
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

/// Sets up a memory-mapped buffer for ultra-fast market data access.
fn setup_mmap_buffer() -> MmapMut {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(MMAP_FILE)
        .expect("Failed to open memory-mapped buffer file");

    file.set_len(MMAP_SIZE as u64)
        .expect("Failed to set mmap file size");

    unsafe { MmapMut::map_mut(&file).expect("Failed to map memory") }
}

/// Processes an array of market data and writes it to memory-mapped buffer & Kafka.
async fn process_market_data(text: &str, mmap: Arc<Mutex<MmapMut>>) {
    if let Ok(json_array) = serde_json::from_str::<Vec<Value>>(text) {
        for json_msg in json_array {
            let json_str = json_msg.to_string(); // ✅ Convert full JSON object to string

            // ✅ Write to Memory-Mapped Buffer (FlatBuffers)
            {
                let mut mmap = mmap.lock().unwrap();
                let data = json_str.as_bytes();
                let len = data.len().min(mmap.len());

                mmap[..len].copy_from_slice(&data[..len]);
                mmap.flush().expect("Failed to flush mmap data");

                println!("[MarketData] ✅ Data written to mmap.");
            }

            // ✅ Publish full JSON object to Kafka
            publish_to_kafka(KAFKA_TOPIC, &json_str).await;
        }
    } else {
        eprintln!(
            "[MarketData] ❌ Failed to parse WebSocket message as JSON array: {}",
            text
        );
    }
}
