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
            Some(LoginAction::Granted(LoginRole::Admin)) => {
                println!("Hello admin");
                break;
            }
            Some(LoginAction::Granted(LoginRole::User)) => {
                println!("Hello user");
                break;
            }
            Some(LoginAction::Denied) | None => {
                if tries > 3 {
                    break;
                }
                println!("Please try again");
                tries += 1;
            }
        }
    }
}
