mod backtest_engine;
mod kafka_producer;
mod strategy_runner;

use backtest_engine::BacktestEngine;
use kafka_producer::publish_trade_signal;
use strategy_runner::run_strategy;

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

    // Get historical data count
    let historical_data_count = match engine.get_historical_data_count().await {
        Some(count) if count > 0 => count,
        _ => {
            println!("⚠️ No historical data available! Exiting...");
            return;
        }
    };

    println!("📊 Total historical data points: {}", historical_data_count);

    let mut total_data_points = 0;

    // Run strategy loop
    while let Some(market_data) = engine.get_next_market_data().await {
        println!("📊 Processing market data: {:?}", market_data);
        total_data_points += 1;

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

        // Stop backtesting when all data has been processed
        if total_data_points >= historical_data_count {
            println!("🏁 All historical data processed. Backtest complete.");
            break;
        }
    }

    // Print final backtest portfolio summary
    engine.print_summary();
}
