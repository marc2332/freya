# Getting Started

**Freya** is a native GUI toolkit written in [Rust](https://www.rust-lang.org/), uses [Skia](https://skia.org/) as renderer and [Dioxus](https://dioxuslabs.com/) as components Library.

```rust
fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

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