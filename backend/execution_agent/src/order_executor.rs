use backend::shared::config::load_config;
use lazy_static::lazy_static;
use reqwest::Client;
use serde_json::json; // Ensures config is loaded only once

// Load config once and reuse it
lazy_static! {
    static ref CONFIG: backend::shared::config::Config = load_config();
}

pub async fn place_order(
    symbol: &str,
    qty: i32,
    side: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let alpaca_config = &CONFIG.alpaca; // Use preloaded config

    let client = Client::new();
    let url = format!("{}/orders", alpaca_config.base_url);

    let order = json!({
        "symbol": symbol,
        "qty": qty,
        "side": side,
        "type": "market",
        "time_in_force": "gtc"
    });

    let response = client
        .post(&url)
        .header("APCA-API-KEY-ID", &alpaca_config.api_key)
        .header("APCA-API-SECRET-KEY", &alpaca_config.api_secret)
        .json(&order)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Order placed successfully: {:?}", response.text().await?);
    } else {
        eprintln!("Order failed: {:?}", response.text().await?);
    }

    Ok(())
}

pub async fn cancel_order(order_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let alpaca_config = &CONFIG.alpaca; // Use preloaded config

    let client = Client::new();
    let url = format!("{}/orders/{}", alpaca_config.base_url, order_id);

    let response = client
        .delete(&url)
        .header("APCA-API-KEY-ID", &alpaca_config.api_key)
        .header("APCA-API-SECRET-KEY", &alpaca_config.api_secret)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Order {} canceled successfully.", order_id);
    } else {
        eprintln!("Failed to cancel order: {:?}", response.text().await?);
    }

    Ok(())
}
