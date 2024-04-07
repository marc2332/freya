# Overview

**Freya** is a **work in progress** cross-platform native GUI library for 🦀 Rust, built on top of 🧬 [Dioxus](https://dioxuslabs.com) and 🎨 [Skia](https://skia.org/) as a graphics library. 


<table>
<tr>
<td style="border:hidden; padding: 0;">

```rust, no_run
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
<td style="border:hidden; padding: 0;">
<video width="400" loop autoplay>
  <source src="https://freya--feat-website-enhancements.deno.dev/demo.mp4" type="video/mp4" />
</video>
</td>
</table>

Check out the examples in the Freya [repository](https://github.com/marc2332/freya/tree/main/examples) to learn more.


### Features
- ⛏️ Built-in **components** (button, scroll views, switch and more) 
- 🚇 Built-in **hooks** library (animations, text editing and more)
- 🔍 Built-in **devtools** panel (experimental ⚠️)
- 🧰 Built-in **headless testing** runner for components
- 🎨 **Theming** support (not extensible yet ⚠️)
- 🛩️ Cross-platform (Windows, Linux, MacOS)
- 🖼️ SKSL **Shaders** support
- 🔄️ Dioxus **Hot-reload** support
- 📒 Multi-line **text editing** (experimental ⚠️)
- 🦾 Basic **Accessibility** Support (experimental ⚠️)
- 🧩 Compatible with Dioxus SDK and other Dioxus renderer-agnostic libraries

### Learn More

- [Setup](./setup.html)
- [API References](https://docs.rs/freya/latest/freya/)
- [Main differences with Dioxus](./differences_with_dioxus.html)
- [Discord](https://discord.gg/sYejxCdewG)

