use std::collections::HashMap;

pub fn hello_world() {
    println!("Hello, World");
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LoginRole {
    Admin, User
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LoginAction {
    Granted(LoginRole), Denied
}

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: password.to_string(),
            role
        }
    }
}

pub fn get_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    return users;
    /*vec! [
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User),
    ]*/
}

#[allow(clippy::needless_return)]
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
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