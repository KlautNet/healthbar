mod check;
mod commands;
mod model;
mod scheduler;
mod store;
mod tray;

use tauri::Manager;

use model::Shared;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check::get_target_status,
            commands::list_targets,
            commands::get_statuses,
            commands::add_target,
            commands::remove_target,
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let shared = Shared::default();
            shared.lock().unwrap().targets = store::load_targets(app.handle());
            app.manage(shared);

            tray::setup(app)?;
            scheduler::start(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
