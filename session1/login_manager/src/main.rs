use authentication::{get_users, LoginRole, User, save_users};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add a user.
    Add {
        /// The user's login name
        username: String,
        /// The user's login password plaintext
        password: String,
        /// Optional - Whether or not the user is an admin
        #[arg(long)]
        admin: Option<bool>
    },
    /// Delete a user
    Delete {
        /// User to delete
        username: String
    },
    /// Change a user's password
    ChangePassword {
        /// Username who's password should change
        username: String,
        /// New password
        new_password: String
    }
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");

    let users = get_users();
    users
        .iter()
        .for_each(|(_, user)| {
            println!("{:<20}{:20?}", user.username, user.role)
        })
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };

    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

fn delete_user(username: String) {
    let mut users = get_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
        println!("Deleted {username}!");
    } else {
        println!("{username} does not exist");
    }
}

fn change_password(username: String, new_password: String) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(&username) {
        user.password = authentication::hash_password(&new_password);
        save_users(users)
    } else {
        print!("{username} does not exist")
    }
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin.unwrap_or(false))
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        Some(Commands::ChangePassword { username, new_password }) => {
            change_password(username, new_password)
        }
        None => {
            println!("Run with --help to see instructions")
        }
    }
}
