# Hello, World!

Let's start by creating a hello world project.

### Creating the project

```sh
mkdir freya-app
cd freya-app
cargo init
```

### Cargo.toml

Make sure to add Freya and Dioxus as dependencies:

```toml
[package]
name = "freya-app"
version = "0.1.0"
edition = "2021"

[dependencies]
freya = { git="https://github.com/marc2332/freya" }
dioxus = { git="https://github.com/DioxusLabs/dioxus", rev="49c5a5043a16fc82210af146c345793dd448e519"}
```

### src/main.rs

And paste this code in your `main.rs` file.

```rust no_run
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
        container {
            height: "100%",
            width: "100%",
            background: "rgb(35, 35, 35)",
            color: "white",
            padding: "12",
            onclick: move |_| count += 1,
            label { "Click to increase -> {count}" }
        }
    )
}
```

### Running
Simply run with `cargo`:

```sh
cargo run
```