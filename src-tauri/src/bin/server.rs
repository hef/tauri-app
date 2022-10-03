use std::{env, os::unix::prelude::OsStrExt};
use app::state::Stuff;
//use app::state::Stuff;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    let identity : libp2p::identity::ed25519::Keypair;
    let x = env::var_os("SERVER_IDENTITY");
    if let Some(y) = x {
        identity = libp2p::identity::ed25519::Keypair::decode(&mut y.as_bytes()).unwrap();
    } else {
        identity = libp2p::identity::ed25519::Keypair::generate()
    }

    Stuff::new(identity, 4001).await;
    sleep(Duration::from_secs(u64::MAX)).await;
}
