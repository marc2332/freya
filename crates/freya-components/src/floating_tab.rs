use freya_core::prelude::*;

use crate::{
    activable_route_context::use_activable_route,
    get_theme,
    theming::component_themes::{
        FloatingTabTheme,
        FloatingTabThemePartial,
    },
};

/// Current status of the Tab.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TabStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the Tab.
    Hovering,
}

#[derive(PartialEq)]
pub struct FloatingTab {
    children: Vec<Element>,
    pub(crate) theme: Option<FloatingTabThemePartial>,
    /// Optionally handle the `onclick` event in the SidebarItem.
    on_press: Option<EventHandler<()>>,
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
/// # launch_doc_hook(|| {
/// #   rect().center().expanded().child(app())
/// # }, (250., 250.).into(), "./images/gallery_floating_tab.png", |t| {
/// #   t.move_cursor((125., 115.));
/// #   t.sync_and_update();
/// # });
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
        }
    }
}

impl Render for FloatingTab {
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
        } = get_theme!(&self.theme, floating_tab);

        let on_press = self.on_press.clone();

        let on_press = move |_| {
            if let Some(onpress) = &on_press {
                onpress.call(());
            }
        };

        let on_pointer_enter = move |_| {
            Cursor::set(CursorIcon::Pointer);
            status.set(TabStatus::Hovering);
        };

        let on_pointer_leave = move |_| {
            Cursor::set(CursorIcon::default());
            status.set(TabStatus::default());
        };

        let background = match *status.read() {
            _ if focus_status() == FocusStatus::Keyboard || is_active => hover_background,
            TabStatus::Hovering => hover_background,
            TabStatus::Idle => background,
        };

        rect()
            .a11y_id(focus.a11y_id())
            .a11y_focusable(Focusable::Enabled)
            .a11y_role(AccessibilityRole::Tab)
            .on_press(on_press)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .width(width)
            .height(height)
            .center()
            .overflow(Overflow::Clip)
            .padding(padding)
            .background(background)
            .color(color)
            .corner_radius(99.)
            .children(self.children.clone())
    }
}
