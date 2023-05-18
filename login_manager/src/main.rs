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

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        None => {
            println!("Run with --help for info");
        }
    }
}
