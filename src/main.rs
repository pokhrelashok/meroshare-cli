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
use handler::handle;
#[tokio::main]
async fn main() {
    // init().await;
    handle().await;
}
