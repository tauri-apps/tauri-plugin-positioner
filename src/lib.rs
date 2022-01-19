// Copyright 2021 Jonas Kruckenberg
// SPDX-License-Identifier: MIT

mod ext;

pub use ext::*;
use tauri::{plugin::Plugin, Invoke, Result, Runtime};

#[cfg(feature = "system-tray")]
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, SystemTrayEvent};

#[cfg(feature = "system-tray")]
struct Tray(std::sync::Mutex<Option<(PhysicalPosition<f64>, PhysicalSize<f64>)>>);

#[cfg(feature = "system-tray")]
pub fn on_tray_event<R: Runtime>(app: &AppHandle<R>, event: &SystemTrayEvent) {
  match event {
    SystemTrayEvent::LeftClick { position, size, .. } => {
      app
        .state::<Tray>()
        .0
        .lock()
        .unwrap()
        .replace((*position, *size));
    }
    SystemTrayEvent::RightClick { position, size, .. } => {
      app
        .state::<Tray>()
        .0
        .lock()
        .unwrap()
        .replace((*position, *size));
    }
    SystemTrayEvent::DoubleClick { position, size, .. } => {
      app
        .state::<Tray>()
        .0
        .lock()
        .unwrap()
        .replace((*position, *size));
    }
    _ => (),
  }
}

#[tauri::command]
async fn move_window<R: Runtime>(window: tauri::Window<R>, position: Position) -> Result<()> {
  window.move_window(position)
}

/// The tauri plugin that exposes [`WindowExt::move_window`] to the webview.
pub struct Positioner<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> Default for Positioner<R> {
  fn default() -> Self {
    Self {
      invoke_handler: Box::new(tauri::generate_handler![move_window]),
    }
  }
}

impl<R: Runtime> Plugin<R> for Positioner<R> {
  fn name(&self) -> &'static str {
    "positioner"
  }

  #[cfg(feature = "system-tray")]
  fn initialize(
    &mut self,
    app: &AppHandle<R>,
    _config: serde_json::Value,
  ) -> tauri::plugin::Result<()> {
    app.manage(Tray(std::sync::Mutex::new(None)));
    Ok(())
  }

  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }
}
