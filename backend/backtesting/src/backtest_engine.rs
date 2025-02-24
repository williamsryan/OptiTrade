use backend::shared::config::MarketData;
use backend::shared::data_loader::load_historical_data;
use tokio::sync::mpsc;

pub struct BacktestEngine {
    market_data_stream: mpsc::Receiver<MarketData>,
}

impl BacktestEngine {
    pub async fn new(symbol: &str, start_time: &str, end_time: &str) -> Self {
        let (tx, rx) = mpsc::channel(100);

        let historical_data = load_historical_data(symbol, start_time, end_time)
            .await
            .unwrap_or_else(|_| vec![]);

        tokio::spawn(async move {
            for market_data in historical_data {
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
