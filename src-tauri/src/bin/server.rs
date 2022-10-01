use app::state::Stuff;
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    Stuff::new().await;
    sleep(Duration::from_secs(u64::MAX)).await;
}