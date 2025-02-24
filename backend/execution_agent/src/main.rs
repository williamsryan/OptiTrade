mod kafka_consumer;
mod order_executor;
mod risk_checker;

use kafka_consumer::consume_trade_signals;
use order_executor::{cancel_order, place_order};
use risk_checker::validate_trade;

#[tokio::main]
async fn main() {
    println!("Execution Agent Started - Connected to Alpaca");

    // Start consuming trade signals from Kafka
    let mut trade_stream = consume_trade_signals().await;

    while let Some(trade_signal) = trade_stream.recv().await {
        println!("Received Trade Signal: {:?}", trade_signal);

        // Example: Check if an order should be canceled
        if trade_signal.side == "CANCEL" {
            println!("Cancelling order for: {:?}", trade_signal.symbol);
            match cancel_order(&trade_signal.symbol).await {
                Ok(_) => println!("Order canceled successfully."),
                Err(e) => eprintln!("Order cancellation failed: {:?}", e),
            }
            continue; // Skip placing a new order if it's a cancel signal
        }

        // Validate trade before execution
        if validate_trade(&trade_signal) {
            match place_order(&trade_signal.symbol, trade_signal.qty, &trade_signal.side).await {
                Ok(_) => println!("Trade executed successfully."),
                Err(e) => eprintln!("Trade execution failed: {:?}", e),
            }
        } else {
            println!("Trade rejected by risk checker: {:?}", trade_signal);
        }
    }
}
