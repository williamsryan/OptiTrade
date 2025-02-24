use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub data_provider: DataProvider,
    pub backtest: BacktestConfig,
    pub alpaca: AlpacaConfig,
    pub ib: IbConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DataProvider {
    pub use_provider: String, // "alpaca" or "ib"
}

#[derive(Debug, Deserialize, Clone)]
pub struct BacktestConfig {
    pub data_source: String, // "db" or "alpaca"
}

#[derive(Debug, Deserialize, Clone)]
pub struct AlpacaConfig {
    pub api_key: String,
    pub api_secret: String,
    pub base_url: String, // Used for REST API calls (trading, historical data)
    pub historic_url: String,
    pub websocket_url: String, // Used for live market data streaming
}

#[derive(Debug, Deserialize, Clone)]
pub struct IbConfig {
    pub host: String,
    pub port: u16,
    pub client_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeSignal {
    pub symbol: String,
    pub qty: i32,
    pub side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketData {
    pub symbol: String,
    pub price: f64,
    pub moving_average_50: f64,
    pub moving_average_200: f64,
}

/// Reads `config.toml` and loads provider configuration
pub fn load_config() -> Config {
    let config_str = fs::read_to_string("backend/config.toml").expect("Failed to read config file");
    toml::from_str(&config_str).expect("Failed to parse config file")
}
