#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

#[derive(Clone)]
struct FruitItem {
    name: String,
    a11y_id: AccessibilityId,
}

fn app() -> impl IntoElement {
    let items = use_state(|| {
        ["Apples", "Bananas", "Cherries", "Dates", "Elderberries"]
            .iter()
            .map(|name| FruitItem {
                name: name.to_string(),
                a11y_id: AccessibilityId::new_unique(),
            })
            .collect::<Vec<_>>()
    });

    let focus_first = move |_| {
        if let Some(item) = items.read().first() {
            item.a11y_id.request_focus();
        }
    };

    let focus_last = move |_| {
        if let Some(item) = items.read().last() {
            item.a11y_id.request_focus();
        }
    };

    let focus_next = move |_| {
        let items = items.read();
        if items.is_empty() {
            return;
        }
        let current = items.iter().position(|item| item.a11y_id.is_focused());
        let next_index = current.map(|i| (i + 1) % items.len()).unwrap_or(0);
        items[next_index].a11y_id.request_focus();
    };

    let focus_previous = move |_| {
        let items = items.read();
        if items.is_empty() {
            return;
        }
        let len = items.len();
        let current = items.iter().position(|item| item.a11y_id.is_focused());
        let previous_index = current.map(|i| (i + len - 1) % len).unwrap_or(len - 1);
        items[previous_index].a11y_id.request_focus();
    };

    rect()
        .expanded()
        .padding(16.)
        .spacing(12.)
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .child(Button::new().on_press(focus_first).child("First"))
                .child(Button::new().on_press(focus_previous).child("Previous"))
                .child(Button::new().on_press(focus_next).child("Next"))
                .child(Button::new().on_press(focus_last).child("Last")),
        )
        .child(
            rect().spacing(4.).children(
                items
                    .read()
                    .iter()
                    .map(|item| {
                        FruitRow {
                            name: item.name.clone(),
                            a11y_id: item.a11y_id,
                        }
                        .into_element()
                    })
                    .collect::<Vec<_>>(),
            ),
        )
}

#[derive(PartialEq, Clone)]
struct FruitRow {
    name: String,
    a11y_id: AccessibilityId,
}

impl Component for FruitRow {
    fn render(&self) -> impl IntoElement {
        let a11y_id = self.a11y_id;
        let focus = use_focus(a11y_id);

        let background = match focus() {
            Focus::Keyboard => (80, 100, 180),
            Focus::Pointer => (60, 70, 110),
            Focus::Not => (45, 45, 45),
        };

        rect()
            .a11y_id(a11y_id)
            .a11y_focusable(true)
            .padding(10.)
            .corner_radius(6.)
            .background(background)
            .color(Color::WHITE)
            .on_mouse_down(move |_| a11y_id.request_focus())
            .child(self.name.clone())
    }
}
