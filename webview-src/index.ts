// Copyright 2021 Jonas Kruckenberg
// SPDX-License-Identifier: MIT

import { invoke } from "@tauri-apps/api/tauri";

export enum Position {
  TopLeft = 0,
  TopRight,
  BottomLeft,
  BottomRight,
  TopCenter,
  BottomCenter,
  LeftCenter,
  RightCenter,
  Center,
  //   TrayLeft,
  //   TrayBottomLeft,
  //   TrayRight,
  //   TrayBottomRight,
  //   TrayCenter,
  //   TrayBottomCenter,
}

export function move_window(to: Position) {
  invoke('plugin:positioner|move_window', {
    position: to
  })
}