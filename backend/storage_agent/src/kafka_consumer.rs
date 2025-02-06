use crate::db_writer::store_market_data;
use backend::market_data_generated::market_data::root_as_market_event;
// use flatbuffers::FlatBufferBuilder;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
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
            let event = root_as_market_event(payload).expect("Invalid FlatBuffer: MarketEvent");

            if let Some(quote) = event.quote() {
                store_market_data(
                    db_client,
                    quote.symbol().unwrap(),
                    quote.bid_price(),
                    quote.ask_price(),
                    None, // No trade data for quotes
                    quote.timestamp(),
                )
                .await;
            }
        }
    }
}
