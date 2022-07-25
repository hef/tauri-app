use std::sync::Mutex;

use libp2p::Swarm;

use crate::{networkbehavior::MyBehaviour, swarm::build_swarm};

pub struct InnerStuff {
    pub count: i32,
    pub swarm: Swarm<MyBehaviour>,
}

pub struct Stuff(pub Mutex<InnerStuff>);


impl Stuff {
    pub async fn new() -> Stuff {
        Stuff(Mutex::new(InnerStuff {
             count: 0, 
             swarm: build_swarm().await,
            }))
    }
}