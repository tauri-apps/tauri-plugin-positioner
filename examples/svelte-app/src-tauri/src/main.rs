#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::{Manager, SystemTray};
use tauri_plugin_positioner::{on_tray_event, Position, Positioner, WindowExt};

fn main() {
  tauri::Builder::default()
    .plugin(Positioner::default())
    .setup(|app| {
      let mut win = app.get_window("main").unwrap();

      tauri::async_runtime::spawn(async move {
        let _ = win.move_window(Position::TopRight);
      });

      Ok(())
    })
    .system_tray(SystemTray::new())
    .on_system_tray_event(|app, event| on_tray_event(app, &event))
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
