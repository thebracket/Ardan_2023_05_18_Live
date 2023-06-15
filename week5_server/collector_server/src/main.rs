mod collector;
mod api;
use std::net::SocketAddr;
use axum::{Router, routing::get, Extension};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file and obtain the database URL
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    // Get a database connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // Start the collector
    let handle = tokio::spawn(collector::data_collector(pool.clone()));

    // Start the web server
    let app = Router::new()
        .route("/", get(test))
        .route("/api/all", get(api::show_all))
        .route("/api/collectors", get(api::show_collectors))
        .route("/api/collector/:uuid", get(api::collector_data))
        .layer(Extension(pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // Wait
    handle.await??;
    
    Ok(())
}

async fn test() -> &'static str {
    "Hello, world!"
}
