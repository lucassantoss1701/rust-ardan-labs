use crate::LoginRole::{Admin, User};

pub fn greet_user(name: &str) -> String{
    format!("Hello {name}")
}

pub fn read_line() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[derive(PartialEq, Debug)]
pub enum LoginAction{
    Granted(LoginRole),
    Denied
}

#[derive(PartialEq, Debug)]
pub enum LoginRole{
    Admin,
    User,
}


pub fn login(username: &str, password: &str) -> Option<LoginAction>{
    let username = username.to_lowercase();

    if username != "admin" && username != "bob"{
        return None;
    }

    if username == "admin" && password == "password"{
        Some(LoginAction::Granted(Admin))
    } else if username == "bob" && password == "password"{
        Some(LoginAction::Granted(User))
    }else{
        Some(LoginAction::Denied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user(){
        assert_eq!("Hello Lucas", greet_user("Lucas"));
    }

    #[test]
    fn test_login(){
        assert_eq!(login("admin", "password"), LoginAction::Granted(Admin));
        assert_eq!(login("bob", "password"), LoginAction::Granted(User));
        assert_eq!(login("wrong", "password"), LoginAction::Denied);
    }
}
