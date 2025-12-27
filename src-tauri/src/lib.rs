mod game;

use crate::game::gamemanager::GameManager;
use crate::game::gamepayload::GamePayload;
use crate::game::input::InputFrame;
use crate::game::state::entityid::EntityId;

use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager};

#[tauri::command]
fn input_frame(gm: tauri::State<Arc<Mutex<GameManager>>>, id: u32, frame: InputFrame) {
    // store in HashMap<PlayerId, InputFrame> per tick
    let mut gm = gm.lock().unwrap();
    let player_id = EntityId(id);
    gm.set_input(player_id, frame);
}

#[tauri::command]
fn set_game_settings(
    player_count: u8,
    target_score: u8,
    gm: tauri::State<Arc<Mutex<GameManager>>>,
) {
    println!(
        "Got the player count: {} and target score: {}",
        player_count, target_score
    );

    let mut gm = gm.lock().unwrap();
    gm.set_game_settings(player_count, target_score);
}

#[tauri::command]
fn request_player_id(gm: tauri::State<Arc<Mutex<GameManager>>>) -> Option<EntityId> {
    let mut gm = gm.lock().unwrap();

    let pid = gm.try_get_new_player();
    println!("Sending player id: {:?}", pid);
    pid
}

#[tauri::command]
fn remove_player(id: u32, gm: tauri::State<Arc<Mutex<GameManager>>>) {
    let mut gm = gm.lock().unwrap();
    let player_id = EntityId(id);
    gm.remove_player(player_id);
}

#[tauri::command]
fn start_game(gm: tauri::State<Arc<Mutex<GameManager>>>) {
    let mut gm = gm.lock().unwrap();
    gm.start_game();
}

//This is for ending a game session, like a round
#[tauri::command]
fn end_game(gm: tauri::State<Arc<Mutex<GameManager>>>) {
    let mut gm = gm.lock().unwrap();
    gm.end_game();
}

//This is for quitting the game where you'd reset #players etc.
#[tauri::command]
fn quit_game(gm: tauri::State<Arc<Mutex<GameManager>>>) {
    let mut gm = gm.lock().unwrap();
    gm.quit_game();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let gm = Arc::new(Mutex::new(GameManager::new(app.handle(), 320.0, 180.0)));
            app.manage(gm.clone());

            let gm_for_loop = Arc::clone(&gm);
            start_game_loop(gm_for_loop);
            Ok(())
        }) // makes GameManager available to all commands
        .invoke_handler(tauri::generate_handler![
            input_frame,
            set_game_settings,
            request_player_id,
            start_game,
            end_game,
            quit_game,
            remove_player,
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

                let payload = GamePayload::from(&*gm);
                if let Err(err) = gm.app.emit("game-state", payload) {
                    eprintln!("Failed to emit game-state: {}", err);
                }
            }

            // sleep the remainder of the tick
            let elapsed = start.elapsed();
            if elapsed < tick_duration {
                thread::sleep(tick_duration - elapsed);
            }
        }
    });
}
