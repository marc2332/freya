# Freya 🦀

<a href="https://freyaui.dev/"><img align="right" src="logo.svg" alt="Freya logo" width="150"/></a>

[![Discord Server](https://img.shields.io/discord/1015005816094478347.svg?logo=discord&style=flat-square)](https://discord.gg/sYejxCdewG)
[![Github Sponsors](https://img.shields.io/github/sponsors/marc2332?style=social)](https://github.com/sponsors/marc2332)
[![codecov](https://codecov.io/github/marc2332/freya/branch/main/graph/badge.svg?token=APSGEC84B8)](https://codecov.io/github/marc2332/freya)

[Website](https://freyaui.dev) | [Nightly Docs](https://docs.freyaui.dev/freya) | [Stable Docs](https://docs.rs/freya/latest/freya) | [Book](https://book.freyaui.dev) | [Discord](https://discord.gg/sYejxCdewG)

**Freya** is a cross-paltform GUI library for Rust powered by 🧬 [Dioxus](https://dioxuslabs.com) and 🎨 [Skia](https://skia.org/). 

**It does not use any web tech**, check the [Differences with Dioxus](https://book.freyaui.dev/differences_with_dioxus.html). 

⚠️ It's currently work in progress, but you can already play with it! You can join the [Discord](https://discord.gg/sYejxCdewG) server if you have any question or issue. 

<br/>
<br/>

<table>
<tr>
<td style="border:hidden;">

```rust
fn app() -> Element {
    let mut count = use_signal(|| 0);

    render!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            label {
                font_size: "75",
                font_weight: "bold",
                "{count}"
            }
        }
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            Button {
                onclick: move |_| count += 1,
                label { "Increase" }
            }
            Button {
                onclick: move |_| count -= 1,
                label { "Decrease" }
            }
        }
    )
}
```
</td>
<td style="border:hidden;">

![Freya Demo](https://github.com/marc2332/freya/assets/38158676/f81a95a2-7add-4dbe-9820-3d3b6b42f6e5)

</td>
</table>

### Sponsors 🤗

Thanks to my sponsors for supporting this project! 😄

<!-- sponsors --><a href="https://github.com/piny4man"><img src="https:&#x2F;&#x2F;avatars.githubusercontent.com&#x2F;u&#x2F;8446285?u&#x3D;fd37db4dd9b4ba94dabe0bccc3a95ef2a35376ab&amp;v&#x3D;4" width="60px" alt="" /></a><!-- sponsors -->

### Want to try it? 🤔

👋 Make sure to check the [Setup guide](https://book.freyaui.dev/setup.html) first.

> ⚠️ If you happen to be on Windows using `windows-gnu` and get compile errors, maybe go check this [issue](https://github.com/marc2332/freya/issues/794).

Clone this repo and run:

```shell
cargo run --example counter
```

You can also try [`freya-template`](https://github.com/marc2332/freya-template)

### Usage 📜
Add Freya and Dioxus as dependencies:

```toml
freya = "0.2"
dioxus = { version = "0.5", features = ["macro", "hooks"], default-features = false }
```

### Features ✨
- ⛏️ Built-in **components** (button, scroll views, switch and more) 
- 🚇 Built-in **hooks** library (animations, text editing and more)
- 🔍 Built-in **devtools** panel
- 🧰 Built-in **headless runner** to test UI
- 🎨 **Theming** support (not extensible yet ⚠️)
- 🛩️ **Cross-platform** (Windows, Linux, MacOS)
- 🖼️ SKSL **Shaders** support
- 🔄️ Dioxus **Hot-reload** support
- 📒 Multi-line **text editing**
- 🦾 Basic **Accessibility** Support (experimental ⚠️)
- 🧩Compatible with dioxus-sdk and other Dioxus renderer-agnostic libraries

### Goals 😁
- Performant and low memory usage
- Good developer experience
- Cross-platform support
- Decent Accessibility support 
- Useful testing APIs
- Useful and extensible built-in components and hooks

### 🤠 Projects

[Valin](https://github.com/marc2332/valin) ⚒️ is a Work-In-Progress cross-platform code editor, made with Freya 🦀 and Rust, by me.

![Valin](https://github.com/marc2332/valin/raw/main/demo.png)

[MIT License](./LICENSE.md)
