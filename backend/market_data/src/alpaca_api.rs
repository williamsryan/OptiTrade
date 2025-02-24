use backend::shared::config::AlpacaConfig;
use futures_util::{SinkExt, StreamExt};
use reqwest::Client;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

/// Connects to Alpaca WebSocket and streams live market data
pub async fn stream_alpaca_market_data(
    config: &AlpacaConfig,
    symbol: &str,
    sender: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = &config.websocket_url;
    println!("[Alpaca] 🔗 Connecting to WebSocket: {}", url);

    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to Alpaca WebSocket");
    let (mut write, mut read) = ws_stream.split();

    // Authenticate with Alpaca WebSocket
    let auth_msg = serde_json::json!({
        "action": "auth",
        "key": config.api_key,
        "secret": config.api_secret
    })
    .to_string();
    write
        .send(Message::Text(auth_msg))
        .await
        .expect("Failed to send auth");

    // Subscribe to real-time market data
    let subscribe_msg = serde_json::json!({
        "action": "subscribe",
        "trades": [symbol],
        "quotes": [symbol]
    })
    .to_string();
    write
        .send(Message::Text(subscribe_msg))
        .await
        .expect("Failed to send subscribe");

    println!("[Alpaca] ✅ Subscribed to market data for {}", symbol);

    // Spawn a task to periodically fetch options chain
    let options_config = config.clone();
    let symbol_clone = symbol.to_string();
    let sender_clone = sender.clone();
    tokio::spawn(async move {
        loop {
            match fetch_alpaca_options_chain(&options_config, &symbol_clone).await {
                Ok(options_data) => {
                    let json_str = serde_json::to_string(&options_data).unwrap();
                    sender_clone
                        .send(json_str)
                        .await
                        .expect("Failed to send options data");
                }
                Err(err) => eprintln!("[Alpaca] ❌ Error fetching options data: {}", err),
            }
            sleep(Duration::from_secs(30)).await; // Fetch options data every 30 seconds
        }
    });

    // Read messages from WebSocket
    while let Some(msg) = read.next().await {
        if let Ok(Message::Text(text)) = msg {
            sender.send(text).await.expect("Failed to send market data");
        }
    }

    Ok(())
}

/// Fetches options chain from Alpaca REST API
pub async fn fetch_alpaca_options_chain(
    config: &AlpacaConfig,
    symbol: &str,
) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let options_url = format!("{}/v2/options?symbol={}", config.base_url, symbol);

    let res = client
        .get(&options_url)
        .header("APCA-API-KEY-ID", &config.api_key)
        .header("APCA-API-SECRET-KEY", &config.api_secret)
        .send()
        .await?;

    let options_json: Value = res.json().await?;
    Ok(options_json)
}
