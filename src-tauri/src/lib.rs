mod util;
mod game;

use crate::util::input::InputHandler;
use std::sync::Mutex; 
use crate::game::gamemanager::GameManager;

use tauri::{AppHandle, Manager, State};


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn input_event(
    key: String,
    pressed: bool,
    gm: State<Mutex<GameManager>>, // Tauri provides access to shared state
) {
    let mut gm = gm.lock().unwrap(); // lock mutex for mutable access
    InputHandler::handle_input(key, pressed, &mut gm);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
         .setup(|app| {
            let gm = GameManager::new(app.handle());
            app.manage(Mutex::new(gm)); // now globally available
            Ok(())
        }) // makes GameManager available to all commands
        .invoke_handler(tauri::generate_handler![
            input_event,
            greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

