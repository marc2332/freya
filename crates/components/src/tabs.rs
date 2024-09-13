use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    use_activable_route,
    use_applied_theme,
    use_focus,
    use_platform,
    BottomTabTheme,
    BottomTabThemeWith,
    TabTheme,
    TabThemeWith,
};
use winit::window::CursorIcon;

/// Horizontal container for Tabs. Use in combination with [`Tab`]
#[allow(non_snake_case)]
#[component]
pub fn Tabsbar(children: Element) -> Element {
    rsx!(
        rect {
            direction: "horizontal",
            {children}
        }
    )
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

///  Clickable Tab. Usually used in combination with [`Tabsbar`], [`Link`] and [`ActivableRoute`].
///
/// # Styling
/// Inherits the [`TabTheme`](freya_hooks::TabTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// # use dioxus_router::prelude::Routable;
/// # #[allow(non_snake_case)]
/// # fn PageNotFound() -> Element { None }
/// # #[allow(non_snake_case)]
/// # fn Settings() -> Element { None }
/// # #[derive(Routable, Clone, PartialEq)]
/// # #[rustfmt::skip]
/// # pub enum Route {
/// #     #[route("/settings")]
/// #     Settings,
/// #     #[route("/..route")]
/// #     PageNotFound { },
/// # }
/// fn app() -> Element {
///     rsx!(
///         Tabsbar {
///             Tab {
///                 label {
///                     "Home"
///                 }
///             }
///             Link {
///                 to: Route::Settings,
///                 Tab {
///                     label {
///                         "Go to Settings"
///                     }
///                 }
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn Tab(children: Element, theme: Option<TabThemeWith>) -> Element {
    let focus = use_focus();
    let mut status = use_signal(TabStatus::default);
    let platform = use_platform();
    let is_active = use_activable_route();

    let a11y_id = focus.attribute();

    let TabTheme {
        background,
        hover_background,
        border_fill,
        focus_border_fill,
        padding,
        width,
        height,
        font_theme,
    } = use_applied_theme!(&theme, tab);

    use_drop(move || {
        if *status.read() == TabStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(TabStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(TabStatus::default());
    };

    let background = match *status.read() {
        TabStatus::Hovering => hover_background,
        TabStatus::Idle => background,
    };
    let border = if focus.is_selected() || is_active {
        focus_border_fill
    } else {
        border_fill
    };

    rsx!(
        rect {
            onmouseenter,
            onmouseleave,
            a11y_id,
            width: "{width}",
            height: "{height}",
            a11y_focusable: "true",
            overflow: "clip",
            a11y_role:"tab",
            color: "{font_theme.color}",
            background: "{background}",
            text_align: "center",
            content: "fit",
            rect {
                padding: "{padding}",
                main_align: "center",
                cross_align: "center",
                {children},
            }
            rect {
                height: "2",
                width: "fill-min",
                background: "{border}"
            }
        }
    )
}

///  Clickable BottomTab. Same thing as Tab but designed to be placed in the bottom of your app,
///  usually used in combination with [`Tabsbar`], [`Link`] and [`ActivableRoute`].
///
/// # Styling
/// Inherits the [`BottomTabTheme`](freya_hooks::BottomTabTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// # use dioxus_router::prelude::Routable;
/// # #[allow(non_snake_case)]
/// # fn PageNotFound() -> Element { None }
/// # #[allow(non_snake_case)]
/// # fn Settings() -> Element { None }
/// # #[derive(Routable, Clone, PartialEq)]
/// # #[rustfmt::skip]
/// # pub enum Route {
/// #     #[route("/settings")]
/// #     Settings,
/// #     #[route("/..route")]
/// #     PageNotFound { },
/// # }
/// fn app() -> Element {
///     rsx!(
///         Tabsbar {
///             BottomTab {
///                 label {
///                     "Home"
///                 }
///             }
///             Link {
///                 to: Route::Settings,
///                 BottomTab {
///                     label {
///                         "Go to Settings"
///                     }
///                 }
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn BottomTab(children: Element, theme: Option<BottomTabThemeWith>) -> Element {
    let focus = use_focus();
    let mut status = use_signal(TabStatus::default);
    let platform = use_platform();
    let is_active = use_activable_route();

    let a11y_id = focus.attribute();

    let BottomTabTheme {
        background,
        hover_background,
        padding,
        width,
        height,
        font_theme,
    } = use_applied_theme!(&theme, bottom_tab);

    use_drop(move || {
        if *status.read() == TabStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(TabStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(TabStatus::default());
    };

    let background = match *status.read() {
        _ if focus.is_selected() || is_active => hover_background,
        TabStatus::Hovering => hover_background,
        TabStatus::Idle => background,
    };
    rsx!(
        rect {
            onmouseenter,
            onmouseleave,
            a11y_id,
            width: "{width}",
            height: "{height}",
            a11y_focusable: "true",
            overflow: "clip",
            a11y_role:"tab",
            color: "{font_theme.color}",
            background: "{background}",
            text_align: "center",
            padding: "{padding}",
            main_align: "center",
            cross_align: "center",
            corner_radius: "99",
            margin: "2 4",
            {children},
        }
    )
}
