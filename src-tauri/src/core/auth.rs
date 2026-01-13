use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineAccount {
    pub username: String,
    pub uuid: String,
}

pub struct AccountState {
    pub active_account: Mutex<Option<OfflineAccount>>,
}

impl AccountState {
    pub fn new() -> Self {
        Self {
            active_account: Mutex::new(None),
        }
    }
}

pub fn generate_offline_uuid(username: &str) -> String {
    // Generate a UUID v3 (MD5-based) using the username as the name
    // This provides a consistent UUID for the same username
    let namespace = Uuid::NAMESPACE_OID;
    Uuid::new_v3(&namespace, username.as_bytes()).to_string()
}
