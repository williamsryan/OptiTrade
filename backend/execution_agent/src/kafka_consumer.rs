use backend::shared::config::TradeSignal;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use tokio::sync::mpsc;

pub async fn consume_trade_signals() -> mpsc::Receiver<TradeSignal> {
    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "execution_agent")
            .set("bootstrap.servers", "localhost:9092")
            .set("enable.auto.commit", "true")
            .create()
            .expect("Failed to create Kafka consumer");

        let topics = ["trade_signals"];
        consumer
            .subscribe(&topics)
            .expect("Failed to subscribe to topics");

        while let Ok(message) = consumer.recv().await {
            if let Some(payload) = message.payload() {
                if let Ok(trade_signal) = serde_json::from_slice::<TradeSignal>(payload) {
                    if let Err(_) = tx.send(trade_signal).await {
                        println!("Receiver dropped, stopping Kafka consumer...");
                        break;
                    }
                }
            }
        }
    });

    rx
}
