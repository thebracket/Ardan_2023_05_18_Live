fn hello_thread(n: u32) {
    println!("Hello from thread number {n}");
}

fn do_math(i: i32) -> i32 {
    let mut n = i + 1;
    for _ in 0..10 {
        n *= 2;
    }
    return n;
}

fn main() {
    println!("Hello from the main thread");

    let mut thread_handles = Vec::new();
    for i in 0 .. 10 {
        let handle = std::thread::spawn(move || do_math(i));
        thread_handles.push(handle);
    }
    
    for handle in thread_handles {
        println!("Thread returned: {}", handle.join().unwrap());
    }
}
