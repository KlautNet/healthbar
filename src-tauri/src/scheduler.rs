use std::{collections::HashMap, time::Duration};

use tauri::{AppHandle, Emitter, Manager};
use tokio::time::Instant;

use crate::check::check_target;
use crate::model::{Shared, StatusUpdate};

const TICK: Duration = Duration::from_secs(1);

pub fn start(app: AppHandle) {
    let shared: Shared = app.state::<Shared>().inner().clone();

    tauri::async_runtime::spawn(async move {
        let client = reqwest::Client::new();
        let mut last_run: HashMap<String, Instant> = HashMap::new();

        loop {
            let targets = { shared.lock().unwrap().targets.clone() };
            let now = Instant::now();

            for target in &targets {
                let due = last_run
                    .get(&target.id)
                    .map(|t| now.duration_since(*t).as_secs() >= target.interval_secs)
                    .unwrap_or(true);
                if !due {
                    continue;
                }
                let (status, message) = check_target(&client, target).await;
                last_run.insert(target.id.clone(), Instant::now());

                let changed = {
                    let mut state = shared.lock().unwrap();
                    state.statuses.insert(target.id.clone(), status) != Some(status)
                };

                if changed {
                    let _ = app.emit(
                        "status-changed",
                        StatusUpdate {
                            id: target.id.clone(),
                            status,
                            message,
                        },
                    );
                    println!("Emitted status-changed event for target {}", target.id);
                }
            }

            last_run.retain(|id, _| targets.iter().any(|t| &t.id == id));

            tokio::time::sleep(TICK).await;
        }
    });
}
