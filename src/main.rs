#[path = "models/bank.rs"]
mod bank;
#[path = "models/capital.rs"]
mod capital;
#[path = "models/company.rs"]
mod company;
#[path = "utils/currency.rs"]
mod currency;
#[path = "controllers/handler.rs"]
mod handler;
#[path = "models/ipo.rs"]
mod ipo;
#[path = "controllers/meroshare.rs"]
mod meroshare;
#[path = "models/portfolio.rs"]
mod portfolio;
#[path = "utils/request.rs"]
mod request;
#[path = "models/transaction.rs"]
mod transaction;
#[path = "models/user.rs"]
mod user;
use std::{
    env,
    io::{self, Write},
};

use handler::handle;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut directory_path = String::new();
    if let Some(dir_path) = args.get(1) {
        directory_path = dir_path.to_string();
    } else {
        print!("Path to the users JSON file (default users.json)? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().is_empty() {
            directory_path = String::from("users.json");
        } else {
            directory_path = input.trim().to_owned();
        }
    }
    handle(directory_path.as_str()).await;
}
