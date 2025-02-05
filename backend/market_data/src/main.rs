use futures_util::StreamExt;
use market_data::run_market_data_agent;
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;
use tokio;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Failed to create Kafka producer");

    println!("[MarketData] Connected to Kafka broker.");

    run_market_data_agent(producer).await;
}
