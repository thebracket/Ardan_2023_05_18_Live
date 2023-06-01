use std::path::Path;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("mytile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

use serde::Deserialize;

#[derive(Deserialize)]
struct User { name: String, password: String }

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_users() -> GenericResult<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}

fn main() {
    let users = load_users();
    match users {
        Ok(users) => {
            for user in users {
                println!("User: {}, {}", user.name, user.password);
            }
        },
        Err(err) => {
            println!("Error: {err}");
        }
    }
}