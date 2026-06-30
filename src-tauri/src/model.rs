use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[default]
    Unknown,
    Up,
    Down,
    Degraded,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Check {
    Http {
        url: String,
        expected_status: Option<u16>,
    },
    Tcp {
        host: String,
        port: u16,
    },
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Target {
    pub id: String,
    pub name: String,
    pub check: Check,
    #[serde(default = "default_interval_secs")]
    pub interval_secs: u64,
    pub degraded_ms: Option<u64>,
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
}

fn default_interval_secs() -> u64 {
    60
}

fn default_timeout_ms() -> u64 {
    5000
}

#[derive(serde::Serialize, Clone)]
pub struct StatusUpdate {
    pub id: String,
    pub status: Status,
    pub message: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct TargetView {
    pub target: Target,
    pub status: Status,
}

#[derive(Default)]
pub struct AppState {
    pub targets: Vec<Target>,
    pub statuses: HashMap<String, Status>,
}

pub type Shared = Arc<Mutex<AppState>>;
