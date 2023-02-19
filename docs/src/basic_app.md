# Basic app

Let's start by creating a simple project.

### Creating the project

```sh
mkdir freya-app
cd freya-app
cargo init
```

### Cargo.toml

```toml
[package]
name = "freya-app"
version = "0.1.0"
edition = "2021"

[dependencies]
freya = { git="https://github.com/marc2332/freya" }
dioxus = { git="https://github.com/DioxusLabs/dioxus", rev="d521da1991719760e271457dfe4f9ddf281afbb3"}
```

### src/main.rs

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
            padding: "25",
            onclick: move |_| count += 1,
            label { "Click to increase -> {count}" }
        }
    )
}
```

### Running
```sh
cargo run
```