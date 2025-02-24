mod backtest_engine;
mod kafka_producer;
mod strategy_runner;

// use backend::shared::config::load_config;
use backtest_engine::BacktestEngine;
use kafka_producer::publish_trade_signal;
use strategy_runner::run_strategy;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Backtesting Agent Started");

    // Load configuration
    // let config = load_config();

    // Define backtest parameters
    let symbol = "AAPL"; // Set your test stock
    let start_time = "2024-01-01T00:00:00Z";
    let end_time = "2024-02-01T00:00:00Z";

    // Initialize backtest engine with symbol and time range
    let mut engine = BacktestEngine::new(symbol, start_time, end_time).await;

    // Run strategy loop
    loop {
        // Get market data for backtest (from TimescaleDB or Alpaca)
        if let Some(market_data) = engine.get_next_market_data().await {
            println!("Processing market data: {:?}", market_data);

            // Run strategy logic on this market data
            if let Some(trade_signal) = run_strategy(&market_data) {
                println!("Generated Trade Signal: {:?}", trade_signal);

                // Publish trade signal to Kafka topic
                if let Err(e) = publish_trade_signal(&trade_signal).await {
                    eprintln!("Failed to publish trade signal: {:?}", e);
                } else {
                    println!("Trade Signal Published: {:?}", trade_signal);
                }
            }
        } else {
            println!("No more historical data. Backtest complete.");
            break;
        }

        // Simulate time passing in backtest (adjustable for faster/slower backtesting)
        sleep(Duration::from_millis(500)).await;
    }
}
