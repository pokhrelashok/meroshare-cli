mod cli;
use cli::Handler;
#[tokio::main]
async fn main() {
    let mut handler = Handler::new();
    handler.handle().await;
}
