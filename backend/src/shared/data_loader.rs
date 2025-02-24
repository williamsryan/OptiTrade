use crate::shared::config::MarketData;
use tokio_postgres::{Error, NoTls};

pub async fn load_historical_data(
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
