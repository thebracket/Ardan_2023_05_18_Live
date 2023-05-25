fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREADS);

    let sum = std::thread::scope(|my_scope| {
        let mut thread_handles = Vec::new();
        for chunk in chunks {
            let handle = my_scope.spawn(move || {
                let mut sum = 0;
                for i in chunk {
                    sum += i;
                }
                sum
            });
            thread_handles.push(handle);
        }

        thread_handles.into_iter().map(|handle| handle.join().unwrap()).sum::<u32>()
    });
    println!("Sum is {sum}");
}
