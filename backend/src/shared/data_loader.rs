use crate::shared::config::{load_config, MarketData};
use reqwest::Client;
use serde_json::Value;
use tokio_postgres::{Error, NoTls};

/// Load historical market data from TimescaleDB (original method)
pub async fn load_historical_data_db(
    symbol: &str,
    start_time: &str,
    end_time: &str,
) -> Result<Vec<MarketData>, Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=your_user dbname=your_db", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    let rows = client
        .query(
            "SELECT symbol, price, moving_average_50, moving_average_200 
        FROM historical_prices 
        WHERE symbol = $1 AND timestamp BETWEEN $2 AND $3 
        ORDER BY timestamp ASC",
            &[&symbol, &start_time, &end_time],
        )
        .await?;

    let mut data = Vec::new();
    for row in rows {
        data.push(MarketData {
            symbol: row.get(0),
            price: row.get(1),
            moving_average_50: row.get(2),
            moving_average_200: row.get(3),
        });
    }

    Ok(data)
}

/// Load historical market data from Alpaca API (updated to use config)
pub async fn load_historical_data_alpaca(
    symbol: &str,
    start_time: &str,
    end_time: &str,
) -> Result<Vec<MarketData>, Box<dyn std::error::Error>> {
    let config = load_config();
    let client = Client::new();
    let url = format!(
        "{}/v2/stocks/{}/bars?start={}&end={}&timeframe=1Day",
        config.alpaca.historic_url, symbol, start_time, end_time
    );

    println!("🔍 Fetching Alpaca data from: {}", url); // Debug print

    let response = client
        .get(&url)
        .header("APCA-API-KEY-ID", &config.alpaca.api_key)
        .header("APCA-API-SECRET-KEY", &config.alpaca.api_secret)
        .send()
        .await?;

    let text_response = response.text().await?;

    let json_response: Value = serde_json::from_str(&text_response)?;

    let mut data = Vec::new();
    if let Some(bars) = json_response.get("bars").and_then(|b| b.as_array()) {
        for bar in bars {
            if let Some(price) = bar.get("c").and_then(|p| p.as_f64()) {
                data.push(MarketData {
                    symbol: symbol.to_string(),
                    price,
                    moving_average_50: bar.get("sma_50").and_then(|m| m.as_f64()).unwrap_or(0.0),
                    moving_average_200: bar.get("sma_200").and_then(|m| m.as_f64()).unwrap_or(0.0),
                });
            }
        }
    }

    println!("📊 Parsed Alpaca Data: {:?}", data); // Debug print parsed market data

    Ok(data)
}
