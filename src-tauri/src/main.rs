#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod state;
mod networkbehavior;
mod swarm;

use state::Stuff;
use tauri::{State, Manager, WindowEvent};

#[tauri::command]
fn bump_counter(state: State<Stuff>) -> i32 {
    let mut stuff_gaurd = state.0.lock().unwrap();
    stuff_gaurd.count += 1;
    stuff_gaurd.count
}

#[tauri::command]
fn send_message(state: State<Stuff>, message: String) {
    state.send_message(message);
}

#[tauri::command]
fn get_counter(state: State<Stuff>) -> i32 {
    let stuff_gaurd = state.0.lock().unwrap();
    stuff_gaurd.count
}

fn on_page_load(window: tauri::window::Window, _: tauri::PageLoadPayload) {
    tauri::async_runtime::spawn(async move {

        let (tx, mut window_close_rx) = tokio::sync::mpsc::channel::<bool>(1);

        window.on_window_event(move |event| {
        match event {
            WindowEvent::Resized(_) => {},
            WindowEvent::Moved(_) => {},
            WindowEvent::CloseRequested { .. } => { let _ = tx.send(true); },
            WindowEvent::Destroyed => {},
            WindowEvent::Focused(_) => {},
            WindowEvent::ScaleFactorChanged { .. } => {},
            WindowEvent::FileDrop(_) => {},
            WindowEvent::ThemeChanged(_) => {},
            _ => {},
        }
        });

        let mut message_rx = window.state::<Stuff>().0.lock().unwrap().tx.subscribe();

        loop {
            tokio::select! {
                m = message_rx.recv() => {
                    match m {
                        Ok(message) => {
                            window.emit("app://message", message).unwrap()
                        },
                        Err(_) => todo!(),
                    }
                }
                _ = window_close_rx.recv() => { return }
            };
        }
    });
    ()
}

fn setup(_app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    Ok(())
}

#[tokio::main]
async fn main() {

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let stuff = Stuff::new().await;

    tauri::Builder::default()
        .setup(setup)
        .on_page_load(on_page_load)
        .manage(stuff)
        .invoke_handler(tauri::generate_handler![bump_counter, get_counter, send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
