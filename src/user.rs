use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "dpId")]
    pub dp: String,
    pub username: String,
    pub password: String,
    pub crn: String,
    pub pin: String,
    pub name: String,
}

pub fn get_users() -> Vec<User> {
    let mut file = File::open("users.json").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let users: Vec<User> = serde_json::from_str(&contents).expect("Failed to parse JSON");
    users
}
