use std::time::Duration;
use tokio::task::spawn_blocking;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    //std::thread::sleep(Duration::from_millis(time));
    //tokio::time::sleep(Duration::from_millis(time)).await;
    let result = spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(time));
    })
    .await;
    println!("Task {task} result {result:?}");
    println!("Task {task} is done.")
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for i in 0..5 {
        futures.push(hello_delay(i, i * 500));
    }
    futures::future::join_all(futures).await;
}

async fn double(n: i32) -> i32 {
    n * 2
}

#[cfg(test)]
mod test {
    use super::double;

    #[test]
    fn simple() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        assert_eq!(rt.block_on(double(2)), 4);
    }

    #[tokio::test]
    async fn the_easy_way() {
        assert_eq!(double(2).await, 4);
    }
}
