use std::{env, os::unix::prelude::OsStrExt};
use app::state::Stuff;
//use app::state::Stuff;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    let identity : libp2p::identity::Keypair;
    let server_identity_env_var = env::var_os("SERVER_IDENTITY");
    if let Some(server_identity) = server_identity_env_var {
        let mut server_identity_copy = server_identity.as_bytes().to_vec();
        identity = libp2p::identity::Keypair::Ed25519(libp2p::identity::ed25519::Keypair::decode(&mut server_identity_copy).unwrap());
    } else {
        identity = libp2p::identity::Keypair::generate_ed25519();
    }

    Stuff::new(identity, 4001).await;
    sleep(Duration::from_secs(u64::MAX)).await;
}
