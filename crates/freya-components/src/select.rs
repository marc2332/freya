use freya_animation::prelude::*;
use freya_core::prelude::*;
use torin::prelude::*;

use crate::{
    get_theme,
    icons::arrow::ArrowIcon,
    menu::MenuGroup,
    theming::component_themes::SelectThemePartial,
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SelectStatus {
    #[default]
    Idle,
    Hovering,
}

/// Select between different items component.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let values = use_hook(|| {
///         vec![
///             "Rust".to_string(),
///             "Turbofish".to_string(),
///             "Crabs".to_string(),
///         ]
///     });
///     let mut selected_select = use_state(|| 0);
///
///     Select::new()
///         .selected_item(values[selected_select()].to_string())
///         .children(values.iter().enumerate().map(|(i, val)| {
///             MenuItem::new()
///                 .selected(selected_select() == i)
///                 .on_press(move |_| selected_select.set(i))
///                 .child(val.to_string())
///                 .into()
///         }))
/// }
///
/// # use freya_testing::prelude::*;
/// # use std::time::Duration;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_select.png").with_hook(|t| { t.move_cursor((125., 125.)); t.click_cursor((125., 125.)); t.poll(Duration::from_millis(1), Duration::from_millis(350)); }).with_scale_factor(1.).render();
/// ```
///
/// # Preview
/// ![Select Preview][select]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("select", "images/gallery_select.png")
)]
#[derive(Clone, PartialEq)]
pub struct Select {
    pub(crate) theme: Option<SelectThemePartial>,
    pub selected_item: Option<Element>,
    pub children: Vec<Element>,
    pub key: DiffKey,
}

impl ChildrenExt for Select {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl Select {
    pub fn new() -> Self {
        Self {
            theme: None,
            selected_item: None,
            children: Vec::new(),
            key: DiffKey::None,
        }
    }

    pub fn theme(mut self, theme: SelectThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn selected_item(mut self, item: impl Into<Element>) -> Self {
        self.selected_item = Some(item.into());
        self
    }

    pub fn key(mut self, key: impl Into<DiffKey>) -> Self {
        self.key = key.into();
        self
    }
}

impl Component for Select {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, select);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let mut status = use_state(SelectStatus::default);
        let mut open = use_state(|| false);
        use_provide_context(|| MenuGroup {
            group_id: focus.a11y_id(),
        });

        let animation = use_animation(move |conf| {
            conf.on_change(OnChange::Rerun);
            conf.on_creation(OnCreation::Finish);

            let scale = AnimNum::new(0.8, 1.)
                .time(350)
                .ease(Ease::Out)
                .function(Function::Expo);
            let opacity = AnimNum::new(0., 1.)
                .time(350)
                .ease(Ease::Out)
                .function(Function::Expo);
            if open() {
                (scale, opacity)
            } else {
                (scale.into_reversed(), opacity.into_reversed())
            }
        });

        use_drop(move || {
            if status() == SelectStatus::Hovering {
                Cursor::set(CursorIcon::default());
            }
        });

        // Close the select when the focused accessibility node changes and its not the select or any of its children
        use_side_effect(move || {
            let platform = Platform::get();
            if *platform.navigation_mode.read() == NavigationMode::Keyboard {
                if let Some(member_of) = platform.focused_accessibility_node.read().member_of() {
                    if member_of != focus.a11y_id() {
                        open.set_if_modified(false);
                    }
                } else {
                    open.set_if_modified(false);
                }
            }
        });

        let on_press = move |e: Event<PressEventData>| {
            focus.request_focus();
            open.toggle();
            // Prevent global mouse up
            e.prevent_default();
            e.stop_propagation();
        };

        let on_pointer_enter = move |_| {
            *status.write() = SelectStatus::Hovering;
            Cursor::set(CursorIcon::Pointer);
        };

        let on_pointer_leave = move |_| {
            *status.write() = SelectStatus::Idle;
            Cursor::set(CursorIcon::default());
        };

        // Close the select if clicked anywhere
        let on_global_mouse_up = move |_| {
            open.set_if_modified(false);
        };

        let on_global_key_down = move |e: Event<KeyboardEventData>| match e.key {
            Key::Named(NamedKey::Escape) => {
                open.set_if_modified(false);
            }
            Key::Named(NamedKey::Enter) if focus.is_focused() => {
                open.toggle();
            }
            _ => {}
        };

        let (scale, opacity) = animation.read().value();

        let background = match *status.read() {
            SelectStatus::Hovering => theme.hover_background,
            SelectStatus::Idle => theme.background_button,
        };

        let border = if focus_status() == FocusStatus::Keyboard {
            Border::new()
                .fill(theme.focus_border_fill)
                .width(2.)
                .alignment(BorderAlignment::Inner)
        } else {
            Border::new()
                .fill(theme.border_fill)
                .width(1.)
                .alignment(BorderAlignment::Inner)
        };

        rect()
            .child(
                rect()
                    .a11y_id(focus.a11y_id())
                    .a11y_member_of(focus.a11y_id())
                    .a11y_role(AccessibilityRole::ListBox)
                    .a11y_focusable(Focusable::Enabled)
                    .on_pointer_enter(on_pointer_enter)
                    .on_pointer_leave(on_pointer_leave)
                    .on_press(on_press)
                    .on_global_key_down(on_global_key_down)
                    .on_global_mouse_up(on_global_mouse_up)
                    .width(theme.width)
                    .margin(theme.margin)
                    .background(background)
                    .padding((6., 16., 6., 16.))
                    .border(border)
                    .horizontal()
                    .center()
                    .color(theme.color)
                    .corner_radius(8.)
                    .maybe_child(self.selected_item.clone())
                    .child(
                        ArrowIcon::new()
                            .margin((0., 0., 0., 8.))
                            .rotate(0.)
                            .fill(theme.arrow_fill),
                    ),
            )
            .maybe_child((open() || opacity > 0.).then(|| {
                rect().height(Size::px(0.)).width(Size::px(0.)).child(
                    rect()
                        .width(Size::window_percent(100.))
                        .margin(Gaps::new(4., 0., 0., 0.))
                        .child(
                            rect()
                                .layer(Layer::Overlay)
                                .border(
                                    Border::new()
                                        .fill(theme.border_fill)
                                        .width(1.)
                                        .alignment(BorderAlignment::Inner),
                                )
                                .overflow(Overflow::Clip)
                                .corner_radius(8.)
                                .background(theme.select_background)
                                // TODO: Shadows
                                .padding(6.)
                                .content(Content::Fit)
                                .opacity(opacity)
                                .scale(scale)
                                .children(self.children.clone()),
                        ),
                )
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
