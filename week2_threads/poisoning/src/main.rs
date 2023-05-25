use std::sync::Mutex;

static DATA: Mutex<u32> = Mutex::new(0);

fn poisoner() {
    let mut lock = DATA.lock().unwrap();
    *lock += 1;
    panic!("And poisoner crashed horribly");
}

fn main() {
    let handle = std::thread::spawn(poisoner);
    println!("Trying to return from the thread:");
    println!("{:?}", handle.join());
    println!("Locking the Mutex after the crash:");
    let lock = DATA.lock();
    println!("{lock:?}");
}