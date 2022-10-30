use super::networkbehavior::MyBehaviour;
use libp2p::{identity::{Keypair, self}, swarm::SwarmBuilder, PeerId, Swarm, yamux, mplex, Transport, noise};
use libp2p::dns;
use libp2p::tcp;

pub async fn build_swarm(identity: Keypair, listen_on: String) -> Swarm<MyBehaviour> {
    let local_key = identity;
    let peer_id = PeerId::from(local_key.public());
    let transport = tokio_development_transport(local_key.clone()).unwrap();
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


fn tokio_development_transport(
    keypair: identity::Keypair,
) -> std::io::Result<libp2p::core::transport::Boxed<(PeerId, libp2p::core::muxing::StreamMuxerBox)>> {
    Ok(  dns::TokioDnsConfig::system(tcp::TokioTcpTransport::new(tcp::GenTcpConfig::new().nodelay(true))).unwrap()    
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::NoiseAuthenticated::xx(&keypair).unwrap())
        .multiplex(libp2p::core::upgrade::SelectUpgrade::new(
            yamux::YamuxConfig::default(),
            mplex::MplexConfig::default(),
        ))
        .timeout(std::time::Duration::from_secs(20))
        .boxed()
    )
}