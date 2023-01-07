# Introduction

<img align="right" src="./logo.svg" alt="Freya logo" width="120"/>

**Freya** is native GUI library for 🦀 Rust, powered 🧬 [Dioxus](https://dioxuslabs.com) and 🎨 [Skia](https://skia.org/). 

⚠️ It's currently work in progress and not usable for production, but you can already play with it! 

You can join the [Discord](https://discord.gg/sYejxCdewG) server if you have any question or issue. 


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
**Freya** is built on top of Dioxus `VirtualDOM` and several other APIs, it provides a renderer powered by a handmade layout engine and Skia, alongside a set of basic elements, components and hooks.
> **Dioxus** is a cross-platform UI components library for Rust, a *rustified* React.

> **Skia** is a cross-platform graphics library, it powers some other GUI libraries, such as Flutter and even big projects, like the Blink engine.