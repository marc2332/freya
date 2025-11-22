use freya_core::prelude::*;
use torin::size::Size;

use crate::{
    activable_route_context::use_activable_route,
    get_theme,
    scrollviews::ScrollView,
    theming::component_themes::{
        SideBarItemTheme,
        SideBarItemThemePartial,
        SideBarTheme,
        SideBarThemePartial,
    },
};

#[derive(PartialEq)]
pub struct SideBar {
    /// Theme override.
    pub(crate) theme: Option<SideBarThemePartial>,
    /// This is what is rendered next to the sidebar.
    content: Option<Element>,
    /// This is what is rendered in the sidebar.
    bar: Option<Element>,
    /// Width of the sidebar.
    width: Size,
}

impl Default for SideBar {
    fn default() -> Self {
        Self::new()
    }
}

impl SideBar {
    pub fn new() -> Self {
        Self {
            theme: None,
            content: None,
            bar: None,
            width: Size::px(180.),
        }
    }

    pub fn content(mut self, content: impl Into<Element>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn bar(mut self, bar: impl Into<Element>) -> Self {
        self.bar = Some(bar.into());
        self
    }

    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.width = width.into();
        self
    }
}

impl Render for SideBar {
    fn render(&self) -> impl IntoElement {
        let SideBarTheme {
            spacing,
            padding,
            background,
            color,
        } = get_theme!(&self.theme, sidebar);

        rect()
            .horizontal()
            .width(Size::fill())
            .height(Size::fill())
            .color(color)
            .child(
                rect()
                    .overflow_mode(OverflowMode::Clip)
                    .width(self.width.clone())
                    .height(Size::fill())
                    .background(background)
                    .child(
                        ScrollView::new()
                            .width(self.width.clone())
                            .spacing(spacing)
                            .child(rect().padding(padding).maybe_child(self.bar.clone())),
                    ),
            )
            .child(
                rect()
                    .overflow_mode(OverflowMode::Clip)
                    .expanded()
                    .maybe_child(self.content.clone()),
            )
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ButtonStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the button.
    Hovering,
}
#[derive(PartialEq)]
pub struct SideBarItem {
    /// Theme override.
    pub(crate) theme: Option<SideBarItemThemePartial>,
    /// Inner child for the [SideBarItem].
    children: Vec<Element>,
    /// Optionally handle the `on_press` event in the [SideBarItem].
    on_press: Option<EventHandler<Event<PressEventData>>>,
    /// Optionally specify a custom `overflow` attribute for this component. Defaults to [OverflowMode::Clip].
    overflow_mode: OverflowMode,
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
            overflow_mode: OverflowMode::Clip,
        }
    }

    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }

    pub fn overflow_mode(mut self, overflow_mode: impl Into<OverflowMode>) -> Self {
        self.overflow_mode = overflow_mode.into();
        self
    }
}

impl Render for SideBarItem {
    fn render(&self) -> impl IntoElement {
        let SideBarItemTheme {
            margin,
            hover_background,
            background,
            corner_radius,
            padding,
            color,
        } = get_theme!(&self.theme, sidebar_item);
        let mut status = use_state(ButtonStatus::default);
        let is_active = use_activable_route();

        let on_pointer_enter = move |_| {
            status.set(ButtonStatus::Hovering);
        };

        let on_pointer_leave = move |_| {
            status.set(ButtonStatus::default());
        };

        let background = match *status.read() {
            _ if is_active => hover_background,
            ButtonStatus::Hovering => hover_background,
            ButtonStatus::Idle => background,
        };

        // TODO: a11y
        rect()
            .map(self.on_press.clone(), |rect, on_press| {
                rect.on_press(on_press)
            })
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .overflow_mode(self.overflow_mode)
            .width(Size::fill())
            .margin(margin)
            .padding(padding)
            .color(color)
            .background(background)
            .corner_radius(corner_radius)
            .children(self.children.clone())
    }
}
