use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    size::Size,
};

use crate::{
    activable_route_context::use_activable_route,
    define_theme,
    get_theme,
};

define_theme! {
    %[component]
    pub FloatingTab {
        %[fields]
        background: Color,
        hover_background: Color,
        width: Size,
        height: Size,
        padding: Gaps,
        color: Color,
        corner_radius: CornerRadius,
    }
}

/// Current status of the Tab.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TabStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the Tab.
    Hovering,
}

#[derive(Clone, PartialEq)]
pub struct FloatingTab {
    pub(crate) theme: Option<FloatingTabThemePartial>,
    children: Vec<Element>,
    /// Optionally handle the `on_press` event in [FloatingTab].
    on_press: Option<EventHandler<Event<PressEventData>>>,
    key: DiffKey,
}

impl KeyExt for FloatingTab {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Default for FloatingTab {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for FloatingTab {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

/// Floating Tab component.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     rect()
///         .spacing(8.)
///         .child(FloatingTab::new().child("Page 1"))
///         .child(FloatingTab::new().child("Page 2"))
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_floating_tab.png").with_hook(|t| { t.move_cursor((125., 115.)); t.sync_and_update(); }).with_scale_factor(1.).render();
/// ```
///
/// # Preview
/// ![FloatingTab Preview][floating_tab]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("floating_tab", "images/gallery_floating_tab.png")
)]
impl FloatingTab {
    pub fn new() -> Self {
        Self {
            children: vec![],
            theme: None,
            on_press: None,
            key: DiffKey::None,
        }
    }

    /// Get the theme override for this component.
    pub fn get_theme(&self) -> Option<&FloatingTabThemePartial> {
        self.theme.as_ref()
    }

    /// Set a theme override for this component.
    pub fn theme(mut self, theme: FloatingTabThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Component for FloatingTab {
    fn render(&self) -> impl IntoElement {
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let mut status = use_state(TabStatus::default);
        let is_active = use_activable_route();

        let FloatingTabTheme {
            background,
            hover_background,
            padding,
            width,
            height,
            color,
            corner_radius,
        } = get_theme!(&self.theme, FloatingTabThemePreference, "floating_tab");

        let on_pointer_enter = move |_| {
            Cursor::set(CursorIcon::Pointer);
            status.set(TabStatus::Hovering);
        };

        let on_pointer_leave = move |_| {
            Cursor::set(CursorIcon::default());
            status.set(TabStatus::default());
        };

        let background = if focus_status() == FocusStatus::Keyboard
            || is_active
            || *status.read() == TabStatus::Hovering
        {
            hover_background
        } else {
            background
        };

        rect()
            .a11y_id(focus.a11y_id())
            .a11y_focusable(Focusable::Enabled)
            .a11y_role(AccessibilityRole::Tab)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .map(self.on_press.clone(), |el, on_press| el.on_press(on_press))
            .width(width)
            .height(height)
            .center()
            .overflow(Overflow::Clip)
            .padding(padding)
            .background(background)
            .color(color)
            .corner_radius(corner_radius)
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
