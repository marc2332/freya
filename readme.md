# trev 🧩

[![Discord Server](https://img.shields.io/discord/1015005816094478347?logo=discord&style=social)](https://discord.gg/sYejxCdewG)

A GUI library based on [Skia](https://skia.org/) and [Dioxus](https://dioxuslabs.com).

> trev is like react-dom, where react is dioxus and the dom is skia.

```rust

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        view {
            height: "20%",
            width: "100%",
            background: "black",
            padding: "25",
            text { "Number is: {count}" }
        }
        view {
            height: "80%",
            width: "100%",
            background: "blue",
            padding: "25",
            onclick: move |_| count += 1,
            text { "Increase!" }
        }
    ))
}
```
### Features ✨
- Text
- Containers and views
- Nested scroll views
- Click, mouse move, mouse scrolled events
- Background, text color, padding, width, height, shadow, border radius, custom layer (like z-index)
- Windows & Linux (MacOS not tested yet)

### Goals 😁
- Fast, lightweight and secure apps
- Full cross platform

### Ideas 💭
- Tauri integration
- Browser-like devtools

## TO-DO 🚧
- [ ] Support for percentages in padding
- [x] rgb(r,g,b) syntax support
- [ ] Add window parameters
- [ ] Support for more events
- [ ] Support for multiple windows
- [ ] Use [taffy](https://github.com/dioxusLabs/taffy) for Flex layouts.
- [x] Renderer-based bounds clipping
- [ ] Improve code and documentation


MIT License