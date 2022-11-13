use {app::network::build_swarm, app::network::Client, clap::Parser};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, env = "IDENTITY")]
    identity: String,
    #[arg(short, long, default_value = "/ip4/0.0.0.0/tcp/80", env = "LISTEN")]
    listen: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    let mut server_identity_bytes = base64::decode(&args.identity).unwrap();
    let identity = libp2p::identity::Keypair::Ed25519(
        libp2p::identity::ed25519::Keypair::decode(&mut server_identity_bytes).unwrap(),
    );
    let swarm = build_swarm(identity, args.listen).await;
    let (c, event_loop) = Client::new(swarm);

    println!("peer id: {:?}", c.peer_id);
    event_loop.run().await;
}
