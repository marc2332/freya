#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

#[derive(Clone, Copy, PartialEq)]
enum FieldState {
    Empty,
    Valid,
    HasExclamation,
    TooLong,
}

impl FieldState {
    fn classify(text: &str) -> Self {
        if text.is_empty() {
            Self::Empty
        } else if text.chars().count() > 12 {
            Self::TooLong
        } else if text.contains('!') {
            Self::HasExclamation
        } else {
            Self::Valid
        }
    }

    fn helper(self) -> &'static str {
        match self {
            Self::Empty => "Required",
            Self::Valid => "Looks good",
            Self::HasExclamation => "No exclamation marks allowed",
            Self::TooLong => "Too long (max 12 characters)",
        }
    }

    fn color(self) -> Color {
        match self {
            Self::Empty => Color::from_rgb(140, 140, 140),
            Self::Valid => Color::from_rgb(30, 150, 60),
            Self::HasExclamation => Color::from_rgb(200, 35, 35),
            Self::TooLong => Color::from_rgb(220, 140, 20),
        }
    }
}

fn app() -> impl IntoElement {
    let username = use_state(String::new);
    let mut field = use_state(|| FieldState::Empty);

    rect()
        .center()
        .expanded()
        .spacing(8.)
        .child(
            Input::new(username)
                .placeholder("Username")
                .width(Size::px(240.))
                .on_validate(move |validator: InputValidator| {
                    let has_digit = validator.text().chars().any(|c| c.is_ascii_digit());
                    validator.set_valid(!has_digit);

                    if !has_digit {
                        field.set_if_modified(FieldState::classify(&validator.text()));
                    }
                })
                .maybe(field() != FieldState::Empty, |el| {
                    el.border_fill(field().color())
                        .focus_border_fill(field().color())
                }),
        )
        .child(
            label()
                .text(field().helper())
                .color(field().color())
                .font_size(14.),
        )
}
