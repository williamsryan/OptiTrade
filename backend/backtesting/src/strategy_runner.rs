use backend::shared::config::TradeSignal;
use backend::shared::config::MarketData; 

pub fn run_strategy(market_data: &MarketData) -> Option<TradeSignal> {
    if market_data.price > market_data.moving_average_50 {
        return Some(TradeSignal {
            symbol: market_data.symbol.clone(),
            qty: 10,
            side: "buy".to_string(),
        });
    }

    if market_data.price < market_data.moving_average_200 {
        return Some(TradeSignal {
            symbol: market_data.symbol.clone(),
            qty: 10,
            side: "sell".to_string(),
        });
    }

    None
}
