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
use std::env;

use handler::handle;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <directory_path>");
        return;
    }
    let directory_path = &args[1];
    handle(directory_path).await;
}
