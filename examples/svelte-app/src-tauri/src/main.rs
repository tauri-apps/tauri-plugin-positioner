#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::Manager;
use tauri_plugin_window_positioner::{WindowExt, Position, Positioner};

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
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
