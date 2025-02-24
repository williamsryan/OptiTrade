mod alpaca_api;
mod ib_api;

use alpaca_api::stream_alpaca_market_data;
use backend::shared::config::load_config;
use backend::shared::kafka_producer::publish_to_kafka;
use backend::shared::mmap_buffer::write_to_mmap;
use ib_api::IBMarketData;
use serde_json::Value;
use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc;

const KAFKA_TOPIC: &str = "market_data"; // Kafka topic for publishing data
const SYMBOLS: [&str; 3] = ["AAPL", "TSLA", "NVDA"];

#[tokio::main]
async fn main() {
    let config = load_config();
    println!(
        "[MarketData] ✅ Loaded config. Selected provider: {}",
        config.data_provider.use_provider
    );

    let (tx, mut rx) = mpsc::channel::<String>(100);

    match config.data_provider.use_provider.as_str() {
        "alpaca" => {
            println!("[MarketData] 🟢 Using Alpaca WebSocket for real-time market data");

            for symbol in SYMBOLS {
                let alpaca_config = config.alpaca.clone();
                let sender = tx.clone();
                tokio::spawn(async move {
                    match stream_alpaca_market_data(&alpaca_config, symbol, sender).await {
                        Ok(_) => println!("[Alpaca] ✅ Streaming data for {}", symbol),
                        Err(err) => eprintln!("[Alpaca] ❌ Error: {}", err),
                    }
                });
            }
        }
        "ib" => {
            println!("[MarketData] 🔵 Using Interactive Brokers API for market data streaming");

            let ib_market_data = Arc::new(IBMarketData::new(config.ib.clone()));

            // ✅ Stream Market Data
            let symbols = SYMBOLS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let ib_clone = Arc::clone(&ib_market_data);
            let tx_clone = tx.clone();
            thread::spawn(move || {
                ib_clone.stream_market_data(symbols, tx_clone);
            });

            // ✅ Fetch Options Chain Data
            let options_symbol = "AAPL".to_string();
            let ib_clone = Arc::clone(&ib_market_data);
            let tx_clone = tx.clone();
            thread::spawn(move || {
                ib_clone.fetch_options_chain(options_symbol, tx_clone);
            });
        }
        _ => {
            eprintln!("[MarketData] ❌ Invalid data provider in config");
        }
    }

    // Process incoming WebSocket messages
    while let Some(text) = rx.recv().await {
        process_market_data(&text).await;
    }
}

/// Processes incoming market data and writes it to memory-mapped buffer & Kafka.
async fn process_market_data(text: &str) {
    if let Ok(json_array) = serde_json::from_str::<Vec<Value>>(&text) {
        for json_msg in json_array {
            let json_str = json_msg.to_string();

            // Write to Shared Memory-Mapped Buffer
            write_to_mmap(&json_str);

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
