// Copyright 2021 Jonas Kruckenberg
// SPDX-License-Identifier: MIT

mod ext;

pub use ext::*;
use tauri::{plugin::Plugin, Invoke, Result, Runtime};

#[tauri::command]
async fn move_window<R: Runtime>(mut window: tauri::Window<R>, position: Position) -> Result<()> {
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

  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }
}
