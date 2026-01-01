#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 450.)))
}

fn app() -> impl IntoElement {
    use_init_theme(|| DARK_THEME);
    let mut show_menu = use_state(|| false);

    rect()
        .theme_background()
        .expanded()
        .child(
            Button::new()
                .on_press(move |_| show_menu.toggle())
                .child("Open"),
        )
        .maybe_child(show_menu().then(|| {
            Menu::new()
                .on_close(move |_| show_menu.set(false))
                .child(MenuButton::new().child("Open"))
                .child(MenuButton::new().child("Save"))
                .child(
                    SubMenu::new()
                        .child(MenuButton::new().child("Option 1"))
                        .child(
                            SubMenu::new()
                                .child(MenuButton::new().child("Option 2"))
                                .child(MenuButton::new().child("Option 3"))
                                .label("More Options"),
                        )
                        .child(
                            SubMenu::new()
                                .child(MenuButton::new().child("Option 4"))
                                .child(MenuButton::new().child("Option 5"))
                                .child(
                                    SubMenu::new()
                                        .child(MenuButton::new().child("Option 6"))
                                        .child(MenuButton::new().child("Option 7"))
                                        .child(MenuButton::new().child("Option 8"))
                                        .label("Even More Options"),
                                )
                                .label("Other Options"),
                        )
                        .label("Options"),
                )
                .child(
                    MenuButton::new()
                        .child("Close")
                        .on_press(move |_| show_menu.set(false)),
                )
        }))
}
