use tokio_postgres::{Client, NoTls};

pub async fn connect_db() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=5433 user=optitrade password=secret dbname=market_data",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    // Ensure the table is created before inserting data
    create_tables(&client).await?;

    Ok(client)
}

async fn create_tables(client: &Client) -> Result<(), tokio_postgres::Error> {
    let stmt = "
        CREATE TABLE IF NOT EXISTS market_data (
            id SERIAL PRIMARY KEY,
            symbol TEXT NOT NULL,
            bid_price DOUBLE PRECISION NOT NULL,
            ask_price DOUBLE PRECISION NOT NULL,
            last_price DOUBLE PRECISION,
            timestamp BIGINT NOT NULL
        );
    ";

    client.execute(stmt, &[]).await?;
    println!("[DB] ✅ Ensured table 'market_data' exists.");
    Ok(())
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
        .expect("[DB] ❌ Failed to insert market data");
}
