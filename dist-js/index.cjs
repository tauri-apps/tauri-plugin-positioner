'use strict';

var core = require('@tauri-apps/api/core');

// Copyright 2021 Jonas Kruckenberg
// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
/**
 * Well known window positions.
 */
exports.Position = void 0;
(function (Position) {
    Position[Position["TopLeft"] = 0] = "TopLeft";
    Position[Position["TopRight"] = 1] = "TopRight";
    Position[Position["BottomLeft"] = 2] = "BottomLeft";
    Position[Position["BottomRight"] = 3] = "BottomRight";
    Position[Position["TopCenter"] = 4] = "TopCenter";
    Position[Position["BottomCenter"] = 5] = "BottomCenter";
    Position[Position["LeftCenter"] = 6] = "LeftCenter";
    Position[Position["RightCenter"] = 7] = "RightCenter";
    Position[Position["Center"] = 8] = "Center";
    Position[Position["TrayLeft"] = 9] = "TrayLeft";
    Position[Position["TrayBottomLeft"] = 10] = "TrayBottomLeft";
    Position[Position["TrayRight"] = 11] = "TrayRight";
    Position[Position["TrayBottomRight"] = 12] = "TrayBottomRight";
    Position[Position["TrayCenter"] = 13] = "TrayCenter";
    Position[Position["TrayBottomCenter"] = 14] = "TrayBottomCenter";
})(exports.Position || (exports.Position = {}));
/**
 * Moves the `Window` to the given {@link Position} using `WindowExt.move_window()`
 * All positions are relative to the **current** screen.
 *
 * @param to The {@link Position} to move to.
 */
async function moveWindow(to) {
    await core.invoke('plugin:positioner|move_window', {
        position: to
    });
}

exports.moveWindow = moveWindow;
