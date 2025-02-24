mod backtest_engine;
mod kafka_producer;
mod strategy_runner;

use backend::shared::config::load_config;
use backtest_engine::BacktestEngine;
use kafka_producer::publish_trade_signal;
use strategy_runner::run_strategy;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Backtesting Agent Started");

    // Load config
    let config = load_config();

    // Initialize backtest engine
    let mut engine = BacktestEngine::new(&config);

    // Run strategy loop
    loop {
        // Get market data for backtest (from local database or Alpaca historical API)
        if let Some(market_data) = engine.get_next_market_data().await {
            // Run strategy logic
            if let Some(trade_signal) = run_strategy(&market_data) {
                // Publish trade signal to Kafka topic
                if let Err(e) = publish_trade_signal(&trade_signal).await {
                    eprintln!("Failed to publish trade signal: {:?}", e);
                } else {
                    println!("Trade Signal Published: {:?}", trade_signal);
                }
            }
        }

        // Simulate time passing in backtest
        sleep(Duration::from_millis(500)).await;
    }
}
