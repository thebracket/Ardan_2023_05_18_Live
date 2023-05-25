fn hello_thread() {
    println!("Hello from thread!");
}

fn main() {
    println!("Hello from the main thread");

    let thread_handle = std::thread::spawn(hello_thread);
    thread_handle.join().unwrap();
}
