#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .padding(20.)
        .spacing(16.)
        .child("Tab: move between groups · ArrowUp/Down: move within a group")
        .child(
            rect()
                .horizontal()
                .spacing(24.)
                .child(
                    Group {
                        title: "Formatting".to_string(),
                        items: vec![
                            "Bold".to_string(),
                            "Italic".to_string(),
                            "Underline".to_string(),
                        ],
                    }
                    .into_element(),
                )
                .child(
                    Group {
                        title: "Alignment".to_string(),
                        items: vec![
                            "Left".to_string(),
                            "Center".to_string(),
                            "Right".to_string(),
                        ],
                    }
                    .into_element(),
                ),
        )
}

#[derive(Clone)]
struct GroupContext {
    group_id: AccessibilityId,
}

#[derive(PartialEq, Clone)]
struct Group {
    title: String,
    items: Vec<String>,
}

impl Component for Group {
    fn render(&self) -> impl IntoElement {
        let group_id = use_a11y();
        let focus = use_focus(group_id);
        use_provide_context(|| GroupContext { group_id });

        let border_fill = if focus().is_focused() {
            (140, 160, 240)
        } else {
            (70, 70, 80)
        };

        rect()
            .a11y_id(group_id)
            .a11y_member_of(group_id)
            .a11y_focusable(true)
            .background((40, 40, 45))
            .border(
                Border::new()
                    .fill(border_fill)
                    .width(2.)
                    .alignment(BorderAlignment::Inner),
            )
            .corner_radius(8.)
            .padding(12.)
            .spacing(6.)
            .color(Color::WHITE)
            .on_mouse_down(move |_| group_id.request_focus())
            .child(self.title.clone())
            .children(
                self.items
                    .iter()
                    .map(|name| GroupItem { name: name.clone() }.into_element())
                    .collect::<Vec<_>>(),
            )
    }
}

#[derive(PartialEq, Clone)]
struct GroupItem {
    name: String,
}

impl Component for GroupItem {
    fn render(&self) -> impl IntoElement {
        let GroupContext { group_id } = use_consume::<GroupContext>();
        let a11y_id = use_a11y();
        let focus = use_focus(a11y_id);

        let background = match focus() {
            Focus::Keyboard => (90, 110, 200),
            Focus::Pointer => (60, 70, 110),
            Focus::Not => (55, 55, 60),
        };

        rect()
            .a11y_id(a11y_id)
            .a11y_member_of(group_id)
            .a11y_focusable(true)
            .background(background)
            .corner_radius(6.)
            .padding(8.)
            .color(Color::WHITE)
            .on_mouse_down(move |_| a11y_id.request_focus())
            .child(self.name.clone())
    }
}
