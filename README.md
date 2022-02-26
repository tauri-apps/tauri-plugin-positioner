# Tauri plugin positioner

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/tauri-plugin-positioner.svg
[crates-url]: https://crates.io/crates/tauri-plugin-positioner
[docs-badge]: https://img.shields.io/docsrs/tauri-plugin-positioner.svg
[docs-url]: https://docs.rs/tauri-plugin-positioner
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

A plugin for Tauri that helps position your windows at well-known locations.

This plugin is a port of [electron-positioner](https://github.com/jenslind/electron-positioner) for [tauri](https://tauri.studio).

## Install

### Rust

```toml
[dependencies]
tauri-plugin-positioner = "0.2"
```

### JavaScript

```
pnpm add tauri-plugin-positioner
# or
npm install tauri-plugin-positioner
# or
yarn add tauri-plugin-positioner
```

## Usage

You need to register the plugin first:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .build()
        .run();
}
```

Now you can import the JavaScript API package and move the window:

```javascript
import { move_window, Position } from 'tauri-plugin-positioner-api'

move_window(Position.TopRight)
```

### Rust only

If you only intend on moving the window from rust code, you can import the `Window` trait extension instead of registering the plugin:

> Note: `Window.move_window` method must be called from a different thread!

```rust
use tauri_plugin_positioner::{WindowExt, Position};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
          let mut win = app.get_window("main").unwrap();
      
          let _ = win.move_window(Position::TopRight);
          
          Ok(())
        })
        .build()
        .run();
}
```

## Contributing

PRs are welcome!

## License

[MIT © Jonas Kruckenberg](./LICENSE)
