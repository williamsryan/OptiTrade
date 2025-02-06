use backend::market_data_generated::MarketData::{MarketEvent, MarketEventArgs, Quote, QuoteArgs};
use flatbuffers::FlatBufferBuilder;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref PRODUCER: Arc<FutureProducer> = Arc::new(
        ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("message.timeout.ms", "5000") // Ensure messages don’t time out too fast
            .set("queue.buffering.max.ms", "1") // Reduce latency by sending immediately
            .create()
            .expect("Failed to create Kafka producer")
    );
}

pub async fn publish_to_kafka(
    topic: &str,
    symbol: &str,
    bid_price: f64,
    ask_price: f64,
    timestamp: u64,
) {
    let producer = Arc::clone(&PRODUCER);

    let mut builder = FlatBufferBuilder::new();

    let symbol_str = builder.create_string(symbol);

    let quote = Quote::create(
        &mut builder,
        &QuoteArgs {
            symbol: Some(symbol_str),
            bid_price,
            ask_price,
            timestamp,
        },
    );

    let event = MarketEvent::create(
        &mut builder,
        &MarketEventArgs {
            event_type: Some(builder.create_string("quote")),
            quote: Some(quote),
            trade: None,
        },
    );

    builder.finish(event, None);

    let flatbuffer_data = builder.finished_data();

    let record = FutureRecord::to(topic).payload(flatbuffer_data).key(symbol);

    match producer
        .send(record, Timeout::After(Duration::from_secs(3)))
        .await
    {
        Ok(_) => {
            println!("[Kafka] Successfully published event for {}", symbol);
        }
        Err((e, _)) => {
            eprintln!("[Kafka ERROR] Failed to send message: {:?}", e);
        }
    }
}
