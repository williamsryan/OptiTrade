use backend::shared::config::{MarketData, TradeSignal};
use backend::shared::data_loader::load_historical_data;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

pub struct BacktestEngine {
    market_data_stream: mpsc::Receiver<MarketData>,
    portfolio: Portfolio,
}

#[derive(Debug)]
pub struct Portfolio {
    cash: f64,
    positions: HashMap<String, f64>, // Symbol -> Quantity
    trade_log: Vec<ExecutedTrade>,
}

#[derive(Debug)]
pub struct ExecutedTrade {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub side: String,
    pub timestamp: String,
}

impl Portfolio {
    pub fn new(starting_cash: f64) -> Self {
        Self {
            cash: starting_cash,
            positions: HashMap::new(),
            trade_log: Vec::new(),
        }
    }

    pub fn execute_trade(&mut self, signal: &TradeSignal, market_data: &MarketData) {
        let cost = market_data.price * signal.qty as f64;

        match signal.side.as_str() {
            "buy" => {
                if self.cash >= cost {
                    *self.positions.entry(signal.symbol.clone()).or_insert(0.0) +=
                        signal.qty as f64;
                    self.cash -= cost;
                    println!(
                        "Bought {} shares of {} at ${}",
                        signal.qty, signal.symbol, market_data.price
                    );
                } else {
                    println!("Insufficient cash to buy {}", signal.symbol);
                }
            }
            "sell" => {
                if let Some(position) = self.positions.get_mut(&signal.symbol) {
                    if *position >= signal.qty as f64 {
                        *position -= signal.qty as f64;
                        self.cash += cost;
                        println!(
                            "Sold {} shares of {} at ${}",
                            signal.qty, signal.symbol, market_data.price
                        );
                    } else {
                        println!("Insufficient shares to sell {}", signal.symbol);
                    }
                }
            }
            _ => println!("Unknown trade signal: {:?}", signal),
        }

        // Log the trade
        self.trade_log.push(ExecutedTrade {
            symbol: signal.symbol.clone(),
            qty: signal.qty as f64,
            price: market_data.price,
            side: signal.side.clone(),
            timestamp: "2024-01-01T00:00:00Z".to_string(), // Replace with real timestamp
        });
    }

    pub fn print_summary(&self) {
        println!("📊 Portfolio Summary:");
        println!("💰 Cash: ${:.2}", self.cash);
        println!("📈 Positions: {:?}", self.positions);

        if self.trade_log.is_empty() {
            println!("📜 No trades were executed.");
        } else {
            println!("📜 Trade Log:");
            for trade in &self.trade_log {
                println!(
                    "📊 {} {} shares of {} at ${:.2} on {}",
                    trade.side.to_uppercase(),
                    trade.qty,
                    trade.symbol,
                    trade.price,
                    trade.timestamp
                );
            }
        }
    }
}

impl BacktestEngine {
    /// Creates a new backtesting engine, fetching historical market data
    pub async fn new(symbol: &str, start_time: &str, end_time: &str, starting_cash: f64) -> Self {
        let (tx, rx) = mpsc::channel(100);

        // Fetch historical market data
        let historical_data = match load_historical_data(symbol, start_time, end_time).await {
            Ok(data) if !data.is_empty() => data,
            _ => {
                eprintln!(
                    "Warning: No historical data found for {} from {} to {}",
                    symbol, start_time, end_time
                );
                vec![]
            }
        };

        // Spawn a task to simulate real-time market data streaming
        tokio::spawn(async move {
            for market_data in historical_data {
                if tx.send(market_data).await.is_err() {
                    println!("Backtest Engine: Receiver dropped, stopping data stream.");
                    break;
                }
                sleep(Duration::from_millis(500)).await;
            }
        });

        Self {
            market_data_stream: rx,
            portfolio: Portfolio::new(starting_cash),
        }
    }

    /// Retrieves the next piece of historical market data, simulating a live stream
    pub async fn get_next_market_data(&mut self) -> Option<MarketData> {
        self.market_data_stream.recv().await
    }

    /// Executes a simulated trade based on trade signals
    pub fn execute_trade(&mut self, trade_signal: &TradeSignal, market_data: &MarketData) {
        self.portfolio.execute_trade(trade_signal, market_data);
    }

    /// Prints a final summary of the portfolio after the backtest
    pub fn print_summary(&self) {
        self.portfolio.print_summary();
    }
}
