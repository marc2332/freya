#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use fermi::*;
use freya::{dioxus_elements, *};

static FERRIS: &[u8] = include_bytes!("./ferris.svg");
static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

fn main() {
    launch_cfg(vec![(
        app,
        WindowConfig {
            width: 550,
            height: 700,
            decorations: true,
            title: "Freya Showcase",
            transparent: false,
        },
    )]);
}

#[allow(non_snake_case)]
fn Space(cx: Scope) -> Element {
    render!(rect { padding: "10" })
}

#[allow(non_snake_case)]
#[inline_props]
fn SectionHeader<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(
        label { font_size: "20", &*children },
        Space {}
    )
}

fn app(cx: Scope) -> Element {
    let theme = use_atom_ref(&cx, THEME);
    let current_theme = &theme.read();
    let body_theme = &current_theme.body;

    render!(
        rect {
            padding: "35",
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

#[allow(non_snake_case)]
fn FirstSection(cx: Scope) -> Element {
    let theme = use_atom_ref(&cx, THEME);
    let current_theme = &theme.read();
    let enabled = current_theme.eq(&LIGHT_THEME);

    let slider_percentage = use_state(&cx, || 50.0);

    render!(
        SectionHeader {
            "Button"
        }
        Space {},
        Button {
            on_click: |_| {
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
            "Value is {slider_percentage}"
        }
    )
}

#[allow(non_snake_case)]
fn SecondSection(cx: Scope) -> Element {
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
                svg_data: FERRIS,
            }
            Space {},
            image {
                image_data: RUST_LOGO,
                width: "130",
                height: "130",
            }
        }
        SectionHeader {
            "And so many other things!"
        }
    )
}
