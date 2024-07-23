use authentication::{login, LoginAction, LoginRole, read_line};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();

        match login(&username, &password) {
            // LoginAction::Granted(LoginRole::Admin) => println!("Admin"),
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => println!("Welcome admin"),
                    LoginRole::User => println!("Welcome user"),
                }
                break;
            }
            Some(LoginAction::Denied) => {
                // Do nothing
            }
            None => {
                println!("New user system");
            }
        }

        println!("Incorrent username or password");
        tries += 1;
        if tries >= 3 {
            println!("Too many failed logins");
            break;
        }
    }
}
