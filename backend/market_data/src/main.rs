use market_data::{connect_db, run_market_data_agent};
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;
use tokio;

#[tokio::main]
async fn main() {
    println!("[MarketData] Starting Market Data Agent...");

    // ✅ Step 1: Connect to the database
    let db_client = connect_db().await.expect("Failed to connect to database");

    // ✅ Step 2: Set up Kafka producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Failed to create Kafka producer");

    // ✅ Step 3: Run the Market Data Agent with database insertion
    tokio::spawn(async move {
        run_market_data_agent(producer, &db_client).await;
    });

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
