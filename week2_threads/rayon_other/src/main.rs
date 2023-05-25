fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.scope(|scope| {
        scope.spawn_broadcast(|_scope, broadcast_channel| {
            println!("Hello from broadcast thread {}", broadcast_channel.index());
        });
    });

    pool.join(test, test);
}

fn test() {
    println!("Hi!");
}