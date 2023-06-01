async fn hello() {
    println!("Hello!");
}

#[tokio::main]
async fn main() {
    tokio::join!(hello(), hello());
}