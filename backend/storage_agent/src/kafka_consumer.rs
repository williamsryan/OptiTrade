use crate::db_writer::store_market_data;
use chrono::DateTime;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use serde_json::Value;
use tokio_postgres::Client;

const KAFKA_TOPIC: &str = "market_data";
const KAFKA_BROKER: &str = "localhost:9093";

pub async fn consume_kafka_messages(db_client: &Client) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "storage_agent")
        .set("bootstrap.servers", KAFKA_BROKER)
        .set("enable.auto.commit", "true")
        .create()
        .expect("Failed to create Kafka consumer");

    consumer
        .subscribe(&[KAFKA_TOPIC])
        .expect("Failed to subscribe to topic");

    while let Ok(message) = consumer.recv().await {
        if let Some(payload) = message.payload() {
            let json_str = String::from_utf8_lossy(payload);
            if let Ok(parsed) = serde_json::from_str::<Value>(&json_str) {
                if let (Some(symbol), Some(bid_price), Some(ask_price), Some(timestamp_str)) = (
                    parsed["S"].as_str(),
                    parsed["bp"].as_f64(),
                    parsed["ap"].as_f64(),
                    parsed["t"].as_str(),
                ) {
                    // ✅ Convert timestamp string to Unix timestamp (seconds)
                    let timestamp: u64 = DateTime::parse_from_rfc3339(timestamp_str)
                        .map(|dt| dt.timestamp() as u64) // Convert `i64` → `u64`
                        .unwrap_or_else(|_| {
                            eprintln!(
                                "[Kafka] ⚠️ Failed to parse timestamp: {}, using default 0",
                                timestamp_str
                            );
                            0
                        });

                    let last_price = parsed["last"].as_f64().unwrap_or(0.0);

                    println!(
                        "[Kafka] ✅ Received market data: Symbol: {}, Bid: {}, Ask: {}, Last: {:?}, Timestamp: {}",
                        symbol, bid_price, ask_price, last_price, timestamp
                    );

                    // ✅ Pass correctly converted timestamp
                    store_market_data(
                        &db_client,
                        symbol,
                        bid_price,
                        ask_price,
                        Some(last_price),
                        timestamp,
                    )
                    .await;
                } else {
                    eprintln!(
                        "[Kafka] ❌ Failed to parse required fields from message: {}",
                        json_str
                    );
                }
            } else {
                eprintln!("[Kafka] ❌ Invalid JSON format: {}", json_str);
            }
        }
    }
}
