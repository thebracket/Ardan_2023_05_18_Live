use sqlx::Row;

use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file and obtain the database URL
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    // Get a database connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // Fetch the messages from the database
    let messages = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
        .fetch_all(&pool)
        .await?;

    // Print the messages
    for message in messages.into_iter() {
        println!("{message:?}");
    }

    println!("--- stream ---");
    use futures::TryStreamExt;
    let mut message_stream =
        sqlx::query_as::<_, Message>("SELECT id, message FROM messages").fetch(&pool);
    while let Some(message) = message_stream.try_next().await? {
        println!("{message:?}");
    }

    Ok(())
}
