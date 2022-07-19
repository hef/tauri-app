use libp2p::{NetworkBehaviour, gossipsub::{Gossipsub, GossipsubEvent, MessageAuthenticity}, mdns::{Mdns, MdnsEvent}, swarm::NetworkBehaviourEventProcess, identity::Keypair};
use libp2p::gossipsub;

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct MyBehaviour {
    gossipsub: Gossipsub,
    mdns: Mdns,
    //ping: Ping,
}

impl MyBehaviour {
    pub async fn new(local_key: Keypair) -> MyBehaviour {
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
        .build()
        .unwrap();
        MyBehaviour {
            gossipsub: Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config).unwrap(),
            mdns: Mdns::new(Default::default()).await.unwrap(),
        }
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for MyBehaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        println!("GossipsubEvent: {:?}", event);
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