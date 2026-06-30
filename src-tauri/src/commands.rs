use std::collections::HashMap;

use tauri::{AppHandle, State};

use crate::model::{Shared, Status, Target};
use crate::store;

#[tauri::command]
pub fn list_targets(state: State<Shared>) -> Vec<Target> {
    state.lock().unwrap().targets.clone()
}

#[tauri::command]
pub fn get_statuses(state: State<Shared>) -> HashMap<String, Status> {
    state.lock().unwrap().statuses.clone()
}

#[tauri::command]
pub fn add_target(app: AppHandle, state: State<Shared>, target: Target) -> Result<(), String> {
    let targets = {
        let mut app_state = state.lock().unwrap();
        if let Some(existing) = app_state.targets.iter_mut().find(|t| t.id == target.id) {
            *existing = target;
        } else {
            app_state.targets.push(target);
        }
        app_state.targets.clone()
    };
    store::save_targets(&app, &targets)
}

#[tauri::command]
pub fn remove_target(app: AppHandle, state: State<Shared>, id: String) -> Result<(), String> {
    let targets = {
        let mut app_state = state.lock().unwrap();
        app_state.targets.retain(|t| t.id != id);
        app_state.statuses.remove(&id);
        app_state.targets.clone()
    };
    store::save_targets(&app, &targets)
}
