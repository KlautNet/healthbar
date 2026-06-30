use std::process::Command;
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, PhysicalPosition,
};

#[tauri::command]
fn get_uptime() -> String {
    let output = Command::new("echo")
        .arg("Hello from Rust!")
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn show_panel(app: &tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("panel") {
        let _ = win.show();
        let _ = win.set_focus();
    }
}

fn position_panel(win: &tauri::WebviewWindow, click: PhysicalPosition<f64>) {
    if let Ok(size) = win.outer_size() {
        let x = (click.x - size.width as f64 / 2.0).max(8.0);
        let y = 28.0;
        let _ = win.set_position(PhysicalPosition::new(x, y));
    }
}

fn toggle_panel(app: &tauri::AppHandle, position: PhysicalPosition<f64>) {
    if let Some(win) = app.get_webview_window("panel") {
        if win.is_visible().unwrap_or(false) {
            let _ = win.hide();
        } else {
            position_panel(&win, position);
            let _ = win.show();
            let _ = win.set_focus();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_uptime])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let open_item = MenuItemBuilder::with_id("open", "Open").build(app)?;
            let close_item = MenuItemBuilder::with_id("close", "Close").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&open_item, &close_item])
                .build()?;

            TrayIconBuilder::with_id("main")
                .icon(Image::from_bytes(include_bytes!(
                    "../icons/tray-green.png"
                ))?)
                .icon_as_template(false)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "open" => show_panel(app),
                    "close" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        position,
                        ..
                    } = event
                    {
                        toggle_panel(tray.app_handle(), position);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
