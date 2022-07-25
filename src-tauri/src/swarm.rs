use libp2p::{Swarm, PeerId, identity, swarm::SwarmBuilder};

use crate::networkbehavior::MyBehaviour;

pub async fn build_swarm() -> Swarm<MyBehaviour> {
    let local_key = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(local_key.public());
    let transport = libp2p::tokio_development_transport(local_key.clone()).unwrap();
    let mut swarm = {
        SwarmBuilder::new(transport, MyBehaviour::new(local_key).await, peer_id)
            .executor(Box::new(|fut| {
                tauri::async_runtime::spawn(fut);
            }))
            .build()
    };
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
    swarm
}
