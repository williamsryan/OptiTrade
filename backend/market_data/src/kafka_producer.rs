use backend::market_data_generated::MarketData::{MarketEvent, MarketEventArgs, Quote, QuoteArgs};
use flatbuffers::FlatBufferBuilder;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub async fn publish_to_kafka(
    topic: &str,
    symbol: &str,
    bid_price: f64,
    ask_price: f64,
    timestamp: u64,
) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Failed to create Kafka producer");

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

    let record = FutureRecord::to(topic)
        .payload(flatbuffer_data)
        .key("market_event");

    let _ = producer.send(record, Duration::from_secs(5)).await;
}
