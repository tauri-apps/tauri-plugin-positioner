/**
 * Well known window positions.
 */
export declare enum Position {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
    TopCenter = 4,
    BottomCenter = 5,
    LeftCenter = 6,
    RightCenter = 7,
    Center = 8
}
/**
 * Moves the `Window` to the given {@link Position} using `WindowExt.move_window()`
 * All positions are relative to the **current** screen.
 *
 * @param to The {@link Position} to move to.
 */
export declare function move_window(to: Position): void;
