// Copyright 2021 Jonas Kruckenberg
// SPDX-License-Identifier: MIT

use serde_repr::Deserialize_repr;
use tauri::{PhysicalPosition, PhysicalSize, Result, Runtime, Window};

/// Well known window positions.
#[derive(Debug, Deserialize_repr)]
#[repr(u16)]
pub enum Position {
  TopLeft = 0,
  TopRight,
  BottomLeft,
  BottomRight,
  TopCenter,
  BottomCenter,
  LeftCenter,
  RightCenter,
  Center,
  // TrayLeft,
  // TrayBottomLeft,
  // TrayRight,
  // TrayBottomRight,
  // TrayCenter,
  // TrayBottomCenter,
}

/// A [`Window`] extension that provides extra methods related to positioning.
pub trait WindowExt {
  /// Moves the [`Window`] to the given [`Position`]
  ///
  /// All positions are relative to the **current** screen.
  fn move_window(&mut self, position: Position) -> Result<()>;
}

impl<R: Runtime> WindowExt for Window<R> {
  fn move_window(&mut self, pos: Position) -> Result<()> {
    use Position::*;

    let screen = self.current_monitor()?.unwrap();
    let screen_position = screen.position();
    let screen_size = PhysicalSize::<i32> {
      width: screen.size().width as i32,
      height: screen.size().height as i32,
    };
    let window_size = PhysicalSize::<i32> {
      width: self.outer_size()?.width as i32,
      height: self.outer_size()?.height as i32,
    };

    let physical_pos = match pos {
      TopLeft => *screen_position,
      TopRight => PhysicalPosition {
        x: screen_position.x + (screen_size.width - window_size.width),
        y: screen_position.y,
      },
      BottomLeft => PhysicalPosition {
        x: screen_position.x,
        y: screen_size.height - (window_size.height - screen_position.y),
      },
      BottomRight => PhysicalPosition {
        x: screen_position.x + (screen_size.width - window_size.width),
        y: screen_size.height - (window_size.height - screen_position.y),
      },
      TopCenter => PhysicalPosition {
        x: screen_position.x + ((screen_size.width / 2) - (window_size.width / 2)),
        y: screen_position.y,
      },
      BottomCenter => PhysicalPosition {
        x: screen_position.x + ((screen_size.width / 2) - (window_size.width / 2)),
        y: screen_size.height - (window_size.height - screen_position.y),
      },
      LeftCenter => PhysicalPosition {
        x: screen_position.x,
        y: screen_position.y + (screen_size.height / 2) - (window_size.height / 2),
      },
      RightCenter => PhysicalPosition {
        x: screen_position.x + (screen_size.width - window_size.width),
        y: screen_position.y + (screen_size.height / 2) - (window_size.height / 2),
      },
      Center => PhysicalPosition {
        x: screen_position.x + ((screen_size.width / 2) - (window_size.width / 2)),
        y: screen_position.y + (screen_size.height / 2) - (window_size.height / 2),
      },
    };

    self.set_position(tauri::Position::Physical(physical_pos))
  }
}
