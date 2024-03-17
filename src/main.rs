pub mod tiktok;
pub mod discord;

use discord::bot;

#[tokio::main]
async fn main() {
    bot::run().await;
}
