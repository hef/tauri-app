use crate::networkbehavior::MyBehaviour;
use libp2p::{identity::Keypair, swarm::SwarmBuilder, PeerId, Swarm};

pub async fn build_swarm(identity: Keypair, port: u32) -> Swarm<MyBehaviour> {
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

    let listenon = format!("/ip4/0.0.0.0/tcp/{port}");
    swarm.listen_on(listenon.parse().unwrap()).unwrap();
    swarm
}
