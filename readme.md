# Freya :crab:

[![Discord Server](https://img.shields.io/discord/1015005816094478347?logo=discord&style=social)](https://discord.gg/sYejxCdewG)
[![Github Sponsors](https://img.shields.io/github/sponsors/marc2332?style=social)](https://github.com/sponsors/marc2332)

A GUI library for Rust powered by [Skia](https://skia.org/) and [Dioxus](https://dioxuslabs.com). 

⚠️ It's currently work in progress and not usable for production, but you can already play with it! You can join the [Discord](https://discord.gg/sYejxCdewG) server if you have any question. 

<table>
<tr>
<td style="border:hidden;">

```rust
fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        container {
            height: "20%",
            width: "100%",
            background: "rgb(233, 196, 106)",
            padding: "25",
            color: "rgb(20, 33, 61)",
            label { 
                font_size: "20", 
                "Number is: {count}"
            }
        }
        container {
            height: "80%",
            width: "100%",
            background: "rgb(168, 218, 220)",
            color: "black",
            padding: "25",
            onclick: move |_| count += 1,
            label { "Click to increase!" }
        }
    ))
```
</td>
<td style="border:hidden;">

![Freya](./demo.png)

</td>
</table>

### Want to try it? 🤔

Windows & MacOS:

```shell
cargo run --example counter
```

Linux:

```shell
cargo run --example counter --features x11
```

### Features ✨
- Label and paragraphs
- Containers and rectangles
- Scroll views (nested too)
- Click, wheel, mouse /down/leave/over events
- Windows, Linux and MacOS support
- Components library
- Animation hook

### Goals 😁
- Fast
- Low memory usage
- Lightweight
- Secure
- Full cross platform
- Fluid animations

### Ideas 💭
- Tauri integration
- Devtools

## TO-DO 🚧
Besides all the [tracking](https://github.com/marc2332/freya/issues?q=is%3Aopen+is%3Aissue+label%3Atracking) issues, here are some of the things to do:
- [ ] Support for percentages in padding
- [ ] Add `Flex` component (using [taffy](https://github.com/dioxusLabs/taffy)).
- [ ] Documentation
- [ ] Improve Auto calculation for width and height
- [ ] Investigate if `image` diffing can be speeded up (reference: https://github.com/DioxusLabs/dioxus/pull/543#issuecomment-1238393539)
- [ ] Better touchpad support
- [ ] Move layout calculation from the layout engine into the node's state? Not sure.
- [ ] Render shadows one layer below it's element to avoid overlapping with it's siblings

[MIT License](./LICENSE.md)
