use std::sync::mpsc;

enum Command {
    SayHello, Quit
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Hello!"),
                Command::Quit => {
                    println!("Quitting now");
                    break;
                }
            }
        }
    });

    let sender2 = tx.clone();
    let handle2 = std::thread::spawn(move || {
        for _ in 0..10 {
            sender2.send(Command::SayHello).unwrap();
        }
    });

    for _ in 0..10 {
        tx.send(Command::SayHello).unwrap();
    }
    handle2.join().unwrap();
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
