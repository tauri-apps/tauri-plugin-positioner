import type { TrayIconEvent } from '@tauri-apps/api/tray';
/**
 * Well known window positions.
 */
export declare enum Position {
    /** Top left corner of the screen. */
    TopLeft = 0,
    /** Top right corner of the screen. */
    TopRight = 1,
    /** Bottom left corner of the screen. */
    BottomLeft = 2,
    /** Bottom right corner of the screen. */
    BottomRight = 3,
    /** Top center of the screen. */
    TopCenter = 4,
    /** Bottom center of the screen. */
    BottomCenter = 5,
    /** Vertically centered on the left edge of the screen. */
    LeftCenter = 6,
    /** Vertically centered on the right edge of the screen. */
    RightCenter = 7,
    /** Center of the screen. */
    Center = 8,
    /**
     * Above the tray icon, aligned with its left edge. Requires the `tray-icon` feature and the
     * tray icon's events to be forwarded through {@link handleIconState}.
     */
    TrayLeft = 9,
    /**
     * Directly below the tray icon, aligned with its left edge. Requires the `tray-icon` feature
     * and the tray icon's events to be forwarded through {@link handleIconState}.
     */
    TrayBottomLeft = 10,
    /**
     * Above the tray icon, aligned with its right edge. Requires the `tray-icon` feature and the
     * tray icon's events to be forwarded through {@link handleIconState}.
     */
    TrayRight = 11,
    /**
     * Directly below the tray icon, aligned with its right edge. Requires the `tray-icon` feature
     * and the tray icon's events to be forwarded through {@link handleIconState}.
     */
    TrayBottomRight = 12,
    /**
     * Above the tray icon, horizontally centered on it. Requires the `tray-icon` feature and the
     * tray icon's events to be forwarded through {@link handleIconState}.
     */
    TrayCenter = 13,
    /**
     * Directly below the tray icon, horizontally centered on it. Requires the `tray-icon` feature
     * and the tray icon's events to be forwarded through {@link handleIconState}.
     */
    TrayBottomCenter = 14
}
/**
 * Moves the `Window` to the given {@link Position} using `WindowExt.move_window()`
 * All positions are relative to the **current** screen.
 *
 * @example
 * ```typescript
 * import { moveWindow, Position } from '@tauri-apps/plugin-positioner'
 *
 * await moveWindow(Position.TopRight)
 * ```
 *
 * @param to The {@link Position} to move to.
 * @since 2.0.0
 */
export declare function moveWindow(to: Position): Promise<void>;
/**
 * Moves the `Window` to the given {@link Position} using `WindowExt.move_window_constrained()`
 *
 * This move operation constrains the window to the screen dimensions in case of
 * tray-icon positions.
 *
 * @example
 * ```typescript
 * import { moveWindowConstrained, Position } from '@tauri-apps/plugin-positioner'
 *
 * await moveWindowConstrained(Position.TrayCenter)
 * ```
 *
 * @param to The (tray) {@link Position} to move to.
 * @since 2.1.0
 */
export declare function moveWindowConstrained(to: Position): Promise<void>;
/**
 * Reports the tray icon's current position and size so the `Tray*` {@link Position} variants
 * can be resolved.
 *
 * This is an internal helper meant to be called from the tray icon's event handler (the
 * `action` callback passed to `TrayIcon.new()`), forwarding every {@link TrayIconEvent} it
 * receives so that {@link moveWindow} and {@link moveWindowConstrained} can later position the
 * window relative to the tray icon.
 *
 * @example
 * ```typescript
 * import { handleIconState } from '@tauri-apps/plugin-positioner'
 * import { TrayIcon, type TrayIconEvent } from '@tauri-apps/api/tray'
 *
 * const action = async (event: TrayIconEvent) => {
 *   await handleIconState(event)
 * }
 *
 * const tray = await TrayIcon.new({ id: 'main', action })
 * ```
 *
 * @param event The tray icon event to read the position and size from.
 * @since 2.0.0
 */
export declare function handleIconState(event: TrayIconEvent): Promise<void>;
