use app::state::Stuff;
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    Stuff::new(4001).await;
    sleep(Duration::from_secs(u64::MAX)).await;
}
