use std::{collections::HashMap, sync::RwLock};
use clap::{Parser, Subcommand};
use auth::*;
use once_cell::sync::Lazy;

static USER_LIST: Lazy<RwLock<HashMap<String, User>>> = Lazy::new(|| RwLock::new(get_users()));

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users
    List,
    /// Add a user
    Add {
        /// Username
        username: String,

        /// Password
        password: String,

        /// Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete a user
    Delete {
        /// Username
        username: String,
    },
    /// Change the user's password
    ChangePassword {
        /// Username whose password you want to change
        username: String,

        /// New password
        new_password: String,
    },
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Login Action");
    println!("{:-<40}", "");

    let users = USER_LIST.read().unwrap();
    users
        .iter()
        .for_each(|(_username, user)| {
            println!("{:<20}{:<20?}", user.username, user.role);
        });
}

fn add_user(username: String, password: String, admin: Option<bool>) {
    let mut users = USER_LIST.write().unwrap();
    if users.contains_key(&username) {
        println!("{username} already exists! Updating record.");
        // return; // <-- Bail out!
    }
    let role = if admin.unwrap_or(false) {
        LoginRole::Admin
    } else {
        LoginRole::User
    };
    let new_user = User::new(&username, &password, role);
    users.insert(username, new_user);
    save_users(&users);
}

fn delete_user(username: String) {
    let mut users = get_users();
    if users.contains_key(&username) {
        let old_user = users.remove(&username);
        println!("Removed: {old_user:?}");
        save_users(&users);
    } else {
        println!("{username} does not exist");
    }
}

fn change_password(username: String, new_password: String) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(&username) {
        user.password = hash_password(&new_password);
        save_users(&users);
    } else {
        println!("{username} does not exist");
    }
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin);
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        Some(Commands::ChangePassword { username, new_password }) => {
            change_password(username, new_password);
        }
        None => {
            println!("Run with --help for info");
        }
    }
}
