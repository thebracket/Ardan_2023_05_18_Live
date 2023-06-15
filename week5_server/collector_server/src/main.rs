mod collector;
mod api;
use std::net::SocketAddr;
use axum::{Router, routing::get, Extension};
use axum::response::Html;

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
        .route("/", get(index))
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

pub async fn index() -> Html<String> {
    let path = std::path::Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}
