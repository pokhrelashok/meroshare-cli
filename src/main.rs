mod bank;
mod company;
mod file;
mod handler;
mod ipo;
mod meroshare;
mod request;
mod user;
use handler::handle;
#[tokio::main]
async fn main() {
    // init().await;
    handle().await;
}
