#[path = "models/bank.rs"]
mod bank;
#[path = "models/company.rs"]
mod company;
mod file;
mod handler;
mod ipo;
mod meroshare;
#[path = "models/portfolio.rs"]
mod portfolio;
mod request;
#[path = "models/transaction.rs"]
mod transaction;
#[path = "models/user.rs"]
mod user;
mod utils;
use handler::handle;
#[tokio::main]
async fn main() {
    // init().await;
    handle(true).await;
}
