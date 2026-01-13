use std::path::PathBuf;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub url: String,
    pub path: PathBuf,
    pub sha1: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadProgress {
    Started(String),
    Progress { file: String, downloaded: u64, total: u64 },
    Finished(String),
    Error(String, String),
}

pub struct Downloader {
    sender: mpsc::Sender<DownloadProgress>,
}

impl Downloader {
    pub fn new(sender: mpsc::Sender<DownloadProgress>) -> Self {
        Self { sender }
    }

    pub async fn download(&self, tasks: Vec<DownloadTask>) {
        // TODO: Implement parallel download with limits
        // Use futures::stream::StreamExt::buffer_unordered
        
        for task in tasks {
            if let Err(_) = self.sender.send(DownloadProgress::Started(task.url.clone())).await {
                break;
            }

            // Simulate download for now or implement basic
            // Ensure directory exists
            if let Some(parent) = task.path.parent() {
                 let _ = tokio::fs::create_dir_all(parent).await;
            }

            // Real implementation would use reqwest here
            
            if let Err(_) = self.sender.send(DownloadProgress::Finished(task.url)).await {
                break;
            }
        }
    }
}
