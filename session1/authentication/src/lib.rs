pub fn greet_user(name: &str) -> String{
    format!("Hello {name}")
}

pub fn read_line() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction{
    Granted(LoginRole),
    Denied
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginRole{
    Admin,
    User,
}

pub struct Usr {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl Usr{
    pub fn new(username: &str, password: &str, role: LoginRole) -> Usr{
        Self{
            username: username.to_lowercase(),
            password: password.to_string(),
            role
        }
    }
}

pub fn get_users() -> [Usr; 2]{
    [
        Usr::new("admin", "password", LoginRole::Admin),
        Usr::new("bob", "password", LoginRole::User),
    ]
}

pub fn login(username: &str, password: &str) -> Option<LoginAction>{

    let username = username.to_lowercase();
    let users = get_users();

    if let Some(user) = users.iter().find(|user: &&Usr| user.username == username ){
        return if user.password == password {
            Some(LoginAction::Granted(user.role.clone()))
        } else {
            Some(LoginAction::Denied)
        }
    };
    None
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
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(Admin)));
        assert_eq!(login("bob", "password"), Some(LoginAction::Granted(User)));
        assert_eq!(login("wrong", "password"), Some(LoginAction::Denied));
    }
}
