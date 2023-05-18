use auth::*;

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let mut tries = 0;
    loop {
        println!("Username:");
        let username = read_line();
        println!("Password:");
        let password = read_line();

        match login(&username, &password) {
            LoginAction::Admin => {
                println!("Hello admin");
                break;
            }
            LoginAction::User => {
                println!("Hello user");
                break;
            }
            LoginAction::Denied => {
                if tries > 3 {
                    break;
                }
                println!("Please try again");
                tries += 1;
            }
        }
    }
}
