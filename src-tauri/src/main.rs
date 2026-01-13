// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::State;

mod core;
mod launcher;

#[tauri::command]
async fn start_game() -> Result<String, String> {
    println!("Backend received StartGame");
    match core::manifest::fetch_version_manifest().await {
        Ok(manifest) => {
            let msg = format!(
                "Fetched manifest.\nLatest release: {}\nLatest snapshot: {}",
                manifest.latest.release, manifest.latest.snapshot
            );
            println!("{}", msg);
            Ok(msg)
        }
        Err(e) => {
            eprintln!("Error fetching manifest: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn get_versions() -> Result<Vec<core::manifest::Version>, String> {
    match core::manifest::fetch_version_manifest().await {
        Ok(manifest) => Ok(manifest.versions),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn login_offline(
    state: State<'_, core::auth::AccountState>,
    username: String,
) -> Result<core::auth::OfflineAccount, String> {
    let uuid = core::auth::generate_offline_uuid(&username);
    let account = core::auth::OfflineAccount {
        username,
        uuid,
    };
    
    *state.active_account.lock().unwrap() = Some(account.clone());
    Ok(account)
}

#[tauri::command]
async fn get_active_account(
    state: State<'_, core::auth::AccountState>,
) -> Result<Option<core::auth::OfflineAccount>, String> {
    Ok(state.active_account.lock().unwrap().clone())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(core::auth::AccountState::new())
        .invoke_handler(tauri::generate_handler![start_game, get_versions, login_offline, get_active_account])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
