use {
    app::client::Client,
    std::env,
    tokio::time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    let identity: libp2p::identity::Keypair;
    let server_identity_env_var = env::var("SERVER_IDENTITY");
    if let Ok(server_identity) = server_identity_env_var {
        let server_identtiy_bytes = base64::decode(server_identity).unwrap();

        let mut server_identity_copy = server_identtiy_bytes.clone();
        identity = libp2p::identity::Keypair::Ed25519(
            libp2p::identity::ed25519::Keypair::decode(&mut server_identity_copy).unwrap(),
        );
    } else {
        identity = libp2p::identity::Keypair::generate_ed25519();
    }

    let (c, event_loop) = Client::new(identity, 4001).await;
    tokio::spawn(event_loop.run());
    println!("peer id: {:?}", c.peer_id);
    sleep(Duration::from_secs(u64::MAX)).await;
}
