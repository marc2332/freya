# Introduction

<img align="right" src="./logo.svg" alt="Freya logo" width="110"/>

**Freya** is native GUI library built on top of 🧬 [Dioxus](https://dioxuslabs.com) and powered by 🎨 [Skia](https://skia.org/), for 🦀 Rust. 

⚠️ It's currently work in progress and not usable for production, but you can already play with it! 

You can join the [Discord](https://discord.gg/sYejxCdewG) server if you have any question or issue. 

> You can also see the [API Reference](https://docs.rs/freya/latest/freya/).

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
            padding: "25",
            onclick: move |_| count += 1,
            label { "Click to increase -> {count}" }
        }
    )
}
```

Check out the examples in the Freya [repository](https://github.com/marc2332/freya/tree/main/examples) to learn more.

### About
**Freya** is built on top of Dioxus, it provides a renderer powered by Skia, alongside a set of elements, components, hooks and testing utilities.

### Why 🧬 Dioxus?

Dioxus is heavily influenced by React, resulting in a streamlined process for creating complex components without the need for excessive code. 

This sets it apart from other Rust libraries, where equivalent components often require a significant amount of additional code.

### Why 🎨 Skia?

Skia is a battle-tested and well maintained graphics library, and there are even some rusty [bindings](https://github.com/rust-skia/rust-skia). 