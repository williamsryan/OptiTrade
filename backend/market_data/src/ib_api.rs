use backend::shared::config::IbConfig;
use ibapi::contracts::Contract;
use ibapi::market_data::realtime::{BarSize, WhatToShow};
use ibapi::client::Client;
use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc;

pub struct IBMarketData {
    config: IbConfig,
}

impl IBMarketData {
    pub fn new(config: IbConfig) -> Arc<Self> {
        println!("[IB] 🔵 Initializing IB Market Data Agent...");
        Arc::new(Self { config })
    }

    /// Fetch real-time stock market data (borrows `self`)
    pub fn stream_market_data(&self, symbols: Vec<String>, sender: mpsc::Sender<String>) {
        let mut handles = vec![];

        for symbol in symbols {
            let config = self.config.clone();
            let sender = sender.clone();

            let handle = thread::spawn(move || {
                let connection_url = format!("{}:{}", config.host, config.port);
                let client = Client::connect(&connection_url, config.client_id as i32)
                    .expect("Connection to IB Gateway/TWS failed!");

                let contract = Contract::stock(&symbol);
                let subscription = client
                    .realtime_bars(&contract, BarSize::Sec5, WhatToShow::Trades, false)
                    .expect("Realtime bars request failed!");

                for bar in subscription {
                    let message = format!(r#"{{"symbol": "{}", "bar": {:?}}}"#, symbol, bar);

                    // ✅ Send data via `mpsc::Sender`
                    if let Err(err) = sender.blocking_send(message) {
                        eprintln!("[IB] ❌ Failed to send market data: {:?}", err);
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.join();
        }
    }

    /// Fetch options chain data (borrows `self`)
    /// TODO: move this to separate 'options' agent?
    pub fn fetch_options_chain(&self, symbol: String, sender: mpsc::Sender<String>) {
        let config = self.config.clone();

        let handle = thread::spawn(move || {
            let connection_url = format!("{}:{}", config.host, config.port);
            let client = Client::connect(&connection_url, config.client_id as i32)
                .expect("Connection to IB Gateway/TWS failed!");

            let contract = Contract::option(&symbol, "202502", 200.0, "C"); 
            let options_chain = client
                .contract_details(&contract)
                .expect("Failed to fetch options chain");

            for option in options_chain {
                let message = format!(r#"{{"symbol": "{}", "option": {:?}}}"#, symbol, option);

                // ✅ Send data via `mpsc::Sender`
                if let Err(err) = sender.blocking_send(message) {
                    eprintln!("[IB] ❌ Failed to send options data: {:?}", err);
                }
            }
        });

        handle.join().unwrap();
    }
}
