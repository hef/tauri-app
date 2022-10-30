use libp2p::kad::store::MemoryStore;
use libp2p::kad::{Kademlia, KademliaEvent};
use libp2p::relay::v2::client;
use libp2p::swarm::behaviour::toggle::Toggle;
use libp2p::{
    dcutr,
    gossipsub::{Gossipsub, GossipsubEvent, MessageAuthenticity},
    identify,
    identity::Keypair,
    relay::v2::relay,
    NetworkBehaviour,
};
use libp2p::{gossipsub, ping};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyMessage {
    message_id: String,
    topic: String,
    source: String,
    data: String,
}

impl MyMessage {
    pub fn new(message: String) -> Self {
        Self {
            message_id: String::from("7"),
            topic: String::from("topic"),
            source: String::from("x"),
            data: message,
        }
    }

    pub fn get_data(self) -> String {
        self.data
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
pub struct MyBehaviour {
    pub gossipsub: Gossipsub,
    pub kademlia: Kademlia<MemoryStore>,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
    relay: relay::Relay,
    dcutr: dcutr::behaviour::Behaviour,
    relay_client: Toggle<client::Client>,
}

#[derive(Debug)]
pub enum MyBehaviourEvent {
    Gossipsub(GossipsubEvent),
    Kademlia(KademliaEvent),
    Identify(identify::Event),
    Ping(ping::Event),
    Relay(relay::Event),
    Dcutr(dcutr::behaviour::Event),
    RelayClient(client::Event),
}

impl MyBehaviour {
    pub fn new(local_key: Keypair) -> Self {
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .build()
            .unwrap();
        let store = MemoryStore::new(local_key.clone().public().to_peer_id());
        Self {
            kademlia: Kademlia::new(local_key.public().to_peer_id(), store),
            identify: identify::Behaviour::new(
                identify::Config::new("/app/0.0.0".into(), local_key.public()).with_cache_size(100),
            ),
            ping: ping::Behaviour::new(ping::Config::new()),
            relay: relay::Relay::new(local_key.public().to_peer_id(), Default::default()),
            dcutr: dcutr::behaviour::Behaviour::new(),
            relay_client: None.into(),
            gossipsub: Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
                .unwrap(),
        }
    }

    pub fn new_with_relay_client(
        local_key: Keypair,
        relay_client: libp2p::relay::v2::client::Client,
    ) -> Self {
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .build()
            .unwrap();
        let store = MemoryStore::new(local_key.clone().public().to_peer_id());
        Self {
            kademlia: Kademlia::new(local_key.public().to_peer_id(), store),
            identify: identify::Behaviour::new(
                identify::Config::new("/app/0.0.0".into(), local_key.public()).with_cache_size(100),
            ),
            ping: ping::Behaviour::new(ping::Config::new()),
            relay: relay::Relay::new(local_key.public().to_peer_id(), Default::default()),
            dcutr: dcutr::behaviour::Behaviour::new(),
            gossipsub: Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
                .unwrap(),
            relay_client: Some(relay_client).into(),
        }
    }
}

impl From<GossipsubEvent> for MyBehaviourEvent {
    fn from(event: GossipsubEvent) -> Self {
        MyBehaviourEvent::Gossipsub(event)
    }
}

impl From<KademliaEvent> for MyBehaviourEvent {
    fn from(event: KademliaEvent) -> Self {
        MyBehaviourEvent::Kademlia(event)
    }
}

impl From<identify::Event> for MyBehaviourEvent {
    fn from(event: identify::Event) -> Self {
        MyBehaviourEvent::Identify(event)
    }
}

impl From<ping::Event> for MyBehaviourEvent {
    fn from(event: ping::Event) -> Self {
        MyBehaviourEvent::Ping(event)
    }
}

impl From<relay::Event> for MyBehaviourEvent {
    fn from(event: relay::Event) -> Self {
        MyBehaviourEvent::Relay(event)
    }
}

impl From<dcutr::behaviour::Event> for MyBehaviourEvent {
    fn from(event: dcutr::behaviour::Event) -> Self {
        MyBehaviourEvent::Dcutr(event)
    }
}

impl From<client::Event> for MyBehaviourEvent {
    fn from(event: client::Event) -> Self {
        MyBehaviourEvent::RelayClient(event)
    }
}
