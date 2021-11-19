# Tauri Plugin positioner

A plugin for tauri that helps positioning you windows at well known locations.

## Install

### Rust

```toml
[dependencies]
tauri-plugin-positioner = { git = "https://github.com/JonasKruckenberg/tauri-plugin-positioner" }
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

```js
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
