import { invoke } from '@tauri-apps/api/tauri';

// Copyright 2021 Jonas Kruckenberg
/**
 * Well known window positions.
 */
var Position;
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
    //   TrayLeft,
    //   TrayBottomLeft,
    //   TrayRight,
    //   TrayBottomRight,
    //   TrayCenter,
    //   TrayBottomCenter,
})(Position || (Position = {}));
/**
 * Moves the `Window` to the given {@link Position} using `WindowExt.move_window()`
 * All positions are relative to the **current** screen.
 *
 * @param to The {@link Position} to move to.
 */
function move_window(to) {
    invoke('plugin:positioner|move_window', {
        position: to
    });
}

export { Position, move_window };
