use futures::executor::block_on;
use futures::join;

async fn do_work() {
    join!(say_hello(), second_fn());
}

async fn say_hello() {
    println!("Hello, world!");
    second_fn().await;
}

async fn second_fn() {
    println!("Second function");
}

fn main() {
    block_on(do_work());
}
