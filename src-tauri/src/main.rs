// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{State, Window}; // Added Window

mod core;
mod launcher;

#[tauri::command]
async fn start_game(
    window: Window,
    version_id: String
) -> Result<String, String> {
    println!("Backend received StartGame for {}", version_id);

    // 1. Fetch manifest to find the version URL
    let manifest = core::manifest::fetch_version_manifest().await.map_err(|e| e.to_string())?;
    
    // Find the version info
    let version_info = manifest.versions.iter().find(|v| v.id == version_id)
        .ok_or_else(|| format!("Version {} not found in manifest", version_id))?;
    
    // 2. Fetch specific version JSON (client.jar info)
    let version_url = &version_info.url;
    let version_details: core::game_version::GameVersion = reqwest::get(version_url)
        .await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;

    // 3. Prepare download task for Client Jar
    let client_jar = version_details.downloads.client;
    // Where to save? Let's use ./versions/{version_id}/{version_id}.jar
    let mut path = std::path::PathBuf::from("versions");
    path.push(&version_id);
    path.push(format!("{}.jar", version_id));

    let task = core::downloader::DownloadTask {
        url: client_jar.url,
        path,
        sha1: Some(client_jar.sha1),
    };

    println!("Starting download of client jar...");
    
    // 4. Start Download
    core::downloader::download_files(window, vec![task]).await.map_err(|e| e.to_string())?;

    Ok(format!("Download complete for {}", version_id))
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

#[tauri::command]
async fn logout(state: State<'_, core::auth::AccountState>) -> Result<(), String> {
    *state.active_account.lock().unwrap() = None;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(core::auth::AccountState::new())
        .invoke_handler(tauri::generate_handler![start_game, get_versions, login_offline, get_active_account, logout])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
