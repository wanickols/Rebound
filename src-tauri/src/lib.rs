mod game;
mod network;
mod startup;

use crate::game::gamemanager::GameManager;
use crate::game::gamepayload::GamePayload;

use crate::network::clientrequest::ClientRequest;
use crate::network::serverevent::ServerEvent;
use crate::startup::startup::{ManagedSenders, StartupManager};

use std::sync::{Arc, Mutex};
use tauri::Manager;

type SharedManager = Arc<tokio::sync::Mutex<StartupManager>>;

#[tauri::command]
fn client_request(managed: tauri::State<ManagedSenders>, request: ClientRequest) {
    // lock the inner senders
    let senders = managed.inner.lock().unwrap();
    // send on the specific channel
    let _ = senders.frontend_request_tx.send(request);
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
async fn host_game<'a>(port: u16, startup: tauri::State<'a, SharedManager>) -> Result<(), ()> {
    let mut start_lock = startup.lock().await;
    let port = if port == 0 { 8080 } else { port };
    start_lock.init_host(port).await;
    Ok(())
}

#[tauri::command]
async fn join_game<'a>(port: u16, startup: tauri::State<'a, SharedManager>) -> Result<(), ()> {
    let mut start_lock = startup.lock().await;
    let port = if port == 0 { 8080 } else { port };
    start_lock.init_join(port).await;
    Ok(())
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

#[tauri::command]
fn list_directories(root_path: String) -> Result<Vec<String>, String> {
    let path = Path::new(&root_path);

    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;

    let mut dirs = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let metadata = entry.metadata().map_err(|e| e.to_string())?;

        if metadata.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                dirs.push(name.to_string());
            }
        }
    }

    Ok(dirs)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> std::io::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            //game manager
            let gm = Arc::new(Mutex::new(GameManager::new(320.0, 180.0)));
            app.manage(gm.clone());

            let gm_for_loop = Arc::clone(&gm);
            //startup
            let startup: SharedManager = Arc::new(tokio::sync::Mutex::new(StartupManager::new(
                gm,
                app.handle().clone(),
            )));

            app.manage(startup.clone());

            //game loop
            start_game_loop(gm_for_loop);

            Ok(())
        }) // makes GameManager available to all commands
        .invoke_handler(tauri::generate_handler![
            set_game_settings,
            client_request,
            start_game,
            host_game,
            join_game,
            end_game,
            list_directories,
            quit_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
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
                if let Some(sender) = gm.snapshot_tx.as_ref() {
                    let _ = sender.send(ServerEvent::WorldSnapshot { snapshot: payload });
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
