use backend::shared::config::TradeSignal;
use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json;
use std::time::Duration;

pub async fn publish_trade_signal(trade_signal: &TradeSignal) -> Result<(), KafkaError> {
    // Use KafkaError directly instead of Box<dyn Error>
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    let payload = serde_json::to_string(trade_signal).unwrap();

    let record = FutureRecord::to("trade_signals")
        .key(&trade_signal.symbol)
        .payload(&payload);

    match producer.send(record, Duration::from_secs(0)).await {
        Ok(_) => {
            println!("Trade signal published: {:?}", trade_signal);
            Ok(())
        }
        Err((e, _)) => {
            // Unpack tuple (KafkaError, OwnedMessage) and only return KafkaError
            eprintln!("Failed to publish trade signal: {:?}", e);
            Err(e)
        }
    }
}
