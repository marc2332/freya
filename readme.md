# trev 🧩

[![Discord Server](https://img.shields.io/discord/1015005816094478347?logo=discord&style=social)](https://discord.gg/sYejxCdewG)

A GUI library based on [Skia](https://skia.org/) and [Dioxus](https://dioxuslabs.com).

> trev is like react-dom, where react is dioxus and the dom is skia. But it also comes with an optional but handy components library. 

```rust

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        rect {
            height: "20%",
            width: "100%",
            background: "black",
            padding: "25",
            label { "Number is: {count}" }
        }
        rect {
            height: "80%",
            width: "100%",
            background: "blue",
            padding: "25",
            onclick: move |_| count += 1,
            label { "Increase!" }
        }
    ))
}
```
### Features ✨
- Text
- Containers and views
- Nested scroll views
- Click, mouse move, scroll events
- Background, text color, padding, width, height, shadow, border radius, custom layer (like z-index)
- Windows, Linux and MacOS support

### Goals 😁
- Fast
- Lightweight
- Secure
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
- [ ] Add `Flex` component (using [taffy](https://github.com/dioxusLabs/taffy)).
- [x] Renderer-based bounds clipping
- [ ] Documentation
- [ ] Improve Auto calculation for width and height
- [ ] Investigate if `image` diffing can be speeded up (reference: https://github.com/DioxusLabs/dioxus/pull/543#issuecomment-1238393539)
- [ ] `FilesystemImage` and `NetworkImage` components
- [x] Add `paragraph` element
- [x] Rename `view` element to `rect`
- [x] Rename `text` element to `label`
- [ ] Better touchpad support

MIT License