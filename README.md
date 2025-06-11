# recaret

`recaret` is a cross-platform Rust crate providing asynchronous utilities to
manage multiple carets and text selections across any application. Global mouse
and keyboard events are captured to mimic VS Code's multi-caret behaviors.

## Building

```bash
# build library
cargo build

# build examples
cargo build --examples
```

## Running examples
Example binaries are provided for Linux, macOS and Windows. They simply start
the global listener and print caret data.

```bash
cargo run --example linux   # on Linux
cargo run --example macos   # on macOS
cargo run --example windows # on Windows
```

The process needs accessibility permissions on macOS and appropriate privileges
on Windows and Linux.

## Library usage

```rust
use recaret::{add_caret, list_carets, start_listener};

#[tokio::main]
async fn main() {
    // start listening to events in background
    tokio::spawn(async { start_listener().await.unwrap(); });

    // manually add a caret
    let id = add_caret(10, 10).await;
    println!("carets: {:?}", list_carets().await);
}
```

## Platform notes
- **Windows**: requires running from a terminal with appropriate privileges.
- **macOS**: grant the binary accessibility permissions in System Preferences.
- **Linux**: works on X11 and Wayland when permissions allow reading input
devices.

## License

This project is licensed under the terms of the MIT license.
