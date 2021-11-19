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

A plugin for tauri that helps positioning you windows at well known locations.

## Install

### Rust

```toml
[dependencies]
tauri-plugin-positioner = "0.1"
```

### JavaScript

```
npm install github:JonasKruckenberg/tauri-plugin-positioner
# or
yarn add github:JonasKruckenberg/tauri-plugin-positioner
```

## Usage

You need to register the plugin first:

```rust
use tauri-plugin-positioner::{Positioner, Position};

fn main() {
    tauri::Builder::default()
        .plugin(Positioner::default())
        .build()
        .run();
}
```

Now you can import the JavaScript API package and move to window:

```javascript
import { move_window, Position } from 'tauri-plugin-positioner-api'

move_window(Position.TopRight)
```

### Rust only

If you only intend on moving the window from rust code, you can just import the `Window` trait extension instead of registering the plugin:

> Note: `Window.move_window` method must be called from a different thread!

```rust
use tauri-plugin-positioner::{WindowExt, Position};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
          let mut win = app.get_window("main").unwrap();
      
          // Call this method from a different thread
          tauri::async_runtime::spawn(async move {
            let _ = win.move_window(Position::TopRight);
          });
          
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
