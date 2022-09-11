use std::sync::Mutex;

use libp2p::{Swarm};
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::mpsc;
use crate::state::broadcast::Receiver;

use crate::{networkbehavior::{MyBehaviour, MyMessage}, swarm::build_swarm, };

pub struct InnerStuff {
    pub count: i32,
    pub swarm: Swarm<MyBehaviour>,
    pub tx: broadcast::Sender<MyMessage>,
    pub tx2: mpsc::Sender<MyMessage>,
    pub rx2: mpsc::Receiver<MyMessage>,

}

pub struct Stuff(pub Mutex<InnerStuff>);

impl Stuff {
    pub async fn new() -> Stuff {

        let (tx, _rx ) = broadcast::channel(2);
        let (tx2, rx2) = mpsc::channel(2);

        Stuff(Mutex::new(InnerStuff {
             count: 0, 
             swarm: build_swarm(tx.clone()).await,
             tx,
             tx2,
             rx2,
            }))
    }

    pub fn on_message(&self) -> Receiver<MyMessage>{
        self.0.lock().unwrap().tx.subscribe()
    }

    pub async fn send_message(&self, message: String) {
        let m = MyMessage::new(message);
        self.0.lock().unwrap().tx2.send(m).await.unwrap();
    }
}