// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![start_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
