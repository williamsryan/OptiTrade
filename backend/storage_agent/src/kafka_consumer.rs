use crate::db_writer::store_market_data;
// use backend::shared::market_data_generated::market_data::root_as_market_event;
// use flatbuffers::FlatBufferBuilder;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use serde_json::Value;
use tokio_postgres::Client;

const KAFKA_TOPIC: &str = "market_data";
const KAFKA_BROKER: &str = "localhost:9092";

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
                if let (Some(symbol), Some(bid_price), Some(ask_price), Some(timestamp)) = (
                    parsed["S"].as_str(),
                    parsed["bp"].as_f64(),
                    parsed["ap"].as_f64(),
                    parsed["t"].as_u64(),
                ) {
                    let last_price = parsed["last"].as_f64();

                    println!(
                            "[Kafka] ✅ Received market data: Symbol: {}, Bid: {}, Ask: {}, Last: {:?}, Timestamp: {}",
                            symbol, bid_price, ask_price, last_price, timestamp
                        );

                    // Insert into the database
                    store_market_data(
                        &db_client, symbol, bid_price, ask_price, last_price, timestamp,
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
