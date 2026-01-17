use freya_core::prelude::*;
use torin::{
    content::Content,
    gaps::Gaps,
    size::Size,
};

use crate::WinitPlatformExt;

#[derive(Clone, PartialEq)]
pub struct Titlebar {
    pub(crate) title: Option<String>,
    pub(crate) elements: Vec<Element>,
    key: DiffKey,
}

impl Default for Titlebar {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for Titlebar {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for Titlebar {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Titlebar {
    pub fn new() -> Self {
        Self {
            title: None,
            elements: Vec::default(),
            key: DiffKey::None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for Titlebar {
    fn render(&self) -> impl IntoElement {
        let title = self.title.clone();

        rect()
            .width(Size::fill())
            .height(Size::px(32.0))
            .background(Color::from_rgb(240, 240, 240))
            .horizontal()
            .content(Content::flex())
            .child(
                rect()
                    .on_pointer_down(|e: Event<PointerEventData>| {
                        match EventsCombos::pressed(e.global_location()) {
                            PressEventType::Single => {
                                Platform::get().with_window(None, |window| {
                                    let _ = window.drag_window();
                                });
                            }
                            PressEventType::Double => {
                                Platform::get().with_window(None, |window| {
                                    if window.is_maximized() {
                                        window.set_maximized(false);
                                    } else {
                                        window.set_maximized(true);
                                    }
                                });
                            }
                            _ => {}
                        }
                    })
                    .width(Size::flex(1.0))
                    .background(Color::from_rgb(240, 240, 240))
                    .maybe_child(if let Some(title) = title {
                        Some(
                            rect()
                                .width(Size::flex(1.0))
                                .padding(Gaps::new(8.0, 8.0, 8.0, 8.0))
                                .child(label().text(title)),
                        )
                    } else {
                        None
                    }),
            )
            .child(
                rect()
                    .horizontal()
                    .content(Content::flex())
                    .children(self.elements.clone()),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(Clone, PartialEq)]
pub struct TitlebarButton {
    pub(crate) elements: Vec<Element>,
    pub(crate) on_press: Option<EventHandler<Event<PressEventData>>>,
    key: DiffKey,
}

impl Default for TitlebarButton {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for TitlebarButton {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for TitlebarButton {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl TitlebarButton {
    pub fn new() -> Self {
        Self {
            elements: Vec::default(),
            on_press: None,
            key: DiffKey::None,
        }
    }

    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }
}

impl Component for TitlebarButton {
    fn render(&self) -> impl IntoElement {
        let focus = use_focus();
        let focus_status = use_focus_status(focus);

        rect()
            .width(Size::px(46.0))
            .height(Size::px(32.0))
            .background(Color::TRANSPARENT)
            .a11y_id(focus.a11y_id())
            .a11y_focusable(true)
            .a11y_role(AccessibilityRole::Button)
            .border(if focus_status() == FocusStatus::Keyboard {
                Border::new()
                    .fill(Color::from_rgb(59, 130, 246))
                    .width(2.)
                    .alignment(BorderAlignment::Inner)
            } else {
                Border::new()
                    .fill(Color::TRANSPARENT)
                    .width(1.)
                    .alignment(BorderAlignment::Inner)
            })
            .center()
            .on_all_press({
                let on_press = self.on_press.clone();
                move |e: Event<PressEventData>| {
                    focus.request_focus();
                    if let Some(handler) = &on_press {
                        handler.call(e);
                    }
                }
            })
            .children(self.elements.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
