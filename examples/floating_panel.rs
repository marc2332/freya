#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    prelude::*,
    winit::{dpi::LogicalPosition, platform::x11::{WindowAttributesExtX11, WindowType}, window::WindowLevel},
};

fn main() {
    let (width, height) = (800, 56);
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("Floating Panel")
                .with_size(width as f64, height as f64)
                .with_min_size(width as f64, height as f64)
                .with_max_size(width as f64, height as f64)
                .with_decorations(false)
                .with_resizable(false)
                .with_transparency(true)
                .with_background(Color::TRANSPARENT)
                .with_window_attributes(move |attributes, el| {
                    let attributes = attributes.with_x11_window_type(vec![WindowType::Dock]).with_window_level(WindowLevel::AlwaysOnTop);

                    // Center the window horizontally, position at top of screen
                    if let Some(monitor) = el
                        .primary_monitor()
                        .or_else(|| el.available_monitors().next())
                    {
                        let size = monitor.size();
                        attributes.with_position(LogicalPosition {
                            x: size.width as i32 / 2 - width / 2,
                            y: 40,
                        })
                    } else {
                        attributes
                    }
                }),
        ),
    )
}

fn app() -> impl IntoElement {
    rect()
        .width(Size::fill())
        .height(Size::fill())
        .horizontal()
        .center()
        .content(Content::Flex)
        .padding(Gaps::new_symmetric(6., 12.))
        .spacing(8.)
        .padding(8.)
        .background(Color::from_rgb(30, 30, 30))
        .corner_radius(8.)
        .child(panel_button("Home", "H"))
        .child(panel_button("Search", "S"))
        .child(panel_button("Files", "F"))
        .child(panel_button("Settings", "G"))
        .child(spacer())
        .child(panel_button("Close", "X"))
}

fn panel_button(text: &'static str, shortcut: &'static str) -> Element {
    let mut hovered = use_state(|| false);

    let background = if *hovered.read() {
        Color::from_rgb(60, 60, 60)
    } else {
        Color::TRANSPARENT
    };

    rect()
        .horizontal()
        .center()
        .padding(Gaps::new_symmetric(6., 12.))
        .corner_radius(6.)
        .background(background)
        .spacing(6.)
        .color(Color::from_rgb(220, 220, 220))
        .on_pointer_enter(move |_| hovered.set(true))
        .on_pointer_leave(move |_| hovered.set(false))
        .on_mouse_up(move |_| {
            println!("Clicked: {}", text);
        })
        .child(text)
        .child(
            rect()
                .padding(Gaps::new_symmetric(2., 6.))
                .corner_radius(4.)
                .background(Color::from_rgb(80, 80, 80))
                .color(Color::from_rgb(160, 160, 160))
                .font_size(11.)
                .child(shortcut),
        )
        .into()
}

fn spacer() -> Element {
    rect().width(Size::flex(1.)).into()
}
