async fn hello() {
    println!("Hello!");
}

#[tokio::main]
async fn main() {
    hello().await;
}