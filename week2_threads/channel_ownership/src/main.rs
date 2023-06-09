use std::sync::mpsc;

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

#[derive(Debug)]
struct MyData {
    data: String,
    n: u32,
}

fn main() {
    let (tx, rx) = mpsc::channel::<MyData>();

    std::thread::spawn(move || {
        while let Ok(data) = rx.recv() {
            println!("--- IN THE THREAD ---");
            println!("{data:?}")
        }
    });

    let mut n = 0;
    loop {
        println!("Enter a string");
        let input = read_line();
        let data_to_move = MyData {
            data: input,
            n,
        };
        n += 1;

        tx.send(data_to_move).unwrap();
    }
}
