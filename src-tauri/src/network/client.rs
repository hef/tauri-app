use super::eventloop::EventLoop;
use super::networkbehavior::{MyBehaviour, MyMessage};
use libp2p::Swarm;
use tokio::sync::{
    broadcast::{self, Receiver},
    mpsc,
};

pub struct Client {
    pub tx: broadcast::Sender<MyMessage>,
    pub sender: mpsc::Sender<MyMessage>,
    pub peer_id: String,
}

impl Client {
    pub fn new(swarm: Swarm<MyBehaviour>) -> (Client, EventLoop) {
        let (tx, _rx) = broadcast::channel(2);
        let (tx2, rx2) = mpsc::channel(2);

        //let swarm = build_swarm(identity,listen_on).await;
        //let swarm = build_swarm_client(identity, listen_on).await;

        let c = Client {
            tx: tx.clone(),
            sender: tx2,
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
        self.sender.send(m).await.unwrap();
    }
}
