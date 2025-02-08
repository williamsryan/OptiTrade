use crate::config::IbConfig;
use ibapi::contracts::Contract;
use ibapi::market_data::realtime::{BarSize, WhatToShow};
use ibapi::messages::IncomingMessages;
use ibapi::Client;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{sleep, Duration};

/// Connects to IB and fetches real-time market data (async, thread-safe)
pub async fn fetch_ib_market_data(
    config: &IbConfig,
    symbols: Vec<&str>,
    sender: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "[IB] 🔵 Connecting to IB Gateway at {}:{}...",
        config.host, config.port
    );

    let connection_string = format!("{}:{}", config.host, config.port);

    // Correct Client Instantiation
    let client = Client::connect(&connection_string, config.client_id).expect("connection failed");

    // Start the event loop for receiving messages
    task::spawn({
        let mut client = client.clone();
        async move {
            if let Err(err) = client.run().await {
                eprintln!("[IB] ❌ Error in client event loop: {:?}", err);
            }
        }
    });

    println!("[IB] ✅ Connected to Interactive Brokers!");

    // Subscribe to market data for each symbol
    for symbol in symbols {
        let mut client = client.clone();
        let sender = sender.clone();

        task::spawn(async move {
            let contract = Contract::stock(symbol);

            match client
                .realtime_bars(&contract, BarSize::Sec5, WhatToShow::Trades, false)
                .await
            {
                Ok(mut subscription) => {
                    while let Some(bar) = subscription.next().await {
                        let message = format!("[IB] 📊 Symbol: {} | Bar Data: {:?}", symbol, bar);
                        sender
                            .send(message)
                            .await
                            .expect("Failed to send market data");
                    }
                }
                Err(err) => {
                    eprintln!("[IB] ❌ Error subscribing to {}: {:?}", symbol, err);
                }
            }
        });
    }

    // Listen for incoming IB messages (Errors, Order Status, etc.)
    while let Some(message) = client.recv().await {
        match message {
            IncomingMessages::MarketData(data) => {
                println!("[IB] 📈 Market Data: {:?}", data);
            }
            IncomingMessages::OrderStatus(status) => {
                println!("[IB] 📊 Order Status: {:?}", status);
            }
            IncomingMessages::Error(error) => {
                eprintln!("[IB] ❌ Error: {:?}", error);
            }
            _ => {}
        }
    }

    Ok(())
}
