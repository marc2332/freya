⚠️⚠️⚠️ **I am currently rewriting Freya, you can follow the progress in https://github.com/marc2332/freya/pull/1351** ⚠️⚠️⚠️

# Freya 🦀

<a href="https://freyaui.dev/"><img align="right" src="logo.svg" alt="Freya logo" width="150"/></a>

[![Discord Server](https://img.shields.io/discord/1015005816094478347.svg?logo=discord&style=flat-square)](https://discord.gg/sYejxCdewG)
[![Github Sponsors](https://img.shields.io/github/sponsors/marc2332?style=social)](https://github.com/sponsors/marc2332)
[![codecov](https://codecov.io/github/marc2332/freya/branch/main/graph/badge.svg?token=APSGEC84B8)](https://codecov.io/github/marc2332/freya)

[Website](https://freyaui.dev) | [Documentation](https://docs.rs/freya/0.3/freya) | [Discord](https://discord.gg/sYejxCdewG)

**Freya** is a **cross-platform and non-web** GUI library for Rust powered by 🎨 [Skia](https://skia.org/).

- [Introduction](https://docs.rs/freya/0.3/freya/_docs/introduction/index.html)
- [Development Setup](https://docs.rs/freya/0.3/freya/_docs/development_setup/index.html)
- [Documentation](https://docs.rs/freya/0.3/freya)
- [Differences with Dioxus](#differences-with-dioxus)
- [Contributing](#contributing-%EF%B8%8F)
- [Support Development](#support-)

#### Counter example
<table>
<tr>
<td style="border:hidden;">

```rust
fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
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

#### Website example

This is Freya's website landing page made with Freya itself:

[Source code](/examples/website.rs).

![Website made with Rust Screenshot](/website/public/blog/0.3/website.png)

#### Animation example

An animated drag and drop.

[Source code](/examples/drag_drop.rs).

![drag](https://github.com/user-attachments/assets/32d6bd50-32b9-4159-b669-f10de03ed17b)


#### Animated router example

Animated transition between router pages.

[Source code](/examples/animated_tabs.rs).

![animated_router](https://github.com/user-attachments/assets/9218e053-1cc0-48ca-a7c3-8f09b2281b92)

<details>
  <summary>More Examples</summary>

#### Valin Code Editor

[Valin](https://github.com/marc2332/valin) ⚒️ is a Work-In-Progress cross-platform code editor, made with Freya 🦀 and Rust, by me.

![Valin](https://github.com/marc2332/valin/raw/main/demo.png)

#### Switch Theme example

[Source code](/examples/switch_theme.rs).

![Switch Theme Screenshot](/website/public/blog/0.3/refreshed_components.png)

#### Todo example

[Source code](/examples/todo.rs).

![todo](https://github.com/user-attachments/assets/e450f514-ad10-4fb5-8b35-909c51d5d539)

#### Resizable containers example

[Source code](/examples/resizable_containers.rs).

![resizable](https://github.com/user-attachments/assets/3d3f4718-a0d6-4e4d-a0ad-0c750fb0c67e)


</details>

### Want to try it? 🤔

Make sure to have [Development Setup](https://docs.rs/freya/0.3/freya/_docs/development_setup/index.html) ready.

> ⚠️ If you happen to be on Windows using `windows-gnu` and get compile errors, maybe go check this [issue](https://github.com/marc2332/freya/issues/794).

Clone this repo and run:

```shell
cargo run --example counter
```

You can also try [`freya-template`](https://github.com/marc2332/freya-template)

### Usage 📜
Add Freya and Dioxus as dependencies:

```toml
freya = "0.3"
dioxus = { version = "0.6", features = ["macro", "hooks"], default-features = false }
```
### Contributing 🧙‍♂️

If you are interested in contributing please make sure to have read the [Contributing](CONTRIBUTING.md) guide first!

### Features ✨
- ⛏️ Built-in **components** (button, scroll views, switch and more) 
- 🚇 Built-in **hooks** (animations, text editing and more)
- 🔍 Built-in **developer tools** (tree inspection, fps overlay)
- 🧰 Built-in **headless runner** to test UI
- 🎨 **Theming** support
- 🛩️ **Cross-platform** (Windows, Linux, MacOS)
- 🖼️ SKSL **Shaders** support
- 📒 Multi-line **text editing**
- 🦾 **Accessibility** support
- 🧩 Compatible with dioxus-sdk and other Dioxus renderer-agnostic libraries

### Goals 😁
- Performant and low memory usage
- Good developer experience
- Cross-platform support
- Decent Accessibility support 
- Useful testing APIs
- Useful and extensible built-in components and hooks

### Differences with Dioxus

**Freya** uses some of the **core** crates from Dioxus. This means that you will effectively be creating Dioxus components using RSX and hooks.

However, thanks to Dioxus being a renderer-agnostic library, you and your app will **NOT** be using Webviews, JavaScript, HTML, CSS, or any other abstraction that ends up using one of those or other web technologies.

Freya does everything on its own when it comes to:
- Elements
- Styling
- Layout
- Events
- Rendering
- Testing
- Built-in components and hooks
- Editing
- Animating

...and more. Dioxus is only used for managing app components (hooks, lifecycle, state, RSX), while **everything else is managed by Freya**.

**Freya is not meant to be a drop-in alternative to Dioxus renderers but a GUI library on its own.**

Below is a comparison of the main differences between Freya and the official Dioxus renderers for Desktop (WebView and Blitz):

| Category                             | Freya            | Dioxus Renderers                |
|--------------------------------------|------------------|---------------------------------|
| **Elements, attributes, and events** | Custom           | HTML                            |
| **Layout** | Custom ([`Torin`](https://github.com/marc2332/freya/tree/main/crates/torin)) | CSS or [`Taffy`](https://github.com/DioxusLabs/taffy) |
| **Styling**                          | Custom                    | CSS                             |
| **Renderer**                         | Skia                      | WebView or WGPU                 |
| **Components library**               | Custom ([`freya-components`](https://github.com/marc2332/freya/tree/main/crates/components)) | None, but can use HTML elements and CSS libraries |
| **Devtools**                         | Custom ([`freya-devtools`](https://github.com/marc2332/freya/tree/main/crates/devtools))   | Provided in WebView              |
| **Headless testing runner**          | Custom ([`freya-testing`](https://github.com/marc2332/freya/tree/main/crates/testing))       | None, but tools like Playwright and similar are available |


### Support 🤗

If you are interested in supporting the development of this project feel free to donate to my [Github Sponsor](https://github.com/sponsors/marc2332/) page.

Thanks to my sponsors for supporting this project! 😄 

<!-- sponsors --><a href="https://github.com/piny4man"><img src="https:&#x2F;&#x2F;github.com&#x2F;piny4man.png" width="60px" alt="User avatar: " /></a><a href="https://github.com/gqf2008"><img src="https:&#x2F;&#x2F;github.com&#x2F;gqf2008.png" width="60px" alt="User avatar: 高庆丰" /></a><a href="https://github.com/lino-levan"><img src="https:&#x2F;&#x2F;github.com&#x2F;lino-levan.png" width="60px" alt="User avatar: Lino Le Van" /></a><a href="https://github.com/d3rpp"><img src="https:&#x2F;&#x2F;github.com&#x2F;d3rpp.png" width="60px" alt="User avatar: Huddy Buddy" /></a><a href="https://github.com/DrigsterI"><img src="https:&#x2F;&#x2F;github.com&#x2F;DrigsterI.png" width="60px" alt="User avatar: Gabriel Jõe" /></a><a href="https://github.com/markalexander"><img src="https:&#x2F;&#x2F;github.com&#x2F;markalexander.png" width="60px" alt="User avatar: Mark" /></a><!-- sponsors -->

### Special thanks 💪

- [Jonathan Kelley](https://github.com/jkelleyrtp) and [Evan Almloff](https://github.com/ealmloff) for making [Dioxus](https://dioxuslabs.com/) and all their help, specially when I was still creating Freya.
- [Armin](https://github.com/pragmatrix) for making [rust-skia](https://github.com/rust-skia/rust-skia/) and all his help and making the favor of hosting prebuilt binaries of skia for the combo of features use by Freya.
- [geom3trik](https://github.com/geom3trik) for helping me figure out how to add incremental rendering.
- [Tropical](https://github.com/Tropix126) for this contributions to improving accessibility and rendering.
- [Aiving](https://github.com/Aiving) for having made heavy contributions to [rust-skia](https://github.com/rust-skia/rust-skia/) for better SVG support, and helped optimizing images rendering in Freya.
- [RobertasJ](https://github.com/RobertasJ) for having added nested parenthesis to the `calc()` function and also pushed for improvements in the animation APIs.
- And to the rest of contributors and anybody who gave me any kind of feedback!

### License

[MIT License](./LICENSE.md)
