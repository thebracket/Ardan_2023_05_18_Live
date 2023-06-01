use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hello_file))
        .route("/json", get(say_hello_json));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn say_hello_text() -> &'static str {
    "Hello, world!"
}

use axum::response::Html;

async fn say_hello_html() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

async fn say_hello_html_included() -> Html<&'static str> {
    const HTML: &str = include_str!("hello.html");
    Html(HTML)
}

async fn say_hello_file() -> Html<String> {
    let path = std::path::Path::new("src/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

use serde::Serialize;

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_json() -> axum::Json<HelloJson> {
    axum::Json(HelloJson {
        message: "Hello, World!".to_string(),
    })
}