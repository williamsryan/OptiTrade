mod backtest_engine;
mod kafka_producer;
mod strategy_runner;

use backtest_engine::BacktestEngine;
use kafka_producer::publish_trade_signal;
use strategy_runner::run_strategy;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("🔵 Backtesting Agent Started");

    // Define backtest parameters
    let symbol = "AAPL"; // Set your test stock
    let start_time = "2024-01-01T00:00:00Z";
    let end_time = "2024-02-01T00:00:00Z";
    let starting_cash = 10_000.0; // Initial portfolio balance

    // Initialize backtest engine with symbol, time range, and starting cash
    let mut engine = BacktestEngine::new(symbol, start_time, end_time, starting_cash).await;

    println!(
        "⏳ Running backtest from {} to {} on {}",
        start_time, end_time, symbol
    );

    // Run strategy loop
    loop {
        // Get market data for backtest (from TimescaleDB or Alpaca)
        if let Some(market_data) = engine.get_next_market_data().await {
            println!("📊 Processing market data: {:?}", market_data);

            // Run strategy logic on this market data
            if let Some(trade_signal) = run_strategy(&market_data) {
                println!("📈 Generated Trade Signal: {:?}", trade_signal);

                // Execute the simulated trade within the backtest engine
                engine.execute_trade(&trade_signal, &market_data);

                // Publish trade signal to Kafka topic (for analysis or logging)
                if let Err(e) = publish_trade_signal(&trade_signal).await {
                    eprintln!("❌ Failed to publish trade signal: {:?}", e);
                } else {
                    println!("✅ Trade Signal Published: {:?}", trade_signal);
                }
            }
        } else {
            println!("🏁 No more historical data. Backtest complete.");
            break;
        }

        // Simulate time passing in backtest (adjustable for faster/slower backtesting)
        sleep(Duration::from_millis(500)).await;
    }

    // Print final backtest portfolio summary
    engine.print_summary();
}
