⚠️⚠️⚠️ **I am currently rewriting Freya, you can follow the progress in https://github.com/marc2332/freya/pull/1351** ⚠️⚠️⚠️

# Freya 🦀

<a href="https://freyaui.dev/"><img align="right" src="logo.svg" alt="Freya logo" width="150"/></a>

[![Discord Server](https://img.shields.io/discord/1015005816094478347.svg?logo=discord&style=flat-square)](https://discord.gg/sYejxCdewG)
[![Github Sponsors](https://img.shields.io/github/sponsors/marc2332?style=social)](https://github.com/sponsors/marc2332)
[![codecov](https://codecov.io/github/marc2332/freya/branch/main/graph/badge.svg?token=APSGEC84B8)](https://codecov.io/github/marc2332/freya)

[Website](https://freyaui.dev) | [Documentation](https://docs.rs/freya/0.3/freya) | [Discord](https://discord.gg/sYejxCdewG) | [Contact](#contact)

**Freya** is a **cross-platform and non-web** GUI library for Rust powered by 🎨 [Skia](https://skia.org/).

#### Counter example
<table>
<tr>
<td style="border:hidden;">

```rust
fn app() -> Element {
    let mut count = use_state(|| 4);

    rect()
        .child(
            rect()
                .width(Size::fill())
                .height(Size::percent(50.))
                .center()
                .color((255, 255, 255))
                .background((15, 163, 242))
                .font_size(75.)
                .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
                .child(count.read().to_string()),
        )
        .child(
            rect()
                .horizontal()
                .width(Size::fill())
                .height(Size::percent(50.))
                .center()
                .spacing(8.0)
                .child(
                    Button::new()
                        .on_press(move |_| {
                            *count.write() += 1;
                        })
                        .child("Increase"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| {
                            *count.write() -= 1;
                        })
                        .child("Decrease"),
                ),
        )
        .into()
}
```
</td>
<td style="border:hidden;">

![Freya Demo](https://github.com/user-attachments/assets/695e3ae9-8914-4354-ac9e-5c53b1cd7442)

</td>
</table>


</details>

### Features ✨
- **Component Model**: Separate UI pieces by turning them into reusable components that. Each component takes some **Input**, might manage some inner **State** and ultimately returns a **UI**.
- **Headless Testing**: Easily test your UI logic in a headless environment, supports all the normal features. In fact most of the internal components and features are tested with it.
- **i18n (Language Translation)**: Easily translate your apps with `freya-i18` using the Fluent language.
- **Shaders**: Render from simple to complex shaders using the Skia Shaders language (SlSl).
- **a11y (Accessibility)**: You can make your UI elements accessible by using the a11y attributes.
- **Built-in Components**: Freya comes with a set of ready-to-use components, these include ScrollViews, VirtualScrollViews, Buttons, Switch, Slider, etc.
- **Animations**: Easily animate your UI whether its a size or a color. You have full control over its behavior.
- **Text Editing**: Freya supports from simple to rich text editing. You can even make cloned editors, virtualized editors, etc.
- **Cross-platform**: Your app will render and behave the same in Windows, Linux and MacOS.
- **Efficient Global State**: Manage your app state efficiently using `freya-radio`.
- **Icons**: Easily add icons to your app using `freya-icons`, currently only supports Lucide.
- **Devtools**: Inspect your app UI tree to to debug, or see performance state.
- **Routing**: Manage your app UI in separate routes using `freya-router`.

### Trying it out

Make sure to have [Development Setup](https://docs.rs/freya/0.3/freya/_docs/development_setup/index.html) ready.

> ⚠️ If you happen to be on Windows using `windows-gnu` and get compile errors, maybe go check this [issue](https://github.com/marc2332/freya/issues/794).

Clone this repo and run:

```shell
cargo run --example counter
```

You can also try [`freya-template`](https://github.com/marc2332/freya-template)

### Usage 📜
Add **Freya** as dependency:

```toml
freya = "0.4"
```
### Contributing 🧙‍♂️

If you are interested in contributing please make sure to have read the [Contributing](CONTRIBUTING.md) guide first!


### Contact 
You may contact me for questions, collaboration or anything that comes to your mind at [marc@mespin.me](mailto:marc@mespin.me).

### Support 🤗

If you are interested in supporting the development of this project feel free to donate to my [Github Sponsor](https://github.com/sponsors/marc2332/) page.

Thanks to my sponsors for supporting this project! 😄 

<!-- sponsors --><a href="https://github.com/piny4man"><img src="https:&#x2F;&#x2F;github.com&#x2F;piny4man.png" width="60px" alt="User avatar: " /></a><a href="https://github.com/gqf2008"><img src="https:&#x2F;&#x2F;github.com&#x2F;gqf2008.png" width="60px" alt="User avatar: 高庆丰" /></a><a href="https://github.com/lino-levan"><img src="https:&#x2F;&#x2F;github.com&#x2F;lino-levan.png" width="60px" alt="User avatar: Lino Le Van" /></a><!-- sponsors -->

### Special thanks 💪

- [Jonathan Kelley](https://github.com/jkelleyrtp) and [Evan Almloff](https://github.com/ealmloff) for making [Dioxus](https://dioxuslabs.com/) and all their help, specially when I was still creating Freya.
- [Armin](https://github.com/pragmatrix) for making [rust-skia](https://github.com/rust-skia/rust-skia/) and all his help and making the favor of hosting prebuilt binaries of skia for the combo of features use by Freya.
- [geom3trik](https://github.com/geom3trik) for helping me figure out how to add incremental rendering.
- [Tropical](https://github.com/Tropix126) for this contributions to improving accessibility and rendering.
- [Aiving](https://github.com/Aiving) for having made heavy contributions to [rust-skia](https://github.com/rust-skia/rust-skia/) for better SVG support, and helped optimizing images rendering in Freya.
- [RobertasJ](https://github.com/RobertasJ) for having added nested parenthesis to the `calc()` function and also pushed for improvements in the animation APIs.
- And to the rest of contributors and anybody who gave me any kind of feedback!

### History with Dioxus
Freya 0.1, 0.2 and 0.3 were based on the core crates of Dioxus. Starting 0.4 Freya does no longer use Dioxus, instead it uses its own reactive core, partially inspired by Dioxus but yet with lots of differences.

### License

[MIT License](./LICENSE.md)
