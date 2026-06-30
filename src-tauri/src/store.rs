use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::model::Target;

fn default_targets() -> Vec<Target> {
    vec![
        Target {
            id: "google".into(),
            name: "Google".into(),
            check: crate::model::Check::Http {
                url: "https://www.google.com".into(),
                expected_status: Some(200),
            },
            interval_secs: 60,
            timeout_ms: 5000,
            degraded_ms: Some(1000),
        },
        Target {
            id: "github".into(),
            name: "GitHub".into(),
            check: crate::model::Check::Tcp {
                host: "localhost".into(),
                port: 22,
            },
            interval_secs: 60,
            timeout_ms: 5000,
            degraded_ms: Some(1000),
        },
    ]
}

fn targets_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("targets.json"))
}

pub fn load_targets(app: &AppHandle) -> Vec<Target> {
    let Ok(path) = targets_path(app) else {
        return Vec::new();
    };
    let Ok(data) = std::fs::read_to_string(&path) else {
        return default_targets();
    };
    serde_json::from_str(&data).unwrap_or_else(|_| default_targets())
}

pub fn save_targets(app: &AppHandle, targets: &[Target]) -> Result<(), String> {
    let path = targets_path(app)?;
    let data = serde_json::to_string_pretty(targets).map_err(|e| e.to_string())?;
    std::fs::write(path, data).map_err(|e| e.to_string())
}
