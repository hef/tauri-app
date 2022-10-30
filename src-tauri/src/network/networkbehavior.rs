use libp2p::kad::store::MemoryStore;
use libp2p::kad::{Kademlia, KademliaEvent};
use libp2p::{gossipsub, ping};
use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent, MessageAuthenticity},
    identify,
    identity::Keypair,
    NetworkBehaviour,
    relay::v2::relay,
};
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
}

#[derive(Debug)]
pub enum MyBehaviourEvent {
    Gossipsub(GossipsubEvent),
    Kademlia(KademliaEvent),
    Identify(identify::Event),
    Ping(ping::Event),
    Relay(relay::Event),
}

impl MyBehaviour {
    pub async fn new(local_key: Keypair) -> Self {
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .build()
            .unwrap();
        let store = MemoryStore::new(local_key.clone().public().to_peer_id());
        Self {
            kademlia: Kademlia::new(local_key.public().to_peer_id(), store),
            identify: identify::Behaviour::new(identify::Config::new("/app/0.0.0".into(), local_key.public())),
            ping: ping::Behaviour::new(ping::Config::new()),
            relay: relay::Relay::new(local_key.public().to_peer_id(), Default::default()),
            gossipsub: Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
                .unwrap(),
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