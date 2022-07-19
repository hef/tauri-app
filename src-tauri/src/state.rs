use std::sync::Mutex;

pub struct InnerStuff {
    pub count: i32,
    //pub swarm: Box<Swarm<MyBehaviour>>,
}

pub struct Stuff(pub Mutex<InnerStuff>);


impl Stuff {
    pub fn new() -> Stuff {
        Stuff(Mutex::new(InnerStuff { count: 0 }))
    }
    
}