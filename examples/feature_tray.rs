#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    prelude::*,
    tray::{
        TrayContext,
        TrayEvent,
        TrayIconBuilder,
        menu::{
            Menu,
            MenuEvent,
            MenuItem,
        },
    },
};

const ICON: &[u8] = include_bytes!("./freya_icon.png");

fn main() {
    let tray_icon = || {
        let tray_menu = Menu::new();
        let _ = tray_menu.append(&MenuItem::new("Open", true, None));
        let _ = tray_menu.append(&MenuItem::new("Toggle Visibility", true, None));
        let _ = tray_menu.append(&MenuItem::new("Close All", true, None));
        let _ = tray_menu.append(&MenuItem::new("Exit", true, None));
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Freya Tray")
            .with_icon(LaunchConfig::tray_icon(ICON))
            .build()
            .unwrap()
    };
    let tray_handler = |ev, mut ctx: TrayContext| match ev {
        TrayEvent::Menu(MenuEvent { id }) if id == "3" => {
            ctx.launch_window(WindowConfig::new(app).with_size(500., 450.));
        }
        TrayEvent::Menu(MenuEvent { id }) if id == "4" => {
            for window in ctx.windows_mut().values_mut() {
                if Some(true) == window.window().is_visible() {
                    window.window_mut().set_visible(false);
                } else {
                    window.window_mut().set_visible(true);
                }
            }
        }
        TrayEvent::Menu(MenuEvent { id }) if id == "5" => {
            ctx.windows_mut().drain();
        }
        TrayEvent::Menu(MenuEvent { id }) if id == "6" => {
            ctx.exit();
        }
        _ => {}
    };

    launch(LaunchConfig::new().with_tray(tray_icon, tray_handler))
}

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
