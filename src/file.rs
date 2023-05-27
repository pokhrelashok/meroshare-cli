use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::user::User;

pub fn create_file() {
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("tokens.txt")
        .unwrap();
    file.write_all(b"").unwrap();
}

pub fn delete_file() {
    let file_path = "tokens.txt";
    match fs::remove_file(file_path) {
        Ok(()) => println!("File deleted successfully."),
        Err(err) => println!("Error deleting file: {}", err),
    }
}

fn save_map_to_file(map: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let data = UserTokens {
        user_tokens: map.clone(),
    };
    let json = serde_json::to_string(&data)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tokens.txt")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn read_map_from_file() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut file = File::open("tokens.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: UserTokens = serde_json::from_str(&contents)?;
    Ok(data.user_tokens)
}

pub fn get_user_stored_token(id: &String) -> Option<String> {
    match read_map_from_file() {
        Ok(data) => match data.get(id) {
            Some(token) => Some(token.clone()),
            None => None,
        },
        Err(_) => None,
    }
}

pub fn store_user_token(user: &User, token: &String) {
    let mut tokens: HashMap<String, String> = HashMap::new();
    match read_map_from_file() {
        Ok(stored_tokens) => {
            tokens = stored_tokens;
        }
        Err(_) => {}
    }
    tokens.insert(user.username.clone(), token.clone());
    save_map_to_file(&tokens).unwrap();
}

#[derive(Serialize, Deserialize)]
struct UserTokens {
    user_tokens: HashMap<String, String>,
}
