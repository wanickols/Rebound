mod game;
mod network;

use crate::game::gamemanager::GameManager;
use crate::game::gamepayload::GamePayload;

use crate::game::state::entityid::EntityId;
use crate::network::clientrequest::ClientRequest;
use crate::network::networkclient::NetworkClient;
use crate::network::networkmanager::{NetworkManager, Role};
use crate::network::serverevent::ServerEvent;

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tokio::sync::mpsc::{self, UnboundedReceiver};

#[tauri::command]
fn client_request(
    request: ClientRequest,
    client_sender: tauri::State<Arc<mpsc::UnboundedSender<ClientRequest>>>,
) {
    client_sender
        .send(request)
        .expect("Failed to send client request");
    // returns immediately, no async/await needed
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
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            let (snapshot_sender, snapshot_receiver) = mpsc::unbounded_channel::<ServerEvent>();
            let (client_request_sender, client_request_receiver) =
                mpsc::unbounded_channel::<ClientRequest>();

            let gm = Arc::new(Mutex::new(GameManager::new(
                320.0,
                180.0,
                Some(snapshot_sender),
            )));
            app.manage(gm.clone());

            let nm = Arc::new(tokio::sync::Mutex::new(None::<NetworkManager>));
            app.manage(nm.clone());

            let gm_for_loop = Arc::clone(&gm);

            spawn_client(client_request_sender);
            spawn_network_manager(
                Role::Host { port: 8080 },
                gm.clone(),
                nm.clone(),
                snapshot_receiver,
                client_request_receiver,
                app.handle().clone(),
            );
            start_game_loop(gm_for_loop);

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
                if let Some(sender) = gm.snapshot_sender.as_ref() {
                    let _ = sender.send(ServerEvent::WorldSnapshot {
                        snapshot: payload.clone(),
                    });
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
    nm_slot: Arc<tokio::sync::Mutex<Option<NetworkManager>>>,
    snapshot_receiver: mpsc::UnboundedReceiver<ServerEvent>,
    client_request_receiver: mpsc::UnboundedReceiver<ClientRequest>,
    app: tauri::AppHandle,
) {
    tauri::async_runtime::spawn(async move {
        match NetworkManager::new(
            role,
            Some(gm),
            snapshot_receiver,
            client_request_receiver,
            app,
        )
        .await
        {
            Ok(nm) => {
                *nm_slot.lock().await = Some(nm);
                println!("NetworkManager ready");
                start_network_loop(nm_slot);
            }
            Err(e) => {
                eprintln!("Failed to start NetworkManager: {}", e);
            }
        }
    });
}

fn start_network_loop(nm_slot: Arc<tokio::sync::Mutex<Option<NetworkManager>>>) {
    tauri::async_runtime::spawn(async move {
        loop {
            {
                let mut nm_lock = nm_slot.lock().await;
                if let Some(nm) = nm_lock.as_mut() {
                    nm.poll();

                    while let Ok(event) = nm.snapshot_receiver.try_recv() {
                        nm.send_server_event(event);
                    }

                    while let Ok(request) = nm.client_request_receiver.try_recv() {
                        nm.process_request(request);
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    });
}

fn spawn_client(client_request_sender: mpsc::UnboundedSender<ClientRequest>) {
    let _network_client = NetworkClient {
        client_request_sender: Arc::new(client_request_sender),
    };

    // You can now use `_network_client` to send requests
}
