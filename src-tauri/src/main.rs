#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod state;
mod networkbehavior;
mod swarm;

use state::Stuff;
use tauri::State;

#[tauri::command]
fn bump_counter(state: State<Stuff>) -> i32 {
    let mut stuff_gaurd = state.0.lock().unwrap();
    stuff_gaurd.count += 1;
    stuff_gaurd.count
}

#[tauri::command]
fn get_counter(state: State<Stuff>) -> i32 {
    let stuff_gaurd = state.0.lock().unwrap();
    stuff_gaurd.count
}

fn on_page_load(window: tauri::window::Window, _: tauri::PageLoadPayload) {
    tauri::async_runtime::spawn(async move {

      /*window.on_window_event( |event| {
        match event {
            WindowEvent::Resized(_) => {},
            WindowEvent::Moved(_) => {},
            WindowEvent::CloseRequested { .. } => {},
            WindowEvent::Destroyed => {},
            WindowEvent::Focused(_) => {},
            WindowEvent::ScaleFactorChanged { .. } => {},
            WindowEvent::FileDrop(_) => {},
            WindowEvent::ThemeChanged(_) => {},
            _ => {},
        }
      });*/

      let mut count = 0;

      loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        count += 1;
        window.emit("app://count", count ).unwrap();
      }
    });
    ()
}

fn setup(_app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    Ok(())
}

#[tokio::main]
async fn main() {
    
    let stuff = Stuff::new().await;
    
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .setup(setup)
        .on_page_load(on_page_load)
        .manage(stuff)
        .invoke_handler(tauri::generate_handler![bump_counter, get_counter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
