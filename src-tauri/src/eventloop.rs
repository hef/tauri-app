use std::str::FromStr;

use libp2p::{
    futures::StreamExt,
    gossipsub::{GossipsubEvent, IdentTopic},
    swarm::SwarmEvent,
    Swarm, identify::{IdentifyEvent}, PeerId, Multiaddr,
};
use tokio::sync::{broadcast, mpsc};

use crate::networkbehavior::{MyBehaviour, MyBehaviourEvent, MyMessage};

pub struct EventLoop {
    pub swarm: Swarm<MyBehaviour>,
    tx: broadcast::Sender<MyMessage>,
    rx: mpsc::Receiver<MyMessage>,
}

impl EventLoop {
    pub fn new(
        swarm: Swarm<MyBehaviour>,
        tx: broadcast::Sender<MyMessage>,
        rx: mpsc::Receiver<MyMessage>,
    ) -> Self {
        Self { swarm, tx, rx }
    }

    pub async fn run(mut self) -> ! {
        let topic = IdentTopic::new("chat");

        let bootaddr = Multiaddr::from_str("/dnsaddr/server.hef.wtf").unwrap();
        self.swarm.behaviour_mut().kademlia.add_address(&PeerId::from_str("12D3KooWKujo2R622ysC9vJXjTP5BRMwkWMFwMjdK3QVdjjQn9JM").unwrap(), bootaddr);

        loop {
            tokio::select! {
                Some(message) = self.rx.recv() => {
                    if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(topic.clone(), message.get_data())
                    {
                        println!("publish error: {:?}", e);
                    }
                },
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(GossipsubEvent::Message{propagation_source: peer_id, message_id ,message}))=>{
                        let s = String::from_utf8_lossy(&message.data);
                        self.tx.send(MyMessage::new(s.to_string())).unwrap();
                        println!("got message: {} with id: {} from peer: {:?}", s, message_id, peer_id);
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Identify(event)) => {
                        println!("identify: {:?}", event);
                        if let IdentifyEvent::Received {
                            peer_id,
                            info,
                        } = event
                        {
                            println!("peer_id: {:?}, info: {:?}", peer_id, info);
                        }
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Kademlia(event)) => {
                        println!("Kademlia event: {:?}", event);
                    },
                    SwarmEvent::NewListenAddr { listener_id, address } => {
                        println!("listener_id: {:?}, address: {:?}", listener_id, address);
                    }
                    _ => {}
                }
            }
        }
    }
}
