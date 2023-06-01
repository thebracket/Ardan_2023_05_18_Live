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

fn load_users_anyhow() -> anyhow::Result<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}

use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users were found")]
    TooManyUsers,
}

fn work_with_my_error() -> Result<Vec<User>, UsersError> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)
        .map_err(|_| UsersError::NoUsers)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)
        .map_err(|_| UsersError::NoUsers)?;
    if users.is_empty() {
        Err(UsersError::NoUsers)
    } else if users.len() > 10 {
        Err(UsersError::TooManyUsers)
    } else {
        Ok(users)
    }
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