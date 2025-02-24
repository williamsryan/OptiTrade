use backend::shared::config::TradeSignal;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json;
use std::time::Duration;

pub async fn publish_trade_signal(
    trade_signal: &TradeSignal,
) -> Result<(), Box<dyn std::error::Error>> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    let payload = serde_json::to_string(trade_signal)?;

    let record = FutureRecord::to("trade_signals")
        .key(&trade_signal.symbol)
        .payload(&payload);

    producer.send(record, Duration::from_secs(0)).await?;

    println!("Trade signal published: {:?}", trade_signal);
    Ok(())
}
