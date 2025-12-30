mod game;
mod network;

use crate::game::gamemanager::GameManager;
use crate::game::gamepayload::GamePayload;

use crate::game::state::entityid::EntityId;
use crate::network::clientrequest::ClientRequest;
use crate::network::networkmanager::{NetworkManager, Role};
use crate::network::serverevent::ServerEvent;

use std::sync::{Arc, Mutex};
use tauri::Manager;

#[tauri::command]
fn client_request(
    request: ClientRequest,
    nm: tauri::State<Arc<Mutex<NetworkManager>>>,
) -> Option<EntityId> {
    let mut nm = nm.lock().unwrap();
    return nm.process_request(request);
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
pub async fn run() -> std::io::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let gm = Arc::new(Mutex::new(GameManager::new(app.handle(), 320.0, 180.0)));
            app.manage(gm.clone());

            let nm = Arc::new(Mutex::new(None::<NetworkManager>));
            app.manage(nm.clone());

            let gm_for_loop = Arc::clone(&gm);

            spawn_network_manager(Role::Host { port: 8080 }, gm.clone(), nm.clone());
            start_game_loop(gm_for_loop, nm.clone());

            Ok(())
        }) // makes GameManager available to all commands
        .invoke_handler(tauri::generate_handler![
            set_game_settings,
            client_request,
            start_game,
            end_game,
            quit_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

use std::thread;
use std::time::{Duration, Instant};

fn start_game_loop(gm: Arc<Mutex<GameManager>>, nm: Arc<Mutex<Option<NetworkManager>>>) {
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

                let nmm = nm.lock().unwrap();
                if let Some(nm) = nmm.as_ref() {
                    nm.send_server_event(
                        &gm,
                        ServerEvent::WorldSnapshot {
                            snapshot: payload.clone(),
                        },
                    );
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

fn spawn_network_manager(
    role: Role,
    gm: Arc<Mutex<GameManager>>,
    nm_slot: Arc<Mutex<Option<NetworkManager>>>,
) {
    tauri::async_runtime::spawn(async move {
        match NetworkManager::new(role, Some(gm)).await {
            Ok(nm) => {
                *nm_slot.lock().unwrap() = Some(nm);
                println!("NetworkManager ready");
            }
            Err(e) => {
                eprintln!("Failed to start NetworkManager: {}", e);
            }
        }
    });
}
