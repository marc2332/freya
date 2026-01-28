# Freya 🦀

<a href="https://freyaui.dev/"><img align="right" src="logo.svg" alt="Freya logo" width="150"/></a>

[![Discord Server](https://img.shields.io/discord/1015005816094478347.svg?logo=discord&style=flat-square)](https://discord.gg/sYejxCdewG)
[![Github Sponsors](https://img.shields.io/github/sponsors/marc2332?style=social)](https://github.com/sponsors/marc2332)
[![codecov](https://codecov.io/github/marc2332/freya/branch/main/graph/badge.svg?token=APSGEC84B8)](https://codecov.io/github/marc2332/freya)

[Website](https://freyaui.dev) | [Stable Documentation](https://docs.rs/freya/) | [Discord](https://discord.gg/sYejxCdewG) | [Contact](#contact)

**Freya** is a **cross-platform, native, declarative** GUI library for Rust 🦀.

> :warning: I recently rewrote a huge percentage of Freya in https://github.com/marc2332/freya/pull/1351, so the `main` branch differs a lot from the latest stable release.

## Feature Showcase ✨

### Component Model & Reactive State

Freya’s component model lets you create reusable UI elements that automatically re-render when the state they depend on changes. Components can hold their own internal state or subscribe to shared state, and they produce UI as their output. Any type that implements the `Component` trait can be a component, while the root (`app`) component can simply be a function. Built-in examples include components like `Button` and `Switch`.

<table>
<tr>
<td style="border:hidden;">

```rust
fn app() -> impl IntoElement {
    let mut count = use_state(|| 4);

    let counter = rect()
        .width(Size::fill())
        .height(Size::percent(50.))
        .center()
        .color((255, 255, 255))
        .background((15, 163, 242))
        .font_weight(FontWeight::BOLD)
        .font_size(75.)
        .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
        .child(count.read().to_string());

    let actions = rect()
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
        );

    rect().child(counter).child(actions)
}
```
</td>
<td style="border:hidden;">

<img src="https://github.com/user-attachments/assets/6f13c144-9557-4518-9b2e-93706568f355">

</td>
</table>


### Out of the box components

Freya comes with a set of components out of the box, from simple like `Button`, `Switch`, `Slider` to more complex like `VirtualScrollView`, `Calendar`, `ColorPicker`, etc.

You can check all the examples that start with `component_` in the [examples folder](https://github.com/marc2332/freya/blob/main/examples/).

Example of [`component_input.rs`](https://github.com/marc2332/freya/blob/main/examples/component_input.rs):
<div align="center">
  <img src="https://github.com/user-attachments/assets/3dd1b7bf-0c9e-4257-a308-f2e3ca057ff1">
</div>

### Smooth Animations

Create transitions for colors, sizes, positions, and other visual properties. The animation API gives you full control over timing, easing functions, and animation sequences.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;

fn app() -> impl IntoElement {
    let mut animation = use_animation(|_| AnimColor::new((246, 240, 240), (205, 86, 86)).time(400));

    rect()
        .background(&*animation.read())
        .expanded()
        .center()
        .spacing(8.0)
        .child(
            Button::new()
                .on_press(move |_| {
                    animation.start();
                })
                .child("Start"),
        )
        .child(
            Button::new()
                .on_press(move |_| {
                    animation.reverse();
                })
                .child("Reverse"),
        )
}
```

</details>

<div align="center">
  <img src="https://github.com/user-attachments/assets/ab0c4637-7c04-4dd3-8c6f-c841d4e163d7">
</div>

[Portal example](https://github.com/marc2332/freya/blob/main/examples/animation_portal.rs)

[Component Portal](https://github.com/user-attachments/assets/720dd8ec-2a76-4f80-8787-25b3ebb06611)

### Rich Text Editing

Freya provides text editing capabilities that go beyond simple input fields. You can create rich text editors with cursor management, text selection, keyboard shortcuts, custom formatting, virtulization and more.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;

fn app() -> impl IntoElement {
    let holder = use_state(ParagraphHolder::default);
    let mut editable = use_editable(|| "Hello, World!".to_string(), EditableConfig::new);
    let focus = use_focus();

    paragraph()
        .a11y_id(focus.a11y_id())
        .cursor_index(editable.editor().read().cursor_pos())
        .highlights(
            editable
                .editor()
                .read()
                .get_selection()
                .map(|selection| vec![selection])
                .unwrap_or_default(),
        )
        .on_mouse_down(move |e: Event<MouseEventData>| {
            focus.request_focus();
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        })
        .on_mouse_move(move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Move {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        })
        .on_global_mouse_up(move |_| editable.process_event(EditableEvent::Release))
        .on_key_down(move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        })
        .on_key_up(move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyUp { key: &e.key });
        })
        .span(editable.editor().read().to_string())
        .holder(holder.read().clone())
}
```

</details>

<div align="center">
  <img src="https://github.com/user-attachments/assets/27fbbcab-3ab4-4dc7-b984-47cec280123c">
</div>


### Routing & Navigation

Define routes, manage navigation state, and transition between different views.
Enable with the `router` feature.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;
use freya::router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppBottomBar)]
        #[route("/")]
        Home,
        #[route("/settings")]
        Settings,
}

fn app() -> impl IntoElement {
    router::<Route>(|| RouterConfig::default().with_initial_path(Route::Settings))
}


#[derive(PartialEq)]
struct AppBottomBar;
impl Component for AppBottomBar {
    fn render(&self) -> impl IntoElement {
        NativeRouter::new().child(
            rect()
                .content(Content::flex())
                .child(
                    rect()
                        .width(Size::fill())
                        .height(Size::flex(1.))
                        .center()
                        .child(outlet::<Route>()),
                )
                .child(
                    rect()
                        .horizontal()
                        .width(Size::fill())
                        .main_align(Alignment::center())
                        .padding(8.)
                        .spacing(8.)
                        .child(
                            Link::new(Route::Home)
                                .child(FloatingTab::new().child("Home"))
                                .activable_route(Route::Home)
                                .exact(true),
                        )
                        .child(
                            Link::new(Route::Settings)
                                .child(FloatingTab::new().child("Settings"))
                                .activable_route(Route::Settings)
                                .exact(true),
                        ),
                ),
        )
    }
}

#[derive(PartialEq)]
struct Home {}
impl Component for Home {
    fn render(&self) -> impl IntoElement {
        Button::new()
            .on_press(|_| {
                RouterContext::get().replace(Route::Settings);
            })
            .child("Go Settings")
    }
}

#[derive(PartialEq)]
struct Settings {}
impl Component for Settings {
    fn render(&self) -> impl IntoElement {
        Button::new()
            .on_press(|_| {
                RouterContext::get().replace(Route::Home);
            })
            .child("Go Home")
    }
}
```

</details>

<div align="center">
  <img src="https://github.com/user-attachments/assets/1b3ed156-7e29-43f7-a11c-455ba0520223">
</div>


### Global State Management

Freya's `freya-radio` state management system provides efficient global state management through a channels system. Components subscribe to specific "channels" and only receive updates when data is mutated and notified through their channel.
Enable with the `radio` feature.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;
use freya::radio::*;

#[derive(Default)]
struct Data {
    pub lists: Vec<Vec<String>>,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum DataChannel {
    ListCreation,
    SpecificListItemUpdate(usize),
}

impl RadioChannel<Data> for DataChannel {}

fn app() -> impl IntoElement {
    use_init_radio_station::<Data, DataChannel>(Data::default);
    let mut radio = use_radio::<Data, DataChannel>(DataChannel::ListCreation);

    let on_press = move |_| {
        radio.write().lists.push(Vec::default());
    };

    rect()
        .horizontal()
        .child(Button::new().on_press(on_press).child("Add new list"))
        .children(
            radio
                .read()
                .lists
                .iter()
                .enumerate()
                .map(|(list_n, _)| ListComp(list_n).into()),
        )
}


#[derive(PartialEq)]
struct ListComp(usize);
impl Component for ListComp {
    fn render(&self) -> impl IntoElement {
        let list_n = self.0;
        let mut radio = use_radio::<Data, DataChannel>(DataChannel::SpecificListItemUpdate(list_n));

        println!("Running DataChannel::SpecificListItemUpdate({list_n})");

        rect()
            .child(
                Button::new()
                    .on_press(move |_| radio.write().lists[list_n].push("Hello, World".to_string()))
                    .child("New Item"),
            )
            .children(
                radio.read().lists[list_n]
                    .iter()
                    .enumerate()
                    .map(move |(i, item)| label().key(i).text(item.clone()).into()),
            )
    }
}

```

</details>


### Icon Library

Easily integrate icons into your applications, only supports Lucide at the moment.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;
use freya::icons;

fn app() -> impl IntoElement {
    svg(icons::lucide::antenna())
        .color((120, 50, 255))
        .expanded()
}
```

</details>

<div align="center">
  <img src="https://github.com/user-attachments/assets/ee561203-d4d2-4283-8dd2-9df61f04f86f">
</div>


### Headless Testing

Using `freya-testing` you can test your Freya components in a no-window (headless) environment. You can decide to render the app at any moment to a file though. `freya-testing` is actually used by Freya itself to test all the out of the box components and other APIs.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;
use freya_testing::prelude::*;

fn app() -> impl IntoElement {
    let mut state = use_consume::<State<i32>>();
    rect()
        .expanded()
        .center()
        .background((240, 240, 240))
        .on_mouse_up(move |_| *state.write() += 1)
        .child(format!("Clicked: {}", state.read()))
}

fn main() {
    // Create headless testing runner
    let (mut test, state) = TestingRunner::new(
        app,
        (300., 300.).into(),
        |runner| runner.provide_root_context(|| State::create(0)),
        1.,
    );

    test.sync_and_update();
    assert_eq!(*state.peek(), 0);

    // Simulate user interactions
    test.click_cursor((15., 15.));
    assert_eq!(*state.peek(), 1);
}
```

</details>

<div align="center">
  <img src="https://github.com/user-attachments/assets/1340796c-f85b-4522-91d2-95d4bc80e7fc">
</div>

### Advanced Plotting & Charts

Using the Plotters library, you can create charts, graphs, and data visualizations directly within your application.
Enable with the `ploters` feature.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;
use freya::plot::*;
use freya::plot::plotters::*;

fn on_render(ctx: &mut RenderContext, (cursor_x, cursor_y): (f64, f64)) {
    let backend = PlotSkiaBackend::new(
        ctx.canvas,
        ctx.font_collection,
        ctx.layout_node.area.size.to_i32().to_tuple(),
    ).into_drawing_area();

    backend.fill(&WHITE).unwrap();

    let pitch = std::f64::consts::PI * (0.5 - cursor_y / ctx.layout_node.area.height() as f64);
    let yaw = std::f64::consts::PI * 2.0 * (cursor_x / ctx.layout_node.area.width() as f64 - 0.5);
    let scale = 0.4 + 0.6 * (1.0 - cursor_y / ctx.layout_node.area.height() as f64);

    let x_axis = (-3.0..3.0).step(0.1);
    let z_axis = (-3.0..3.0).step(0.1);

    let mut chart = ChartBuilder::on(&backend)
        .caption("3D Surface Plot", ("sans", 20))
        .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone())
        .unwrap();

    chart.with_projection(|mut pb| {
        pb.pitch = pitch;
        pb.yaw = yaw;
        pb.scale = scale;
        pb.into_matrix()
    });

    chart
        .draw_series(
            SurfaceSeries::xoz(
                (-30..30).map(|f| f as f64 / 10.0),
                (-30..30).map(|f| f as f64 / 10.0),
                |x, z| (x * x + z * z).cos(),
            )
            .style(BLUE.mix(0.2).filled()),
        )
        .unwrap()
        .label("Interactive Surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));
}

fn app() -> impl IntoElement {
    let mut cursor_position = use_state(CursorPoint::default);

    plot(RenderCallback::new(move |context| {
        on_render(context, cursor_position().to_tuple());
    }))
    .expanded()
}
```

</details>

[Plots](https://github.com/user-attachments/assets/236285ea-a2c2-4739-a59d-029a7c3ef601)

### Internationalization (i18n)

Freya supports internationalization with built-in support for the [Fluent](https://github.com/projectfluent/fluent-rs) localization system. Easily manage translations, pluralization, and locale-specific formatting.
Enable with the `i18n` feature.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;
use freya::i18n::*;

fn app() -> impl IntoElement {
    let mut i18n = use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("./i18n/en-US.ftl")))
            .with_locale((langid!("es-ES"), PathBuf::from("./examples/i18n/es-ES.ftl")))
    });

    let change_to_english = move |_| i18n.set_language(langid!("en-US"));
    let change_to_spanish = move |_| i18n.set_language(langid!("es-ES"));

    rect()
        .expanded()
        .center()
        .child(
            rect()
                .horizontal()
                .child(Button::new().on_press(change_to_english).child("English"))
                .child(Button::new().on_press(change_to_spanish).child("Spanish")),
        )
        .child(t!("hello_world"))
        .child(t!("hello", name: "Freya!"))
}
```

</details>

<div align="center">
  <img src="https://github.com/user-attachments/assets/95dc2547-9137-4291-87d4-3f5161493c65">
</div>

### Material Design Components

Freya provides Material Design-inspired style modifiers.
Enable with the `material-design` feature.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;
use freya::material_design::*;

fn app() -> impl IntoElement {
    rect().center().expanded().child(
        Button::new()
            .on_press(|_| println!("Material button pressed"))
            .ripple()  // Adds Material Design ripple effect
            .child("Material Button"),
    )
}
```

</details>

<div align="center">
  <img src="https://github.com/user-attachments/assets/fdc8e4e9-321a-4ad2-8190-ad6772fc9dc0">
</div>

### WebView Integration

Integrate web content into your native applications with Freya's WebView support. Embed web applications, or simply display web-based content alongside your native UI components.
Enable with the `webview` feature.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;
use freya::webview::*;

fn app() -> impl IntoElement {
    // Multi-tab webview implementation
    let mut tabs = use_state(|| vec![Tab {
        id: WebViewId::new(),
        title: "Tab 1".to_string(),
        url: "https://duckduckgo.com".to_string(),
    }]);
    let mut active_tab = use_state(|| tabs.read()[0].id);

    rect()
        .expanded()
        .height(Size::fill())
        .background((35, 35, 35))
        .child(
            rect()
                .width(Size::fill())
                .height(Size::px(45.))
                .padding(4.)
                .background((50, 50, 50))
                .horizontal()
                .cross_align(Alignment::Center)
                .spacing(4.)
                .children(tabs.read().iter().map(|tab| {
                    // Tab implementation...
                }))
        )
        .child(WebView::new("https://duckduckgo.com").expanded())
}
```

</details>

<div align="center">
  <img src="https://github.com/user-attachments/assets/60d6db71-fa22-4bbb-a4ad-1ff00a056614">
</div>


### Terminal Emulation

Freya includes terminal emulation capabilities with full PTY (pseudo-terminal) support. Create integrated terminal applications, SSH clients, or development tools.
Enable with the `terminal` feature.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;
use freya::terminal::*;

fn app() -> impl IntoElement {
    let mut handle = use_state(|| {
        let mut cmd = CommandBuilder::new("bash");
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");
        TerminalHandle::new(cmd).ok()
    });

    rect()
        .expanded()
        .center()
        .background((30, 30, 30))
        .color((245, 245, 245))
        .child(if let Some(handle) = handle.read().clone() {
            rect()
                .child(Terminal::new(handle.clone()))
                .expanded()
                .background((10, 10, 10))
                .padding(6.)
                .a11y_id(focus.a11y_id())
                .on_key_down(move |e: Event<KeyboardEventData>| {
                    if e.modifiers.contains(Modifiers::CONTROL)
                        && matches!(&e.key, Key::Character(ch) if ch.len() == 1)
                    {
                        if let Key::Character(ch) = &e.key {
                            let _ = handle.write(&[ch.as_bytes()[0] & 0x1f]);
                        }
                    } else if let Some(ch) = e.try_as_str() {
                        let _ = handle.write(ch.as_bytes());
                    } else {
                        let _ = handle.write(match &e.key {
                            Key::Named(NamedKey::Enter) => b"\r",
                            Key::Named(NamedKey::Backspace) => &[0x7f],
                            Key::Named(NamedKey::Delete) => b"\x1b[3~",
                            Key::Named(NamedKey::Tab) => b"\t",
                            Key::Named(NamedKey::Escape) => &[0x1b],
                            Key::Named(NamedKey::ArrowUp) => b"\x1b[A",
                            Key::Named(NamedKey::ArrowDown) => b"\x1b[B",
                            Key::Named(NamedKey::ArrowLeft) => b"\x1b[D",
                            Key::Named(NamedKey::ArrowRight) => b"\x1b[C",
                            _ => return,
                        });
                    };
                })
        } else {
            "Terminal exited".into_element()
        })
}
```

</details>

<div align="center">
  <img src="https://github.com/user-attachments/assets/12322dad-cb89-450b-bd8f-c9d5fbe2d45a">
</div>

### Developer Tools

Examine the component tree in real-time.

Enable the `devtools` feature in `freya` and then run the devtools app.

<details>
<summary>Code</summary>

```rust
use freya::prelude::*;

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .child("Hello, World!")
}
```

</details>


<div align="center">
  <img src="https://github.com/user-attachments/assets/906fdbec-7b3c-4dc4-a420-95fdf852b1e4">
</div>

### Trying it out

Make sure to have [Development Setup](https://docs.rs/freya/0.3/freya/_docs/development_setup/index.html) ready.

> ⚠️ If you happen to be on Windows using `windows-gnu` and get compile errors, maybe go check this [issue](https://github.com/marc2332/freya/issues/794).

Clone this repo and run:

> **Note:** After cloning, make sure to initialize and update the git submodules: `git submodule update --init --recursive`

```shell
cargo run --example counter
```

### Usage 📜
`main` branch:

```toml
freya = { git = "https://github.com/marc2332/freya", branch = "main" }
```

Release candidates:

```toml
freya = "0.4.0-rc.6"
```

### Contributing 🧙‍♂️
If you are interested in contributing please make sure to have read the [Contributing](CONTRIBUTING.md) guide first!

### Contact 
You may contact me for questions, collaboration or anything that comes to your mind at [marc@mespin.me](mailto:marc@mespin.me).

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

### History with Dioxus
Freya 0.1, 0.2 and 0.3 were based on the core crates of Dioxus. Starting 0.4 Freya does no longer use Dioxus, instead it uses its own reactive core, partially inspired by Dioxus but yet with lots of differences.

### License

[MIT License](./LICENSE.md)
