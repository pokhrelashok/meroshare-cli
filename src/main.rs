mod meroshare;
mod request;

use meroshare::init;
#[tokio::main]
async fn main() {
    init().await;
}
