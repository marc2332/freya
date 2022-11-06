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

### Introduction

**Freya** provides rendering for a Dioxus app using Skia, but also has some utility [hooks](/references/hooks.html), [components](/references/components.html) and theming support.

> **Dioxus** is a cross-platform UI components library for Rust, conceptually similar to React.

> **Skia** is a cross-platform graphics library, it powers some other GUI libraries, such as Flutter and even big projects, like Chromium.

Check out the examples in the Freya [repository](https://github.com/marc2332/freya/tree/main/examples) to learn more.