use super::networkbehavior::MyBehaviour;
use libp2p::{identity::Keypair, swarm::SwarmBuilder, PeerId, Swarm};

pub async fn build_swarm(identity: Keypair, listen_on: String) -> Swarm<MyBehaviour> {
    let local_key = identity;
    let peer_id = PeerId::from(local_key.public());
    let transport = libp2p::tokio_development_transport(local_key.clone()).unwrap();
    let mut swarm = {
        SwarmBuilder::new(transport, MyBehaviour::new(local_key).await, peer_id)
            .executor(Box::new(|fut| {
                tauri::async_runtime::spawn(fut);
            }))
            .build()
    };

    swarm.listen_on(listen_on.parse().unwrap()).unwrap();
    swarm
}
