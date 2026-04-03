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
    pub SideBarItem {
        %[fields]
        color: Color,
        background: Color,
        hover_background: Color,
        active_background: Color,
        corner_radius: CornerRadius,
        margin: Gaps,
        padding: Gaps,
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SideBarItemStatus {
    /// Default state.
    #[default]
    Idle,
    /// User is hovering the sidebar item.
    Hovering,
}

/// Button designed for sidebars.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     rect()
///         .horizontal()
///         .child(
///             rect()
///                 .theme_background()
///                 .padding(8.)
///                 .width(Size::px(150.))
///                 .height(Size::fill())
///                 .child(SideBarItem::new().child("Home"))
///                 .child(SideBarItem::new().child("Settings")),
///         )
///         .child(rect().expanded().center().child("Main content"))
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(
/// #       app()
/// #   )
/// # }, "./images/gallery_sidebar.png")
/// # .with_hook(|t| { t.move_cursor((20., 20.)); t.sync_and_update(); })
/// # .with_scale_factor(0.75)
/// # .render();
/// ```
///
/// # Preview
/// ![SideBarItem Preview][SideBarItem]

#[derive(Clone, PartialEq)]
pub struct SideBarItem {
    /// Theme override.
    pub(crate) theme: Option<SideBarItemThemePartial>,
    /// Inner child for the [SideBarItem].
    children: Vec<Element>,
    /// Optionally handle the `on_press` event in the [SideBarItem].
    on_press: Option<EventHandler<Event<PressEventData>>>,
    /// Optionally specify a custom `overflow` attribute for this component. Defaults to [OverflowMode::Clip].
    overflow: Overflow,
    key: DiffKey,
}

impl KeyExt for SideBarItem {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Default for SideBarItem {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for SideBarItem {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl SideBarItem {
    pub fn new() -> Self {
        Self {
            theme: None,
            children: Vec::new(),
            on_press: None,
            overflow: Overflow::Clip,
            key: DiffKey::None,
        }
    }

    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }

    pub fn overflow(mut self, overflow: impl Into<Overflow>) -> Self {
        self.overflow = overflow.into();
        self
    }

    /// Get the theme override for this component.
    pub fn get_theme(&self) -> Option<&SideBarItemThemePartial> {
        self.theme.as_ref()
    }

    /// Set a theme override for this component.
    pub fn theme(mut self, theme: SideBarItemThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Component for SideBarItem {
    fn render(&self) -> impl IntoElement {
        let SideBarItemTheme {
            margin,
            hover_background,
            active_background,
            background,
            corner_radius,
            padding,
            color,
        } = get_theme!(&self.theme, SideBarItemThemePreference, "sidebar_item");
        let mut status = use_state(SideBarItemStatus::default);
        let is_active = use_activable_route();

        let on_pointer_enter = move |_| {
            status.set(SideBarItemStatus::Hovering);
        };

        let on_pointer_leave = move |_| {
            status.set(SideBarItemStatus::default());
        };

        let background = match *status.read() {
            _ if is_active => active_background,
            SideBarItemStatus::Hovering => hover_background,
            SideBarItemStatus::Idle => background,
        };

        rect()
            .a11y_focusable(true)
            .a11y_role(AccessibilityRole::Link)
            .map(self.on_press.clone(), |rect, on_press| {
                rect.on_press(on_press)
            })
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .overflow(self.overflow)
            .width(Size::fill())
            .margin(margin)
            .padding(padding)
            .color(color)
            .background(background)
            .corner_radius(corner_radius)
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
