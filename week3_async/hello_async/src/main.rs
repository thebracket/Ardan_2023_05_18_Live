use futures::executor::block_on;

async fn say_hello() {
    println!("Hello, world!");
    second_fn().await;
}

async fn second_fn() {
    println!("Second function");
}

fn main() {
    block_on(say_hello());
}
