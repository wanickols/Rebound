mod game;

use crate::game::gamemanager::GameManager;
use crate::game::input::GameAction;
use crate::game::playerid::PlayerId;
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager};

#[tauri::command]
fn input_event(
    id: u32,
    action: GameAction,
    pressed: bool,
    gm: tauri::State<Arc<Mutex<GameManager>>>,
) {
    let player_id = PlayerId(id);
    let mut gm = gm.lock().unwrap();
    gm.set_input(player_id, action, pressed);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let gm = Arc::new(Mutex::new(GameManager::new(app.handle(), 320.0, 180.0)));
            app.manage(gm.clone());

            let gm_for_loop = Arc::clone(&gm);
            start_game_loop(gm_for_loop);
            Ok(())
        }) // makes GameManager available to all commands
        .invoke_handler(tauri::generate_handler![input_event,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::thread;
use std::time::{Duration, Instant};

fn start_game_loop(gm: Arc<Mutex<GameManager>>) {
    thread::spawn(move || {
        let tick_rate = 60.0; // Hz
        let tick_duration = Duration::from_secs_f32(1.0 / tick_rate);

        loop {
            let start = Instant::now();

            {
                // lock the GameManager inside the loop
                let mut gm = gm.lock().unwrap();
                gm.update(); // apply input & physics + emit state

                gm.app.emit("game-state", gm.states.clone()).unwrap();
            }

            // sleep the remainder of the tick
            let elapsed = start.elapsed();
            if elapsed < tick_duration {
                thread::sleep(tick_duration - elapsed);
            }
        }
    });
}
