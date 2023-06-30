# Getting started

I encourage you to learn how [Dioxus works](https://dioxuslabs.com/docs/0.3/guide/en/describing_ui/index.html), when you are done you can continue here. Also make sure you have the followed the [environment setup](../setup.html) guide.

Now, let's start by creating a hello world project.

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
freya = { git = "https://github.com/marc2332/freya" }
dioxus = { git = "https://github.com/DioxusLabs/dioxus", rev="11c9abcf7ce731ccb4a44c52de383c090ab319af", features = ["macro", "hooks"]}
```

### src/main.rs

And paste this code in your `main.rs` file.

```rust, no_run no_run
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
        rect {
            overflow: "clip",
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
