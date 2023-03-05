#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

static FERRIS: &[u8] = include_bytes!("./ferris.svg");
static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

fn main() {
    launch_cfg(
        app,
        WindowConfig::<()>::builder()
            .with_width(550)
            .with_height(700)
            .with_decorations(true)
            .with_transparency(false)
            .with_title("Freya showcase!")
            .build(),
    );
}

#[allow(non_snake_case)]
fn Space(cx: Scope) -> Element {
    render!(rect { padding: "5" })
}

#[allow(non_snake_case)]
#[inline_props]
fn SectionHeader<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(
        label { font_size: "20", &*children },
        Space {}
    )
}

#[allow(non_snake_case)]
fn Body(cx: Scope) -> Element {
    let theme = use_theme(&cx);
    let theme = theme.read();
    let body_theme = &theme.body;

    render!(
        rect {
            padding: "20",
            background: "{body_theme.background}",
            color: "{body_theme.color}",
            width: "100%",
            height: "100%",
            direction: "horizontal",
            container {
                width: "50%",
                height: "100%",
                FirstSection {}
            }
            Space {},
            container {
                width: "50%",
                height: "100%",
                SecondSection {}
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    use_init_theme(&cx, DARK_THEME);

    render!(Body {})
}

#[allow(non_snake_case)]
fn FirstSection(cx: Scope) -> Element {
    let theme = use_theme(&cx);
    let slider_percentage = use_state(cx, || 50.0);

    let current_theme = &theme.read();
    let enabled = current_theme.eq(&LIGHT_THEME);

    render!(
        SectionHeader {
            "Button"
        }
        Space {},
        Button {
            onclick: move |_| {
                *theme.write() = DARK_THEME.clone();
            },
            label {
                width: "115",
                "Restart Theme"
            }
        }
        Space {},
        SectionHeader {
            "Switch"
        }
        Space {},
        Switch {
            enabled: enabled,
            ontoggled: move |_| {
                if enabled {
                    *theme.write() = DARK_THEME.clone();
                } else {
                    *theme.write() = LIGHT_THEME.clone();
                }
            }
        }
        Switch {
            enabled: !enabled,
            ontoggled: move |_| {
                if enabled {
                    *theme.write() = DARK_THEME.clone();
                } else {
                    *theme.write() = LIGHT_THEME.clone();
                }
            }
        }
        Space {},
        SectionHeader {
            "Slider"
        }
        Space {},
        Slider {
            width: 100.0,
            value: *slider_percentage.get(),
            onmoved: |p| {
                slider_percentage.set(p);
            }
        }
        Space {},
        label {
            "Value is {slider_percentage.floor()}"
        }
        Space {},
        SectionHeader {
            "Scrollview"
        }
        label {
            "Make any content scrollable, even in both directions."
        }
        Space {},
        ScrollView {
            show_scrollbar: true,
            width: "100%",
            height: "200",
            rect {
                width: "110%",
                height: "105",
                background: "rgb(152, 201, 163)"
            }
            rect {
                width: "110%",
                height: "105",
                background: "rgb(221, 231, 199)"
            }
            rect {
                width: "110%",
                height: "105",
                background: "rgb(197, 186, 175)",
            }
        }
    )
}

#[allow(non_snake_case)]
fn SecondSection(cx: Scope) -> Element {
    let ferris = bytes_to_data(cx, FERRIS);
    let rust = bytes_to_data(cx, RUST_LOGO);
    render!(
        SectionHeader {
            "SVG and Images"
        }
        Space {},
        rect {
            direction: "horizontal",
            height: "150",
            svg {
                width: "150",
                height: "150",
                svg_data: ferris,
            }
            Space {},
            image {
                image_data: rust,
                width: "130",
                height: "130",
            }
        }
        SectionHeader {
            "And so many other things!"
        }
    )
}
