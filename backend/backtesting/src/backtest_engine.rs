use backend::shared::config::MarketData;
use tokio::sync::mpsc;

pub struct BacktestEngine {
    market_data_stream: mpsc::Receiver<MarketData>,
}

impl BacktestEngine {
    pub fn new(config: &backend::shared::config::Config) -> Self {
        let (tx, rx) = mpsc::channel(100);
        tokio::spawn(async move {
            // Simulate fetching historical market data
            for market_data in backend::shared::market_data::load_historical_data() {
                if tx.send(market_data).await.is_err() {
                    break;
                }
            }
        });

        Self {
            market_data_stream: rx,
        }
    }

    pub async fn get_next_market_data(&mut self) -> Option<MarketData> {
        self.market_data_stream.recv().await
    }
}
