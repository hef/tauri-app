use super::networkbehavior::MyBehaviour;
use libp2p::core::multiaddr::{Multiaddr};
use libp2p::dns;
use libp2p::tcp;
use libp2p::{
    core::transport::OrTransport,
    identity::{self, Keypair},
    mplex, noise,
    relay::v2::client::{self, Client},
    swarm::SwarmBuilder,
    yamux, PeerId, Swarm, Transport,
};

pub async fn build_swarm(identity: Keypair, listen_on: String) -> Swarm<MyBehaviour> {
    let local_key = identity;
    let peer_id = PeerId::from(local_key.public());
    let transport = server_transport(local_key.clone()).unwrap();
    let mut swarm = {
        SwarmBuilder::new(transport, MyBehaviour::new(local_key), peer_id)
            .executor(Box::new(|fut| {
                tauri::async_runtime::spawn(fut);
            }))
            .build()
    };

    swarm.listen_on(listen_on.parse().unwrap()).unwrap();
    swarm
}

fn server_transport(
    keypair: identity::Keypair,
) -> std::io::Result<libp2p::core::transport::Boxed<(PeerId, libp2p::core::muxing::StreamMuxerBox)>>
{
    Ok(dns::TokioDnsConfig::system(tcp::TokioTcpTransport::new(
        tcp::GenTcpConfig::new().nodelay(true),
    ))
    .unwrap()
    .upgrade(libp2p::core::upgrade::Version::V1)
    .authenticate(noise::NoiseAuthenticated::xx(&keypair).unwrap())
    .multiplex(libp2p::core::upgrade::SelectUpgrade::new(
        yamux::YamuxConfig::default(),
        mplex::MplexConfig::default(),
    ))
    .timeout(std::time::Duration::from_secs(20))
    .boxed())
}

pub async fn build_swarm_client(identity: Keypair, listen_on: String) -> Swarm<MyBehaviour> {
    let local_key = identity;
    let peer_id = PeerId::from(local_key.public());
    let (transport, client) = client_transport(local_key.clone()).unwrap();
    let mut swarm = {
        SwarmBuilder::new(
            transport,
            MyBehaviour::new_with_relay_client(local_key, client),
            peer_id,
        )
        .executor(Box::new(|fut| {
            tauri::async_runtime::spawn(fut);
        }))
        .build()
    };

    let l: Multiaddr = listen_on.parse().unwrap();

    swarm.listen_on(l.clone()).unwrap();
    swarm
}

fn client_transport(
    keypair: identity::Keypair,
) -> std::io::Result<(
    libp2p::core::transport::Boxed<(PeerId, libp2p::core::muxing::StreamMuxerBox)>,
    client::Client,
)> {
    let (relay_transport, client) =
        Client::new_transport_and_behaviour(keypair.public().to_peer_id());

    let transport = OrTransport::new(
        relay_transport,
        dns::TokioDnsConfig::system(tcp::TokioTcpTransport::new(
            tcp::GenTcpConfig::new().nodelay(true),
        ))
        .unwrap(),
    )
    .upgrade(libp2p::core::upgrade::Version::V1)
    .authenticate(noise::NoiseAuthenticated::xx(&keypair).unwrap())
    .multiplex(libp2p::core::upgrade::SelectUpgrade::new(
        yamux::YamuxConfig::default(),
        mplex::MplexConfig::default(),
    ))
    .timeout(std::time::Duration::from_secs(20))
    .boxed();
    Ok((transport, client))
}
