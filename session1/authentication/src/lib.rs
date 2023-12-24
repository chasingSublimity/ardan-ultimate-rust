use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};

// always faster to handle pointers than to copy the whole string
pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn read_line() -> String {
    let mut input = String::new();
    // throws away the error
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");
    input.trim().to_string() // trim returns str, which is immutable
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize )]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        // self refers to the type we are implementing
        Self {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    ); 
    users.insert(
        "bob".to_string(),
        User::new("bob", "password", LoginRole::User),
    );
    users
}

fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // load the file
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = hash_password(password);
    let users = get_users();

    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    None
}

// cfg(test) means that when the binary is built for production, the tests
// aren't included in the artifact
#[cfg(test)]
// declares a module called `tests`
// modules live inside crates
// the code in this module/scope are referred to
// with `tests::`
mod tests {
    // imports all the code from the parent scope
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello blake!", greet_user("blake"))
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("bob", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("admin", "booyah"), Some(LoginAction::Denied));
        assert_eq!(login("not-admin", "booyah"), None);
    }
}
