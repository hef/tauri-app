#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::State;
use std::sync::Mutex;
use libp2p::{identity, PeerId};
use libp2p::gossipsub::{Gossipsub, GossipsubEvent, MessageAuthenticity};
use libp2p::swarm::{NetworkBehaviourEventProcess, SwarmBuilder};
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::NetworkBehaviour;
use libp2p::gossipsub;

pub struct InnerStuff {
  pub count: i32,
}

pub struct Stuff(pub Mutex<InnerStuff>);

#[tauri::command]
fn bump_counter( state: State<Stuff>) -> i32 {
  
  let mut stuff_gaurd = state.0.lock().unwrap();
  stuff_gaurd.count += 1;
  stuff_gaurd.count
}

#[tauri::command]
fn get_counter( state: State<Stuff>) -> i32 {
  let stuff_gaurd = state.0.lock().unwrap();
  stuff_gaurd.count
}

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct MyBehaviour {
    gossipsub: Gossipsub,
    mdns: Mdns,
    //ping: Ping,
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

fn on_page_load(window: tauri::window::Window, _: tauri::PageLoadPayload) {
  println!("hello page load!");
  let window2 = window.clone();
  tauri::async_runtime::spawn(async move {
    tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
    println!("foo: bar");
    window2.emit("foo", "bar").unwrap();
  });
  ()
}

fn setup( _app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
  tauri::async_runtime::spawn( async {
    let local_key = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(local_key.public());
    let transport = libp2p::tokio_development_transport(local_key.clone()).unwrap();
    let gossipsub_config = gossipsub::GossipsubConfigBuilder::default().build().unwrap();
    
    let mut swarm = {
      let behavior = MyBehaviour {
        gossipsub: Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config).unwrap(),
        mdns: Mdns::new(Default::default()).await.unwrap()
      };
      SwarmBuilder::new(transport, behavior, peer_id)
      .executor(Box::new(|fut| {
        tauri::async_runtime::spawn(fut);
      }))
      .build()
    };
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
  });


  Ok(())
}

fn main() {
  println!("Hello, Rust!");
  tauri::Builder::default()
  .setup(setup)
  .on_page_load(on_page_load)
    .manage( Stuff(Mutex::new(InnerStuff { count: 0 })) )
    .invoke_handler(tauri::generate_handler![bump_counter, get_counter])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
