use libp2p::gossipsub;
use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent, MessageAuthenticity},
    identity::Keypair,
    NetworkBehaviour,
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
    //mdns: Mdns,
    //ping: Ping,

    //#[behaviour(ignore)]
    //on_message: Sender<MyMessage>
}

#[derive(Debug)]
pub enum MyBehaviourEvent {
    Gossipsub(GossipsubEvent),
}

impl MyBehaviour {
    pub async fn new(local_key: Keypair) -> MyBehaviour {
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .build()
            .unwrap();
        MyBehaviour {
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
