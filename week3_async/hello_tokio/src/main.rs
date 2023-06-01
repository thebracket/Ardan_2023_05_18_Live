async fn hello() {
    println!("Hello!");
}

async fn double(n: i32) -> i32 {
    n * 2
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        // Relinquish control
        tokio::task::yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    tokio::join!(hello(), hello());
    let futures = vec![double(2), double(3), double(4)];
    let result = futures::future::join_all(futures).await;
    println!("{result:?}");

    // Use Tokio JoinSet
    let mut set = tokio::task::JoinSet::new();
    for i in 0..10 {
        set.spawn(double(i));
    }
    while let Some(res) = set.join_next().await {
        println!("{res:?}");
    }

    // Spawning
    tokio::spawn(ticker());
    hello().await;
}