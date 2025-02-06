mod db_writer;
mod kafka_consumer;

use db_writer::connect_db;
use kafka_consumer::consume_kafka_messages;
use tokio_postgres::Client;

#[tokio::main]
async fn main() {
    let db_client: Client = connect_db().await.expect("Failed to connect to database");
    consume_kafka_messages(&db_client).await;
}
