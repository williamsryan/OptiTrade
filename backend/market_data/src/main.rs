use futures_util::{SinkExt, StreamExt};
use memmap2::MmapMut;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde_json::Value;
// use std::fs::{File, OpenOptions};
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::time::Duration;
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
    // Set up Kafka Producer
    let producer = Arc::new(setup_kafka_producer());

    // Set up memory-mapped file
    let mmap = Arc::new(Mutex::new(setup_mmap_buffer()));

    let url = Url::parse(ALPACA_WS_URL).expect("Invalid WebSocket URL");
    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to Alpaca WebSocket");

    let (mut write, mut read) = ws_stream.split();

    // Authenticate
    let auth_msg = format!(
        r#"{{"action": "auth", "key": "{}", "secret": "{}"}}"#,
        API_KEY, SECRET_KEY
    );

    write
        .send(Message::Text(auth_msg))
        .await
        .expect("Failed to send auth");

    // Subscribe to data
    let subscribe_msg =
        r#"{"action":"subscribe","trades":["AAPL","TSLA"],"quotes":["AAPL","TSLA"]}"#;
    write
        .send(Message::Text(subscribe_msg.into()))
        .await
        .expect("Failed to subscribe");

    println!("[MarketData] Subscribed to Alpaca market feed.");

    // Process incoming messages
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let text = text.clone(); // Clone to avoid async borrow issues
                let producer = Arc::clone(&producer);
                let mmap = Arc::clone(&mmap);

                // Spawn a task to handle Kafka + memory-mapped writing
                tokio::spawn(async move {
                    process_market_data(&text, producer, mmap).await;
                });
            }
            Err(e) => eprintln!("[MarketData] WebSocket Error: {:?}", e),
            _ => {}
        }
    }
}

/// Sets up Kafka producer for publishing market data.
fn setup_kafka_producer() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Failed to create Kafka producer")
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

/// Processes market data and sends it to Kafka and memory-mapped buffer.
async fn process_market_data(text: &str, producer: Arc<FutureProducer>, mmap: Arc<Mutex<MmapMut>>) {
    // Parse JSON
    if let Ok(json_msg) = serde_json::from_str::<Value>(text) {
        // println!("[MarketData] New Event: {:?}", json_msg);

        // Step 1: Publish to Kafka**
        let key = json_msg["symbol"].as_str().unwrap_or("unknown");
        let record = FutureRecord::to(KAFKA_TOPIC).key(key).payload(text);

        let _ = producer
            .send(record, Duration::from_secs(0))
            .await
            .map_err(|e| eprintln!("[MarketData] Kafka error: {:?}", e));

        // Step 2: Write to Memory-Mapped Buffer**
        if let Ok(mut mmap) = mmap.lock() {
            let data = text.as_bytes();
            let len = data.len().min(MMAP_SIZE); // Ensure it fits in buffer

            mmap[..len].copy_from_slice(&data[..len]);
            mmap.flush().expect("Failed to flush mmap data");

            println!("[MarketData] Data written to memory-mapped buffer.");
        }
    }
}
