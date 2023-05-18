use std::{collections::HashMap, path::Path};
use serde::{Serialize, Deserialize};

pub fn hello_world() {
    println!("Hello, World");
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoginRole {
    Admin, User
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoginAction {
    Granted(LoginRole), Denied
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: hash_password(password),
            role
        }
    }
}

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    return users;
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file
        let users_json = std::fs::read_to_string(users_path)
            .expect("Unable to read file");
        let users = serde_json::from_str(&users_json)
            .expect("Unable to deserialize");
        return users;
    } else {
        // Create the file
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).expect("Serialization fail");
        std::fs::write(users_path, users_json).expect("File error");
        return users;
    }
}

#[allow(clippy::needless_return)]
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    let password = hash_password(password);
    match users.get(username) {
        Some(user) => {
            if user.password == password {
                return Some(LoginAction::Granted(user.role));
            } else {
                return Some(LoginAction::Denied);
            }
        }
        None => {
            return None;
        }
    }

    /*match users.iter().find(|user| user.username == username) {
        Some(user) => {
            if user.password == password {
                return Some(LoginAction::Granted(user.role));
            } else {
                return Some(LoginAction::Denied);
            }
        }
        None => {
            return None;
        }
    }*/
}

/*
#[cfg(test)]
mod test {
    use super::*;

#[test]
fn test_enums() {
    assert_eq!(login("admin", "password"), LoginAction::Admin);
    assert_eq!(login("bob", "password"), LoginAction::User);
    assert_eq!(login("admin", "wrong"), LoginAction::Denied);
    assert_eq!(login("wrong", "password"), LoginAction::Denied);
}
}*/