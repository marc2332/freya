# trev 🧩

A GUI library based on [Skia](https://skia.org/) and [Dioxus](https://dioxuslabs.com).

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
- [ ] rgb(r,g,b) syntax support
- [ ] Add window parameters
- [ ] Support for more events
- [ ] Support for multiple windows
- [ ] Use [taffy](https://github.com/dioxusLabs/taffy) for Flex layouts.
- [x] Renderer-based bounds clipping
- [ ] Improve code and documentation


MIT License