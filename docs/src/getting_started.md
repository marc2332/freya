# Getting Started

Let's start by creating a simple project:

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

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
freya = { git="https://github.com/marc2332/freya" }
dioxus = { git="https://github.com/DioxusLabs/dioxus", rev="a616a8fa9d5fe46a253e1b4bfef24abd46a623fa"}
```

### src/main.rs

```rust no_run
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