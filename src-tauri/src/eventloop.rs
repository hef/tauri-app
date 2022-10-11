use libp2p::{
    futures::StreamExt,
    gossipsub::{GossipsubEvent, IdentTopic},
    swarm::SwarmEvent,
    Swarm,
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
        loop {
            tokio::select! {
                Some(message) = self.rx.recv() => {
                    if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(topic.clone(), message.get_data())
                    {
                        println!("publish error: {:?}", e);
                    }
                },
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(GossipsubEvent::Message{propagation_source: _,message_id: _,message}))=>{
                        let s = String::from_utf8_lossy(&message.data);
                        self.tx.send(MyMessage::new(s.to_string())).unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
}
