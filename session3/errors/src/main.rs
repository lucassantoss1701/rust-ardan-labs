use std::io::ErrorKind;
use std::path::Path;
use serde::Deserialize;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents =maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

#[derive(Deserialize)]
struct User{
    user: String
}

type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

fn load_users() -> anyhow::Result<Vec<User>>{
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    anyhow::bail!("Oh no!");
    Ok(users)
}


fn main() {
    if let  Ok(content) = file_to_uppercase(){
      println!("Contets: {content}")
    };

    let my_file = Path::new("myfile.txt");
    let content = std::fs::read_to_string(my_file);
    match content {
        Ok(contents) => println!("{contents}"),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("File not found - myfile.txt"),
            _ => println!("Error: {e:#?}")
        },
    };
}
