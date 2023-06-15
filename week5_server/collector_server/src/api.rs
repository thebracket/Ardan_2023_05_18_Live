use axum::Extension;
use sqlx::FromRow;
use futures::TryStreamExt;

#[derive(FromRow, Debug)]
pub struct DataPoint {
    id: i32,
    collector_id: String,
    received: i64,
    total_memory: i64,
    used_memory: i64,
    average_cpu: f32,
}

pub async fn show_all(Extension(pool): Extension<sqlx::SqlitePool>) {
    let mut rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries")
        .fetch(&pool);

    while let Some(row) = rows.try_next().await.unwrap() {
        println!("{:?}", row);
    }
}