use clap::{Parser, Subcommand};
use auth::*;

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
    }
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Login Action");
    println!("{:-<40}", "");

    let users = get_users();
    users
        .iter()
        .for_each(|(_username, user)| {
            println!("{:<20}{:<20?}", user.username, user.role);
        });
}

fn add_user(username: String, password: String, admin: Option<bool>) {
    let mut users = get_users();
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

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin);
        }
        None => {
            println!("Run with --help for info");
        }
    }
}
