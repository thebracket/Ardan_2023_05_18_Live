use futures::executor::block_on;
use futures::join;

async fn do_work() {
    join!(say_hello(), second_fn());
    println!("2 x 2 = {}", double(2).await);
    not_async();
}

fn not_async() {
    println!("I'm not async!");
}

async fn double(n: i32) -> i32 {
    n * 2
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
