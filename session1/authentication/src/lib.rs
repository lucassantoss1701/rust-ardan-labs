use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};
use sha2::Digest;

pub fn hash_password(password: &str) -> String{
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn greet_user(name: &str) -> String{
    format!("Hello {name}")
}

pub fn read_line() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginAction{
    Granted(LoginRole),
    Denied
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole{
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User{
    pub fn new(username: &str, password: &str, role: LoginRole) -> User{
        Self{
            username: username.to_lowercase(),
            password: hash_password(password),
            role
        }
    }
}

// pub fn get_users() -> Vec<User>{
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("bob", "password", LoginRole::User),
//     ]
// }

pub fn get_default_users() -> HashMap<String, User>{
    let mut users =  HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(),  User::new("bob", "password", LoginRole::User));
    users
}

pub fn get_users() -> HashMap<String, User>{
   let users_path = Path::new("users.json");
    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    }else{
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

// fn get_admin_users(){
//     let users: Vec<String> = get_users()
//         .into_iter()
//         .filter(|u: &User| u.role == LoginRole::Admin)
//         .map(|u: User| u.username)
//         .collect();
//
// }
pub fn login(username: &str, password: &str) -> Option<LoginAction>{
    let username = username.to_lowercase();
    let password = hash_password(password);

    let users = get_users();
    if let Some(user) = users.get(&username){
        return if user.password == password {
            Some(LoginAction::Granted(user.role.clone()))
        } else {
            Some(LoginAction::Denied)
        }
    }

    None


    // if let Some(user) = users.iter().find(|user: &&User| user.username == username ){
    //     return if user.password == password {
    //         Some(LoginAction::Granted(user.role.clone()))
    //     } else {
    //         Some(LoginAction::Denied)
    //     }
    // };
}

#[cfg(test)]
mod tests {
    use crate::LoginRole::{Admin,User};
use super::*;

    #[test]
    fn test_greet_user(){
        assert_eq!("Hello Lucas", greet_user("Lucas"));
    }

    #[test]
    fn test_login(){
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(Admin)));
        assert_eq!(login("bob", "password"), Some(LoginAction::Granted(User)));
        assert_eq!(login("wrong", "password"), Some(LoginAction::Denied));
    }
}
