mod core;
mod launcher;
mod ui;

use tokio::sync::mpsc;
use tokio::runtime::Runtime;
use std::thread;

fn main() {
    // channel for UI -> Backend
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn Tokio runtime in a background thread
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            println!("Backend started");
            while let Some(msg) = rx.recv().await {
                match msg {
                    ui::UiEvent::StartGame => {
                        println!("Backend received StartGame");
                        match core::manifest::fetch_version_manifest().await {
                            Ok(manifest) => {
                                println!("Fetched manifest. Latest release: {}", manifest.latest.release);
                                println!("Latest snapshot: {}", manifest.latest.snapshot);
                            }
                            Err(e) => {
                                eprintln!("Error fetching manifest: {}", e);
                            }
                        }
                    }
                }
            }
        });
    });

    // Run UI on main thread (must be main thread for GTK on some platforms)
    ui::init(tx);
}
