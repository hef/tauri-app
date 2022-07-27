use libp2p::{NetworkBehaviour, gossipsub::{Gossipsub, GossipsubEvent, MessageAuthenticity}, mdns::{Mdns, MdnsEvent}, swarm::NetworkBehaviourEventProcess, identity::Keypair};
use libp2p::gossipsub;
use serde::{Serialize, Deserialize};
use tokio::sync::broadcast::Sender;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyMessage {
    message_id: String,
    topic: String,
    source: String,
    data: String,
}


#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct MyBehaviour {
    gossipsub: Gossipsub,
    mdns: Mdns,
    //ping: Ping,
    #[behaviour(ignore)]
    on_message: Sender<MyMessage>
}

impl MyBehaviour {
    pub async fn new(local_key: Keypair, on_message: Sender<MyMessage>) -> MyBehaviour {
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
        .build()
        .unwrap();
        MyBehaviour {
            gossipsub: Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config).unwrap(),
            mdns: Mdns::new(Default::default()).await.unwrap(),
            on_message: on_message,
        }
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for MyBehaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message { propagation_source: _, message_id, message } => {
                self.on_message.send(MyMessage{
                    message_id: message_id.to_string(),
                    topic: message.topic.into_string(),
                    source: message.source.unwrap().to_string(),
                    data: String::from(std::str::from_utf8(&message.data).unwrap()),
                }).unwrap();
            },
            GossipsubEvent::Subscribed { peer_id: _, topic: _ } => todo!(),
            GossipsubEvent::Unsubscribed { peer_id: _, topic: _ } => todo!(),
            GossipsubEvent::GossipsubNotSupported { peer_id: _ } => todo!(),
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        println!("MdnsEvent: {:?}", event);
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, multiaddr) in list {
                    println!("Discovered: {:?} {:?}", peer_id, multiaddr);
                    self.gossipsub.add_explicit_peer(&peer_id);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, multiaddr) in list {
                    println!("Expired: {:?} {:?}", peer_id, multiaddr);
                    self.gossipsub.remove_explicit_peer(&peer_id);
                }
            }
        }
    }
}