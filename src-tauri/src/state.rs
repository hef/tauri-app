
use libp2p::futures::StreamExt;
use libp2p::gossipsub::{IdentTopic, GossipsubEvent};
use libp2p::swarm::SwarmEvent;
use libp2p::Swarm;
use tokio::sync::{mpsc, broadcast};
use crate::networkbehavior::MyBehaviourEvent;
use crate::state::broadcast::Receiver;

use crate::{networkbehavior::{MyBehaviour, MyMessage}, swarm::build_swarm, };

pub struct InnerStuff {
    pub swarm: Swarm<MyBehaviour>,
    tx: broadcast::Sender<MyMessage>,
    rx: mpsc::Receiver<MyMessage>,
}

impl InnerStuff {
    pub async fn run(mut self) -> ! {
        let topic = IdentTopic::new("chat");
        loop{
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

pub struct Stuff
{
    pub tx: broadcast::Sender<MyMessage>,
    pub tx2: mpsc::Sender<MyMessage>,
}

impl Stuff {
    pub async fn new() -> Stuff {

        let (tx, _rx ) = broadcast::channel(2);
        let (tx2, rx2) = mpsc::channel(2);

        let s = Stuff {
            tx: tx.clone(),
            tx2,
        };

        let inner_stuff = InnerStuff {
            swarm: build_swarm().await,
            tx,
            rx: rx2,
        };

        tokio::spawn(async move {
            inner_stuff.run().await;
        });

        return s;
    }

    pub fn on_message(&self) -> Receiver<MyMessage>{
        self.tx.subscribe()
    }

    pub async fn send_message(&self, message: String) {
        let m = MyMessage::new(message);
        self.tx2.send(m).await.unwrap();
    }
}