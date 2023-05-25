use std::sync::Mutex;

static MY_SHARED : Mutex<u32> = Mutex::new(0);

fn main() {
    //let lock = MY_SHARED.lock().unwrap();
    //let lock = MY_SHARED.lock().unwrap();    

    /*if let Ok(lock) = MY_SHARED.try_lock() {
        println!("I got the lock");

        if let Ok(lock) = MY_SHARED.try_lock() {
            println!("I got the lock");
        } else {
            println!("I couldn't get the lock");
        }
    } else {
        println!("I couldn't get the lock");
    }*/

    /*let lock = MY_SHARED.lock().unwrap();
    std::mem::drop(lock);
    let lock = MY_SHARED.lock().unwrap();*/

    {
        let lock = MY_SHARED.lock().unwrap();
    }
    let lock = MY_SHARED.lock().unwrap();
}
