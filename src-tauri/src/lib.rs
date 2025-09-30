mod game;

use std::sync::{Arc, Mutex};
use crate::game::gamemanager::GameManager;

use tauri::{Manager, State, Emitter};


#[tauri::command]
fn input_event(
    key: &str,
    pressed: bool,
    gm: tauri::State<Arc<Mutex<GameManager>>>,
) {
    let mut gm = gm.lock().unwrap();
    gm.set_input(key, pressed);
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
         .setup(|app| {
            let gm = Arc::new(Mutex::new(GameManager::new(app.handle())));
            app.manage(gm.clone());


            let gm_for_loop = Arc::clone(&gm);
            start_game_loop(gm_for_loop);
            Ok(())
        }) // makes GameManager available to all commands
        .invoke_handler(tauri::generate_handler![
            input_event,
        ])
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

                gm.app.emit("game-state", gm.state.clone()).unwrap();

            }

            // sleep the remainder of the tick
            let elapsed = start.elapsed();
            if elapsed < tick_duration {
                thread::sleep(tick_duration - elapsed);
            }
        }
    });
}