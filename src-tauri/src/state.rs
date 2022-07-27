use std::sync::Mutex;

use libp2p::{Swarm};
use tokio::sync::broadcast::{self, Sender};

use crate::{networkbehavior::{MyBehaviour, MyMessage}, swarm::build_swarm, };

pub struct InnerStuff {
    pub count: i32,
    pub swarm: Swarm<MyBehaviour>,
    pub tx: Sender<MyMessage>
}

pub struct Stuff(pub Mutex<InnerStuff>);

impl Stuff {
    pub async fn new() -> Stuff {

        let (tx, _rx ) = broadcast::channel(2);

        Stuff(Mutex::new(InnerStuff {
             count: 0, 
             swarm: build_swarm(tx.clone()).await,
             tx,
            }))
    }       
}