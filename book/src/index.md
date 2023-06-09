# Introduction

**Freya** is native GUI library built on top of 🧬 [Dioxus](https://dioxuslabs.com) and powered by 🎨 [Skia](https://skia.org/), for 🦀 Rust. 

You can check the check [API References](https://docs.freyaui.dev/freya/) or join the [Discord](https://discord.gg/sYejxCdewG) server if you have any questions or issues. 

> It's currently work in progress and not usable for production, but you can already play with it! 

Example
<br>

```rust no_run
fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
        container {
            height: "100%",
            width: "100%",
            background: "rgb(35, 35, 35)",
            color: "white",
            padding: "12.5",
            onclick: move |_| count += 1,
            label { "Click to increase -> {count}" }
        }
    )
}
```

Check out the examples in the Freya [repository](https://github.com/marc2332/freya/tree/main/examples) to learn more.

### About
**Freya** is built on top of Dioxus. It provides a renderer powered by Skia alongside a set of elements, components, hooks and testing utilities.

### Why 🧬 Dioxus?

Dioxus is a React-like library for Rust. Its component and hooks model make it simple to use and scales to complex apps.

### Why 🎨 Skia?

Skia is a battle-tested and well-maintained graphics library, and there are even some rusty [bindings](https://github.com/rust-skia/rust-skia). 
