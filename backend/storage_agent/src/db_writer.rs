use tokio_postgres::{Client, NoTls};

pub async fn connect_db() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=optitrade password=secret dbname=market_data",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn store_market_data(
    client: &Client,
    symbol: &str,
    bid_price: f64,
    ask_price: f64,
    last_price: Option<f64>,
    timestamp: u64,
) {
    let stmt = "
        INSERT INTO market_data (symbol, bid_price, ask_price, last_price, timestamp)
        VALUES ($1, $2, $3, $4, $5)";

    client
        .execute(
            stmt,
            &[
                &symbol,
                &bid_price,
                &ask_price,
                &last_price,
                &(timestamp as i64),
            ],
        )
        .await
        .expect("Failed to insert market data");
}
