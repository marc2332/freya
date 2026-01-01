#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 450.)))
}

fn context_menu() -> Menu {
    Menu::new()
        .child(
            SubMenu::new()
                .child(MenuButton::new().child("Option 1"))
                .child(MenuButton::new().child("Option 2"))
                .label("Options"),
        )
        .child(MenuButton::new().child("Empty"))
        .child(
            SubMenu::new()
                .child(MenuButton::new().child("Option 3"))
                .label("Other Options"),
        )
        .child(
            MenuButton::new()
                .child("Close")
                .on_press(move |_| ContextMenu::close()),
        )
}

fn app() -> impl IntoElement {
    use_init_root_theme(|| DARK_THEME);

    rect().theme_background().center().expanded().child(
        Button::new()
            .on_press(move |_| {
                if ContextMenu::is_open() {
                    ContextMenu::close();
                } else {
                    ContextMenu::open(context_menu())
                }
            })
            .child("Open"),
    )
}
