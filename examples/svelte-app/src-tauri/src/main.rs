#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::{Manager, SystemTray, SystemTrayEvent, CustomMenuItem, SystemTrayMenu};
use tauri_plugin_positioner::{on_tray_event, Position, Positioner, WindowExt};

fn main() {
  let tray_left = CustomMenuItem::new("tray_left".to_string(), "TrayLeft");
  let tray_bottom_left = CustomMenuItem::new("tray_bottom_left".to_string(), "TrayBottomLeft");
  let tray_right = CustomMenuItem::new("tray_right".to_string(), "TrayRight");
  let tray_bottom_right = CustomMenuItem::new("tray_bottom_right".to_string(), "TrayBottomRight");
  let tray_center = CustomMenuItem::new("tray_center".to_string(), "TrayCenter");
  let tray_bottom_center = CustomMenuItem::new("tray_bottom_center".to_string(), "TrayBottomCenter");

  let tray_menu = SystemTrayMenu::new()
    .add_item(tray_left)
    .add_item(tray_bottom_left)
    .add_item(tray_right)
    .add_item(tray_bottom_right)
    .add_item(tray_center)
    .add_item(tray_bottom_center);

  let system_tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
    .plugin(Positioner::default())
    .setup(|app| {
      let win = app.get_window("main").unwrap();

      let _ = win.move_window(Position::TopLeft);

      Ok(())
    })
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| {
      on_tray_event(app, &event);
      if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        let win = app.get_window("main").unwrap();

        let _ = match id.as_str() {
          "tray_left" => win.move_window(Position::TrayLeft),
          "tray_bottom_left" => win.move_window(Position::TrayBottomLeft),
          "tray_right" => win.move_window(Position::TrayRight),
          "tray_bottom_right" => win.move_window(Position::TrayBottomRight),
          "tray_center" => win.move_window(Position::TrayCenter),
          "tray_bottom_center" => win.move_window(Position::TrayBottomCenter),
          _ => Ok(())
        };
      }
    })
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
