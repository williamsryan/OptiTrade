// use backend::market_data_generated::market_data::{MarketEvent, MarketEventArgs, Quote, QuoteArgs};
// use flatbuffers::FlatBufferBuilder;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::sync::Arc;
use std::time::Duration;

use lazy_static::lazy_static;

lazy_static! {
    static ref PRODUCER: Arc<FutureProducer> = Arc::new(
        ClientConfig::new()
            .set("bootstrap.servers", "localhost:9093")
            .set("message.timeout.ms", "5000")
            .set("acks", "all")  // Ensure messages are confirmed by the broker
            .set("queue.buffering.max.ms", "1")  // Reduce buffering delay
            .set("batch.num.messages", "1")  // Send messages immediately
            .create()
            .expect("❌ Failed to create Kafka producer")
    );
}

/// Publish a market event to Kafka
pub async fn publish_to_kafka(topic: &str, json_message: &str) {
    let producer = Arc::clone(&PRODUCER);

    // Convert JSON string to byte vector for Kafka
    let json_bytes = json_message.as_bytes().to_vec();

    // Ensure key is a `String`, as `&str` causes type inference issues
    let key = String::from("market_data_key");

    let record = FutureRecord::<String, Vec<u8>>::to(topic)
        .key(&key)
        .payload(&json_bytes);

    match producer.send(record, Duration::from_secs(3)).await {
        Ok(_) => (),
        Err((e, _)) => eprintln!("[Kafka ERROR] ❌ Failed to send JSON message: {:?}", e),
    }

    // producer.flush(Duration::from_secs(1));
}

// pub async fn publish_to_kafka(
//     topic: &str,
//     symbol: &str,
//     bid_price: f64,
//     ask_price: f64,
//     timestamp: u64,
// ) {
//     let producer = Arc::clone(&PRODUCER);
//     let mut builder = FlatBufferBuilder::new();

//     let symbol_str = builder.create_string(symbol);
//     let event_type_str = builder.create_string("quote");

//     let quote = Quote::create(
//         &mut builder,
//         &QuoteArgs {
//             symbol: Some(symbol_str),
//             bid_price,
//             ask_price,
//             timestamp,
//         },
//     );

//     let event = MarketEvent::create(
//         &mut builder,
//         &MarketEventArgs {
//             event_type: Some(event_type_str),
//             quote: Some(quote),
//             trade: None,
//         },
//     );

//     builder.finish(event, None);
//     let flatbuffer_data = builder.finished_data();

//     let record = FutureRecord::to(topic).payload(flatbuffer_data).key(symbol);

//     match producer
//         .send(record, Timeout::After(Duration::from_secs(3)))
//         .await
//     {
//         Ok(_) => println!("[Kafka] ✅ Published event for {}", symbol),
//         Err((e, _)) => eprintln!("[Kafka ERROR] ❌ Failed to send message: {:?}", e),
//     }
// }
