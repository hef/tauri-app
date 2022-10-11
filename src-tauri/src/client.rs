use crate::eventloop::EventLoop;
use crate::networkbehavior::MyMessage;
use crate::swarm::build_swarm;
use libp2p::identity::Keypair;
use tokio::sync::{
    broadcast::{self, Receiver},
    mpsc,
};

pub struct Client {
    pub tx: broadcast::Sender<MyMessage>,
    pub tx2: mpsc::Sender<MyMessage>,
    pub peer_id: String,
}

impl Client {
    pub async fn new(identity: Keypair, port: u32) -> (Client, EventLoop) {
        let (tx, _rx) = broadcast::channel(2);
        let (tx2, rx2) = mpsc::channel(2);

        let swarm = build_swarm(identity, port).await;

        let c = Client {
            tx: tx.clone(),
            tx2,
            peer_id: swarm.local_peer_id().to_string(),
        };

        let event_loop = EventLoop::new(swarm, tx, rx2);

        (c, event_loop)
    }

    pub fn on_message(&self) -> Receiver<MyMessage> {
        self.tx.subscribe()
    }

    pub async fn send_message(&self, message: String) {
        let m = MyMessage::new(message);
        self.tx2.send(m).await.unwrap();
    }
}
