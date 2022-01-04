extern crate test_flutter_rust_bridge;

use std::time::Duration;

use test_flutter_rust_bridge::play;
use test_flutter_rust_bridge::Screen;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let _ = play(Screen::Splash);
    sleep(Duration::from_secs(5)).await;
}