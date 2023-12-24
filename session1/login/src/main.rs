use authentication::{login, read_line, LoginAction, LoginRole};
fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(LoginRole::Admin)) => println!("Admin"),
            Some(LoginAction::Granted(LoginRole::User)) => println!("User"),
            Some(LoginAction::Denied) => {
                println!("Incorrect password");
                tries += 1;
            }
            None => {
                println!("New user system")
            }
        }
        if tries >= 3 {
            println!("Too many attempts");
            break
        }
    }
}
