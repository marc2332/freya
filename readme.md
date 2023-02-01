# Freya :crab:

<img align="right" src="logo.svg" alt="Freya logo" width="150"/>

[![Discord Server](https://img.shields.io/discord/1015005816094478347.svg?logo=discord&style=flat-square)](https://discord.gg/sYejxCdewG)
[![Github Sponsors](https://img.shields.io/github/sponsors/marc2332?style=social)](https://github.com/sponsors/marc2332)
[![codecov](https://codecov.io/github/marc2332/freya/branch/main/graph/badge.svg?token=APSGEC84B8)](https://codecov.io/github/marc2332/freya)

**Freya** is native GUI library for Rust powered by 🧬 [Dioxus](https://dioxuslabs.com) and 🎨 [Skia](https://skia.org/). 

⚠️ It's currently work in progress and not usable for production, but you can already play with it! You can join the [Discord](https://discord.gg/sYejxCdewG) server if you have any question or issue. 

<br/>
<br/>

<table>
<tr>
<td style="border:hidden;">

```rust
fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
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
    )
}
```
</td>
<td style="border:hidden;">

![Freya](./demo.png)

</td>
</table>

### Want to try it? 🤔

**⚠️ Note for Windows**: You need the **latest** (this is very important) Visual Studio 2022.

Run:

```shell
cargo run --example counter
```

### Usage 📜
Add Freya as a dependency in your `Cargo.toml`:

```toml
freya = { git = "https://github.com/marc2332/freya" }
dioxus = { git = "https://github.com/DioxusLabs/dioxus", rev="4f83c1b86d396a24b32da6229b73e95a05855db3"}
```

### Features ✨
- Text
- Containers
- Scroll views (nested too)
- Events: click, wheel, mouse /down/leave/over for now
- Support for Windows, Linux and MacOS
- Optional Components library (buttons, switch, etc)
- Animation hook utility
- SVG Support
- Devtools / Wireframe inspector
- Headless testing

### Goals 😁
- Easy to use
- Fast
- Low memory usage
- Lightweight
- Secure
- Full cross platform
- Fluid animations

### Ideas 💭
- Tauri integration

## TO-DO 🚧
Besides all the [tracking](https://github.com/marc2332/freya/issues?q=is%3Aopen+is%3Aissue+label%3Atracking) issues, here are some of the things to do:
- [ ] Support for percentages in padding
- [ ] Add `flex` support.
- [ ] Documentation
- [ ] Improve Auto calculation for width and height
- [ ] Investigate if `image` diffing can be speeded up ([reference](https://github.com/DioxusLabs/dioxus/pull/543#issuecomment-1238393539))

[MIT License](./LICENSE.md)
