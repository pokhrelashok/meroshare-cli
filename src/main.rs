mod handler;
mod meroshare;
mod request;
use handler::handle;
#[tokio::main]
async fn main() {
    // init().await;
    handle().await;
}
