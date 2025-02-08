mod alpaca_api;
mod config;
mod ib_api;

use alpaca_api::stream_alpaca_market_data;
use backend::shared::kafka_producer::publish_to_kafka;
use backend::shared::mmap_buffer::write_to_mmap;
use config::load_config;
// use ib_api::fetch_ib_market_data;
use serde_json::Value;
// use std::sync::Arc;
// use std::thread;
use tokio::sync::mpsc;

const KAFKA_TOPIC: &str = "market_data"; // Kafka topic for publishing data
const SYMBOLS: [&str; 3] = ["AAPL", "TSLA", "NVDA"]; // Modify for dynamic symbols

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
        // "ib" => {
        //     println!("[MarketData] 🔵 Using Interactive Brokers API for market data streaming");

        //     let ib_config = Arc::new(config.ib.clone());
        //     let mut handles = vec![];

        //     for symbol in SYMBOLS {
        //         let ib_config = Arc::clone(&ib_config);
        //         let handle =
        //             thread::spawn(move || match fetch_ib_market_data(&ib_config, symbol) {
        //                 Ok(_) => println!("[IB] ✅ Streaming market data for {}", symbol),
        //                 Err(err) => eprintln!("[IB] ❌ Error fetching IB market data: {}", err),
        //             });
        //         handles.push(handle);
        //     }

        //     for handle in handles {
        //         handle.join().unwrap();
        //     }
        // }
        _ => {
            eprintln!("[MarketData] ❌ Invalid data provider in config");
        }
    }

    // ✅ Process incoming WebSocket messages
    while let Some(text) = rx.recv().await {
        process_market_data(&text).await;
    }
}

/// Processes incoming market data and writes it to memory-mapped buffer & Kafka.
async fn process_market_data(text: &str) {
    if let Ok(json_array) = serde_json::from_str::<Vec<Value>>(&text) {
        for json_msg in json_array {
            let json_str = json_msg.to_string();

            // ✅ Write to Shared Memory-Mapped Buffer
            write_to_mmap(&json_str);

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
