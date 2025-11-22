use freya_core::prelude::*;
use torin::prelude::*;

use crate::{
    get_theme,
    icons::arrow::ArrowIcon,
    theming::component_themes::{
        DropdownItemThemePartial,
        DropdownThemePartial,
    },
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DropdownItemStatus {
    #[default]
    Idle,
    Hovering,
}

#[derive(Clone, PartialEq)]
pub struct DropdownItem {
    pub(crate) theme: Option<DropdownItemThemePartial>,
    pub selected: bool,
    pub on_press: Option<EventHandler<Event<PressEventData>>>,
    pub children: Vec<Element>,
    pub key: DiffKey,
}

impl Default for DropdownItem {
    fn default() -> Self {
        Self::new()
    }
}

impl DropdownItem {
    pub fn new() -> Self {
        Self {
            theme: None,
            selected: false,
            on_press: None,
            children: Vec::new(),
            key: DiffKey::None,
        }
    }

    pub fn theme(mut self, theme: DropdownItemThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_press(mut self, handler: impl FnMut(Event<PressEventData>) + 'static) -> Self {
        self.on_press = Some(EventHandler::new(handler));
        self
    }

    pub fn child(mut self, child: impl Into<Element>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn key(mut self, key: impl Into<DiffKey>) -> Self {
        self.key = key.into();
        self
    }
}

impl Render for DropdownItem {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, dropdown_item);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let mut status = use_state(DropdownItemStatus::default);
        let dropdown_group = use_consume::<DropdownGroup>();

        let background = if self.selected {
            theme.select_background
        } else if *status.read() == DropdownItemStatus::Hovering {
            theme.hover_background
        } else {
            theme.background
        };

        let border = if focus_status() == FocusStatus::Keyboard {
            Border::new()
                .fill(theme.select_border_fill)
                .width(2.)
                .alignment(BorderAlignment::Inner)
        } else {
            Border::new()
                .fill(theme.border_fill)
                .width(1.)
                .alignment(BorderAlignment::Inner)
        };

        rect()
            .width(Size::fill_minimum())
            .color(theme.color)
            .a11y_id(focus.a11y_id())
            .a11y_focusable(Focusable::Enabled)
            .a11y_member_of(dropdown_group.group_id)
            .a11y_role(AccessibilityRole::Button)
            .background(background)
            .border(border)
            .corner_radius(6.)
            .padding((6., 10., 6., 10.))
            .main_align(Alignment::center())
            .on_pointer_enter(move |_| {
                *status.write() = DropdownItemStatus::Hovering;
            })
            .on_pointer_leave(move |_| {
                *status.write() = DropdownItemStatus::Idle;
            })
            .map(self.on_press.clone(), |rect, on_press| {
                rect.on_press(on_press)
            })
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(Clone)]
struct DropdownGroup {
    group_id: AccessibilityId,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DropdownStatus {
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
///     let mut selected_dropdown = use_state(|| 0);
///
///     Dropdown::new()
///         .selected_item(values[selected_dropdown()].to_string())
///         .children_iter(values.iter().enumerate().map(|(i, val)| {
///             DropdownItem::new()
///                 .on_press(move |_| selected_dropdown.set(i))
///                 .child(val.to_string())
///                 .into()
///         }))
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc_hook(|| {
/// #   rect().center().expanded().child(app())
/// # }, (250., 250.).into(), "./images/gallery_dropdown.png", |t| {
/// #   t.move_cursor((125., 125.));
/// #   t.click_cursor((125., 125.));
/// #   t.sync_and_update();
/// # });
/// ```
///
/// # Preview
/// ![Dropdown Preview][dropdown]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("dropdown", "images/gallery_dropdown.png")
)]
#[derive(Clone, PartialEq)]
pub struct Dropdown {
    pub(crate) theme: Option<DropdownThemePartial>,
    pub selected_item: Option<Element>,
    pub children: Vec<Element>,
    pub key: DiffKey,
}

impl ChildrenExt for Dropdown {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Default for Dropdown {
    fn default() -> Self {
        Self::new()
    }
}

impl Dropdown {
    pub fn new() -> Self {
        Self {
            theme: None,
            selected_item: None,
            children: Vec::new(),
            key: DiffKey::None,
        }
    }

    pub fn theme(mut self, theme: DropdownThemePartial) -> Self {
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

impl Render for Dropdown {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, dropdown);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let mut status = use_state(DropdownStatus::default);
        let mut open = use_state(|| false);
        use_provide_context(|| DropdownGroup {
            group_id: focus.a11y_id(),
        });

        let background = match *status.read() {
            DropdownStatus::Hovering => theme.hover_background,
            DropdownStatus::Idle => theme.background_button,
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

        // Close the dropdown when the focused accessibility node changes and its not the dropdown or any of its childrens
        use_side_effect(move || {
            if let Some(member_of) = PlatformState::get()
                .focused_accessibility_node
                .read()
                .member_of()
            {
                if member_of != focus.a11y_id() {
                    open.set(false);
                }
            } else {
                open.set(false);
            }
        });

        let on_press = move |e: Event<PressEventData>| {
            focus.request_focus();
            open.set(true);
            // Prevent global mouse up
            e.prevent_default();
            e.stop_propagation();
        };

        let on_pointer_enter = move |_| {
            *status.write() = DropdownStatus::Hovering;
        };

        let on_pointer_leave = move |_| {
            *status.write() = DropdownStatus::Idle;
        };

        // Close the dropdown if clicked anywhere
        let on_global_mouse_up = move |_| {
            open.set(false);
        };

        let on_global_key_down = move |e: Event<KeyboardEventData>| match e.code {
            Code::Escape => {
                open.set(false);
            }
            Code::Enter if focus.is_focused() => {
                open.toggle();
            }
            _ => {}
        };

        rect()
            .child(
                rect()
                    .a11y_id(focus.a11y_id())
                    .a11y_member_of(focus.a11y_id())
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
            .maybe_child(open().then(|| {
                rect().height(Size::px(0.)).width(Size::px(0.)).child(
                    rect()
                        .width(Size::window_percent(100.))
                        .margin(Gaps::new(4., 0., 0., 0.))
                        .child(
                            rect()
                                .border(
                                    Border::new()
                                        .fill(theme.border_fill)
                                        .width(1.)
                                        .alignment(BorderAlignment::Inner),
                                )
                                .overflow_mode(OverflowMode::Clip)
                                .corner_radius(8.)
                                .background(theme.dropdown_background)
                                // TODO: Shadows
                                .padding(6.)
                                .content(Content::Fit)
                                .children(self.children.clone()),
                        ),
                )
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
