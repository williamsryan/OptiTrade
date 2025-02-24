use crate::kafka_consumer::TradeSignal;

pub fn validate_trade(trade_signal: &TradeSignal) -> bool {
    // Example rule: Prevent trades over 1000 shares
    if trade_signal.qty > 1000 {
        println!("Trade rejected: Order size too large");
        return false;
    }

    // Example rule: Prevent trading penny stocks
    if trade_signal.symbol.starts_with("OTC") {
        println!("Trade rejected: Penny stock detected");
        return false;
    }

    true
}
