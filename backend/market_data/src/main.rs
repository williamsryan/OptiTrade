use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

const ALPACA_WS_URL: &str = "wss://stream.data.alpaca.markets/v2/iex";
const API_KEY: &str = "PKS83YQBEUPZL111E7NJ";
const SECRET_KEY: &str = "NDSora4h27DyMzn1vgRElYWr40gkDpkZTrzIXwvh";

#[tokio::main]
async fn main() {
    let url = Url::parse(ALPACA_WS_URL).expect("Invalid WebSocket URL");
    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to Alpaca WebSocket");

    let (mut write, mut read) = ws_stream.split();

    // Authenticate
    let auth_msg = format!(
        r#"{{"action": "auth", "key": "{}", "secret": "{}"}}"#,
        API_KEY, SECRET_KEY
    );

    write
        .send(Message::Text(auth_msg))
        .await
        .expect("Failed to send auth");

    // Subscribe to data
    let subscribe_msg =
        r#"{"action":"subscribe","trades":["AAPL","TSLA"],"quotes":["AAPL","TSLA"]}"#;
    write
        .send(Message::Text(subscribe_msg.into()))
        .await
        .expect("Failed to subscribe");

    println!("[MarketData] Subscribed to Alpaca market feed.");

    // Process incoming messages
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Send to processing pipeline
                process_market_data(&text);
            }
            Err(e) => eprintln!("[MarketData] WebSocket Error: {:?}", e),
            _ => {}
        }
    }
}

/// Parses and processes market data messages with zero-copy handling.
fn process_market_data(text: &str) {
    if let Ok(json_msg) = serde_json::from_str::<serde_json::Value>(text) {
        println!("[MarketData] New Event: {:?}", json_msg);
        // TODO: Publish to Kafka or in-memory buffer
    }
}
