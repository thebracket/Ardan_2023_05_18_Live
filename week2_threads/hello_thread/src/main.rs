fn hello_thread(n: u32) {
    println!("Hello from thread number {n}");
}

fn main() {
    println!("Hello from the main thread");

    let mut thread_handles = Vec::new();
    for i in 0 .. 5 {
        let handle = std::thread::spawn(move || hello_thread(i));
        thread_handles.push(handle);
    }
    thread_handles.into_iter().for_each(|h| h.join().unwrap());
}
