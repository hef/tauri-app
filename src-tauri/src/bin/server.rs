use std::{env, os::unix::prelude::OsStrExt};
use app::state::Stuff;
//use app::state::Stuff;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    let mut identity : libp2p::identity::Keypair;
    let x = env::var_os("SERVER_IDENTITY");
    if let Some(y) = x {
        let mut z = y.as_bytes().to_vec();
        identity = libp2p::identity::Keypair::Ed25519(libp2p::identity::ed25519::Keypair::decode(&mut z).unwrap());

    } else {
        identity = libp2p::identity::Keypair::generate_ed25519();
    }

    Stuff::new(identity, 4001).await;
    sleep(Duration::from_secs(u64::MAX)).await;
}
