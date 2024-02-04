# Getting started

I encourage you to learn how [Dioxus works](https://dioxuslabs.com/learn/0.4/guide/your_first_component), when you are done you can continue here. Also make sure you have the followed the [environment setup](../setup.html) guide.

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
freya = "0.1"
dioxus = { version = "0.4", features = ["macro", "hooks"], default-features = false }
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

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            background: "rgb(0, 119, 182)",
            color: "white",
            main_align: "center",
            cross_align: "center",
            onclick: move |_| count += 1,
            font_size: "35",
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

Nice! You have created your first Freya app. 

You can learn more with the [examples](https://github.com/marc2332/freya/tree/main/examples) in the repository.