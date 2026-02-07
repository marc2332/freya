#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_os = "linux")]
use freya::winit::platform::x11::{
    WindowAttributesExtX11,
    WindowType,
};
use freya::{
    prelude::*,
    winit::{
        dpi::LogicalPosition,
        window::WindowLevel,
    },
};

fn main() {
    let (width, height) = (800, 56);
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new_app(MyApp {})
                .with_title("Floating Panel")
                .with_size(width as f64, height as f64)
                .with_decorations(false)
                .with_resizable(false)
                .with_transparency(true)
                .with_background(Color::TRANSPARENT)
                .with_window_attributes(move |attributes, el| {
                    #[cfg(target_os = "linux")]
                    let attributes = attributes.with_x11_window_type(vec![WindowType::Dock]);

                    let mut attributes = attributes.with_window_level(WindowLevel::AlwaysOnTop);

                    if let Some(monitor) = el
                        .primary_monitor()
                        .or_else(|| el.available_monitors().next())
                    {
                        let size = monitor.size();
                        attributes = attributes.with_position(LogicalPosition {
                            x: size.width as i32 / 2 - width / 2,
                            y: 40,
                        })
                    }

                    attributes
                }),
        ),
    )
}

struct MyApp;

impl App for MyApp {
    fn render(&self) -> impl IntoElement {
        rect()
            .expanded()
            .horizontal()
            .center()
            .content(Content::Flex)
            .padding((6., 12.))
            .spacing(8.)
            .padding(8.)
            .background((30, 30, 30))
            .corner_radius(8.)
            .child(PanelButton {
                text: "Home",
                shortcut: "H",
            })
            .child(PanelButton {
                text: "Search",
                shortcut: "S",
            })
            .child(PanelButton {
                text: "Files",
                shortcut: "F",
            })
            .child(PanelButton {
                text: "Settings",
                shortcut: "G",
            })
            .child(rect().width(Size::flex(1.)))
            .child(PanelButton {
                text: "Close",
                shortcut: "X",
            })
    }
}

#[derive(PartialEq)]
struct PanelButton {
    text: &'static str,
    shortcut: &'static str,
}

impl Component for PanelButton {
    fn render(&self) -> impl IntoElement {
        let mut hovered = use_state(|| false);

        let background = if *hovered.read() {
            Color::from_rgb(60, 60, 60)
        } else {
            Color::TRANSPARENT
        };

        let text = self.text;
        let shortcut = self.shortcut;

        rect()
            .horizontal()
            .center()
            .padding((6., 12.))
            .corner_radius(6.)
            .background(background)
            .spacing(6.)
            .color((220, 220, 220))
            .on_pointer_enter(move |_| hovered.set(true))
            .on_pointer_leave(move |_| hovered.set(false))
            .on_press(move |_| {
                println!("Clicked: {}", text);
            })
            .child(text)
            .child(
                rect()
                    .padding((2., 6.))
                    .corner_radius(4.)
                    .background((80, 80, 80))
                    .color((160, 160, 160))
                    .font_size(11.)
                    .child(shortcut),
            )
    }
}
