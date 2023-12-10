#[doc(hidden)]
pub use ::core::default::Default;
#[doc(hidden)]
pub use ::paste::paste;
#[doc(hidden)]
pub use ::std::borrow::Cow;
use dioxus_core::ScopeState;
use dioxus_hooks::{use_shared_state, use_shared_state_provider, UseSharedState};

/// Provide a custom [`Theme`].
pub fn use_init_theme(cx: &ScopeState, theme: Theme) {
    use_shared_state_provider(cx, || theme);
}

/// Provide the default [`Theme`].
pub fn use_init_default_theme(cx: &ScopeState) {
    use_shared_state_provider(cx, Theme::default);
}

/// Subscribe to [`Theme`] changes.
pub fn use_theme(cx: &ScopeState) -> &UseSharedState<Theme> {
    use_shared_state::<Theme>(cx).unwrap()
}

/// Subscribe to [`Theme`] changes, default theme will be used if there is no provided [`Theme`].
///
/// Primarily used by built-in components that have no control of whether they will inherit a [`Theme`] or not.
pub fn use_get_theme(cx: &ScopeState) -> Theme {
    use_shared_state::<Theme>(cx)
        .map(|v| v.read().clone())
        .unwrap_or_default()
}

/// Example usage:
///
/// ```rust,ignore
/// define_theme! {
///     %[component]
///     pub Test<'a> {
///         %[cows]
///         cow_string: str,
///         %[borrowed]
///         borrowed_data: &'a Foo,
///         %[owned]
///         owned_data: Bar,
///         %[subthemes],
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_theme {
    (NOTHING=) => {};

    (
        $(#[$attrs:meta])*
        $(%[component$($component_attr_control:tt)?])?
        $vis:vis $name:ident $(<$lifetime:lifetime>)? {
            $(
                %[cows$($cows_attr_control:tt)?]
                $(
                    $(#[$cow_field_attrs:meta])*
                    $cow_field_name:ident: $cow_field_ty:ty,
                )*
            )?
            $(
                %[borrowed$($borrowed_attr_control:tt)?]
                $(
                    $(#[$borrowed_field_attrs:meta])*
                    $borrowed_field_name:ident: $borrowed_field_ty:ty,
                )*
            )?
            $(
                %[owned$($owned_attr_control:tt)?]
                $(
                    $(#[$owned_field_attrs:meta])*
                    $owned_field_name:ident: $owned_field_ty:ty,
                )*
            )?
            $(
                %[subthemes$($subthemes_attr_control:tt)?]
                $(
                    $(#[$subtheme_field_attrs:meta])*
                    $subtheme_field_name:ident: $subtheme_field_ty_name:ident $(<$subtheme_field_ty_lifetime:lifetime>)?,
                )*
            )?
    }) => {
        $crate::define_theme!(NOTHING=$($($component_attr_control)?)?);
        $crate::define_theme!(NOTHING=$($($cows_attr_control)?)?);
        $crate::define_theme!(NOTHING=$($($borrowed_attr_control)?)?);
        $crate::define_theme!(NOTHING=$($($owned_attr_control)?)?);
        $crate::define_theme!(NOTHING=$($($subthemes_attr_control)?)?);
        $crate::paste! {
            #[derive(Default, Clone, Debug, PartialEq, Eq)]
            #[doc = "You can use this to change a theme for only one component, with the `theme` property."]
            $(#[$attrs])*
            $vis struct [<$name ThemeWith>] $(<$lifetime>)? {
                $($(
                    $(#[$borrowed_field_attrs])*
                    pub $borrowed_field_name: Option<$borrowed_field_ty>,
                )*)?
                $($(
                    $(#[$owned_field_attrs])*
                    pub $owned_field_name: Option<$owned_field_ty>,
                )*)?
                $($(
                    $(#[$subtheme_field_attrs])*
                    pub $subtheme_field_name: Option< [<$subtheme_field_ty_name With>] $(<$subtheme_field_ty_lifetime>)? >,
                )*)?
                $($(
                    $(#[$cow_field_attrs])*
                    pub $cow_field_name: Option<$crate::Cow<'static, $cow_field_ty>>,
                )*)?
            }

            #[derive(Clone, Debug, PartialEq, Eq)]
            $(#[doc = "Theming properties for the `" $name "` component."] $($component_attr_control)?)?
            $(#[$attrs])*
            $vis struct [<$name Theme>] $(<$lifetime>)? {
                $($(
                    $(#[$borrowed_field_attrs])*
                    pub $borrowed_field_name: $borrowed_field_ty,
                )*)?
                $($(
                    $(#[$owned_field_attrs])*
                    pub $owned_field_name: $owned_field_ty,
                )*)?
                $($(
                    $(#[$subtheme_field_attrs])*
                    pub $subtheme_field_name: $subtheme_field_ty_name $(<$subtheme_field_ty_lifetime>)?,
                )*)?
                $($(
                    $(#[$cow_field_attrs])*
                    pub $cow_field_name: $crate::Cow<'static, $cow_field_ty>,
                )*)?
            }

            impl $(<$lifetime>)? [<$name Theme>] $(<$lifetime>)? {
                #[doc = "Checks each field in `optional` and if it's `Some`, it overwrites the corresponding `self` field."]
                pub fn apply_optional(&mut self, optional: & $($lifetime)? [<$name ThemeWith>]) {
                    $($(
                        if let Some($borrowed_field_name) = optional.$borrowed_field_name {
                            self.$borrowed_field_name = $borrowed_field_name;
                        }
                    )*)?

                    $($(
                        if let Some($owned_field_name) = &optional.$owned_field_name {
                            self.$owned_field_name = $owned_field_name.clone();
                        }
                    )*)?

                    $($(
                        if let Some($subtheme_field_name) = &optional.$subtheme_field_name {
                            self.$subtheme_field_name.apply_optional($subtheme_field_name);
                        }
                    )*)?

                    $($(
                        if let Some($cow_field_name) = &optional.$cow_field_name {
                            self.$cow_field_name = $cow_field_name.clone();
                        }
                    )*)?
                }
            }
        }
    };
}

/// Create `FooThemeWith` structs without having to deal with the verbose syntax.
///
/// # Examples
///
/// Without the macro:
///
/// ```no_run
/// # use dioxus::prelude::*;
/// # use freya::prelude::*;
/// # fn theme_with_example_no_macro(cx: Scope) -> Element {
/// render! {
///     Button {
///         theme: ButtonThemeWith {
///             background: "blue".into(),
///             font_theme: FontThemeWith {
///                 color: "white".into(),
///                 ..Default::default()
///             }.into(),
///             ..Default::default()
///         }
///     }
/// }
/// # }
/// ```
///
/// With the macro:
///
/// ```no_run
/// # use dioxus::prelude::*;
/// # use freya::prelude::*;
/// # fn theme_with_example_no_macro(cx: Scope) -> Element {
/// render! {
///     Button {
///         theme: theme_with!(ButtonTheme {
///             background: "blue",
///             font_theme: theme_with!(FontTheme {
///                 color: "white",
///             }),
///         })
///     }
/// }
/// # }
/// ```
#[macro_export]
macro_rules! theme_with {
    ($theme_name:ident {
        $(
            $theme_field_name:ident: $theme_field_val:expr,
        )*
    }) => {
        $crate::paste! {
            #[allow(clippy::needless_update)]
            [<$theme_name With>] {
                $($theme_field_name: Some($theme_field_val),)*
                ..$crate::Default::default()
            }
        }
    };
}

define_theme! {
    %[component]
    pub Dropdown {
        %[cows]
        dropdown_background: str,
        background_button: str,
        hover_background: str,
        border_fill: str,
        arrow_fill: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub DropdownItem {
        %[cows]
        background: str,
        select_background: str,
        hover_background: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub Button {
        %[cows]
        background: str,
        hover_background: str,
        border_fill: str,
        margin: str,
        corner_radius: str,
        width: str,
        height: str,
        padding: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub Input {
        %[cows]
        background: str,
        hover_background: str,
        border_fill: str,
        width: str,
        margin: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    /// Theming properties for Fonts.
    pub Font {
        %[cows]
        color: str,
    }
}

define_theme! {
    %[component]
    pub Switch {
        %[cows]
        background: str,
        thumb_background: str,
        enabled_background: str,
        enabled_thumb_background: str,
    }
}

define_theme! {
    %[component]
    pub ScrollBar {
        %[cows]
        background: str,
        thumb_background: str,
        hover_thumb_background: str,
        active_thumb_background: str,
        offset_x: str,
        offset_y: str,
    }
}

define_theme! {
    /// Also used by `VirtualScrollView`.
    %[component]
    pub ScrollView {
        %[cows]
        height: str,
        width: str,
        padding: str,
    }
}

define_theme! {
    %[component]
    pub Body {
        %[cows]
        background: str,
        color: str,
        padding: str,
    }
}

define_theme! {
    %[component]
    pub Slider {
        %[cows]
        background: str,
        thumb_background: str,
        thumb_inner_background: str,
    }
}

define_theme! {
    %[component]
    pub Tooltip {
        %[cows]
        background: str,
        color: str,
        border_fill: str,
    }
}

define_theme! {
    %[component]
    pub ExternalLink {
        %[cows]
        highlight_color: str,
    }
}

define_theme! {
    %[component]
    pub Accordion {
        %[cows]
        color: str,
        background: str,
        border_fill: str,
    }
}

define_theme! {
    %[component]
    pub Loader {
        %[cows]
        primary_color: str,
        secondary_color: str,
    }
}

define_theme! {
    %[component]
    pub ProgressBar {
        %[cows]
        color: str,
        background: str,
        progress_background: str,
        width: str,
        height: str,
    }
}

define_theme! {
    %[component]
    pub Table {
        %[cows]
        background: str,
        arrow_fill: str,
        alternate_row_background: str,
        row_background: str,
        divider_fill: str,
        height: str,
        corner_radius: str,
        shadow: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub Canvas {
        %[cows]
        width: str,
        height: str,
        background: str,
    }
}

define_theme! {
    %[component]
    pub Graph {
        %[cows]
        width: str,
        height: str,
    }
}

define_theme! {
    %[component]
    pub NetworkImage {
        %[cows]
        width: str,
        height: str,
    }
}

define_theme! {
    %[component]
    pub ArrowIcon {
        %[cows]
        margin: str,
        width: str,
        height: str,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    pub name: &'static str,
    pub body: BodyTheme,
    pub button: ButtonTheme,
    pub switch: SwitchTheme,
    pub scroll_bar: ScrollBarTheme,
    pub scroll_view: ScrollViewTheme,
    pub slider: SliderTheme,
    pub tooltip: TooltipTheme,
    pub external_link: ExternalLinkTheme,
    pub dropdown: DropdownTheme,
    pub dropdown_item: DropdownItemTheme,
    pub accordion: AccordionTheme,
    pub loader: LoaderTheme,
    pub progress_bar: ProgressBarTheme,
    pub table: TableTheme,
    pub input: InputTheme,
    pub canvas: CanvasTheme,
    pub graph: GraphTheme,
    pub network_image: NetworkImageTheme,
    pub arrow_icon: ArrowIconTheme,
}

impl Default for Theme {
    fn default() -> Self {
        LIGHT_THEME
    }
}

/// Alias for `Cow::Borrowed`, because that's used a million times so shortening it is nice.
/// Makes the code more readable.
macro_rules! cb {
    ($val:expr) => {
        $crate::Cow::Borrowed($val)
    };
}

pub const LIGHT_THEME: Theme = Theme {
    name: "light",
    body: BodyTheme {
        background: cb!("white"),
        color: cb!("black"),
        padding: cb!("none"),
    },
    slider: SliderTheme {
        background: cb!("rgb(210, 210, 210)"),
        thumb_background: cb!("rgb(210, 210, 210)"),
        thumb_inner_background: cb!("rgb(103, 80, 164)"),
    },
    button: ButtonTheme {
        background: cb!("rgb(245, 245, 245)"),
        hover_background: cb!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cb!("rgb(10, 10, 10)"),
        },
        border_fill: cb!("rgb(210, 210, 210)"),
        padding: cb!("8 16"),
        margin: cb!("4"),
        corner_radius: cb!("8"),
        width: cb!("auto"),
        height: cb!("auto"),
    },
    input: InputTheme {
        background: cb!("rgb(245, 245, 245)"),
        hover_background: cb!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cb!("rgb(10, 10, 10)"),
        },
        border_fill: cb!("rgb(210, 210, 210)"),
        width: cb!("150"),
        margin: cb!("4"),
    },
    switch: SwitchTheme {
        background: cb!("rgb(121, 116, 126)"),
        thumb_background: cb!("rgb(231, 224, 236)"),
        enabled_background: cb!("rgb(103, 80, 164)"),
        enabled_thumb_background: cb!("rgb(234, 221, 255)"),
    },
    scroll_bar: ScrollBarTheme {
        background: cb!("rgb(225, 225, 225)"),
        thumb_background: cb!("rgb(135, 135, 135)"),
        hover_thumb_background: cb!("rgb(115, 115, 115)"),
        active_thumb_background: cb!("rgb(95, 95, 95)"),
        offset_x: cb!("0"),
        offset_y: cb!("0"),
    },
    scroll_view: ScrollViewTheme {
        height: cb!("100%"),
        width: cb!("100%"),
        padding: cb!("0"),
    },
    tooltip: TooltipTheme {
        background: cb!("rgb(245, 245, 245)"),
        color: cb!("rgb(25,25,25)"),
        border_fill: cb!("rgb(210, 210, 210)"),
    },
    external_link: ExternalLinkTheme {
        highlight_color: cb!("rgb(43,106,208)"),
    },
    dropdown: DropdownTheme {
        dropdown_background: cb!("white"),
        background_button: cb!("rgb(245, 245, 245)"),
        hover_background: cb!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cb!("rgb(10, 10, 10)"),
        },
        border_fill: cb!("rgb(210, 210, 210)"),
        arrow_fill: cb!("rgb(40, 40, 40)"),
    },
    dropdown_item: DropdownItemTheme {
        background: cb!("white"),
        select_background: cb!("rgb(240, 240, 240)"),
        hover_background: cb!("rgb(220, 220, 220)"),
        font_theme: FontTheme {
            color: cb!("rgb(10, 10, 10)"),
        },
    },
    accordion: AccordionTheme {
        color: cb!("black"),
        background: cb!("rgb(245, 245, 245)"),
        border_fill: cb!("rgb(210, 210, 210)"),
    },
    loader: LoaderTheme {
        primary_color: cb!("rgb(50, 50, 50)"),
        secondary_color: cb!("rgb(150, 150, 150)"),
    },
    progress_bar: ProgressBarTheme {
        color: cb!("white"),
        background: cb!("rgb(210, 210, 210)"),
        progress_background: cb!("rgb(103, 80, 164)"),
        width: cb!("100%"),
        height: cb!("20"),
    },
    table: TableTheme {
        font_theme: FontTheme {
            color: cb!("black"),
        },
        background: cb!("white"),
        arrow_fill: cb!("rgb(40, 40, 40)"),
        row_background: cb!("transparent"),
        alternate_row_background: cb!("rgb(240, 240, 240)"),
        divider_fill: cb!("rgb(200, 200, 200)"),
        height: cb!("auto"),
        corner_radius: cb!("6"),
        shadow: cb!("0 2 15 5 rgb(35, 35, 35, 70)"),
    },
    canvas: CanvasTheme {
        width: cb!("300"),
        height: cb!("150"),
        background: cb!("white"),
    },
    graph: GraphTheme {
        width: cb!("100%"),
        height: cb!("100%"),
    },
    network_image: NetworkImageTheme {
        width: cb!("100%"),
        height: cb!("100%"),
    },
    arrow_icon: ArrowIconTheme {
        width: cb!("10"),
        height: cb!("10"),
        margin: cb!("none"),
    },
};

pub const DARK_THEME: Theme = Theme {
    name: "dark",
    body: BodyTheme {
        background: cb!("rgb(25, 25, 25)"),
        color: cb!("white"),
        padding: LIGHT_THEME.body.padding,
    },
    slider: SliderTheme {
        background: cb!("rgb(60, 60, 60)"),
        thumb_background: cb!("rgb(60, 60, 60)"),
        thumb_inner_background: cb!("rgb(255, 95, 0)"),
    },
    button: ButtonTheme {
        background: cb!("rgb(35, 35, 35)"),
        hover_background: cb!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cb!("white"),
        },
        border_fill: cb!("rgb(80, 80, 80)"),
        padding: LIGHT_THEME.button.padding,
        margin: LIGHT_THEME.button.margin,
        corner_radius: LIGHT_THEME.button.corner_radius,
        width: LIGHT_THEME.button.width,
        height: LIGHT_THEME.button.height,
    },
    input: InputTheme {
        background: cb!("rgb(35, 35, 35)"),
        hover_background: cb!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cb!("white"),
        },
        border_fill: cb!("rgb(80, 80, 80)"),
        width: LIGHT_THEME.input.width,
        margin: LIGHT_THEME.input.margin,
    },
    switch: SwitchTheme {
        background: cb!("rgb(60, 60, 60)"),
        thumb_background: cb!("rgb(200, 200, 200)"),
        enabled_background: cb!("rgb(255, 95, 0)"),
        enabled_thumb_background: cb!("rgb(234, 221, 255)"),
    },
    scroll_bar: ScrollBarTheme {
        background: cb!("rgb(35, 35, 35)"),
        thumb_background: cb!("rgb(100, 100, 100)"),
        hover_thumb_background: cb!("rgb(120, 120, 120)"),
        active_thumb_background: cb!("rgb(140, 140, 140)"),
        offset_x: LIGHT_THEME.scroll_bar.offset_x,
        offset_y: LIGHT_THEME.scroll_bar.offset_y,
    },
    scroll_view: ScrollViewTheme {
        height: LIGHT_THEME.scroll_view.height,
        width: LIGHT_THEME.scroll_view.width,
        padding: LIGHT_THEME.scroll_view.padding,
    },
    tooltip: TooltipTheme {
        background: cb!("rgb(35,35,35)"),
        color: cb!("rgb(240,240,240)"),
        border_fill: cb!("rgb(80, 80, 80)"),
    },
    external_link: ExternalLinkTheme {
        highlight_color: cb!("rgb(43,106,208)"),
    },
    dropdown: DropdownTheme {
        dropdown_background: cb!("rgb(25, 25, 25)"),
        background_button: cb!("rgb(35, 35, 35)"),
        hover_background: cb!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cb!("white"),
        },
        border_fill: cb!("rgb(80, 80, 80)"),
        arrow_fill: cb!("rgb(40, 40, 40)"),
    },
    dropdown_item: DropdownItemTheme {
        background: cb!("rgb(35, 35, 35)"),
        select_background: cb!("rgb(80, 80, 80)"),
        hover_background: cb!("rgb(55, 55, 55)"),
        font_theme: FontTheme {
            color: cb!("white"),
        },
    },
    accordion: AccordionTheme {
        color: cb!("white"),
        background: cb!("rgb(60, 60, 60)"),
        border_fill: cb!("rgb(80, 80, 80)"),
    },
    loader: LoaderTheme {
        primary_color: cb!("rgb(150, 150, 150)"),
        secondary_color: cb!("rgb(255, 255, 255)"),
    },
    progress_bar: ProgressBarTheme {
        color: cb!("white"),
        background: cb!("rgb(60, 60, 60)"),
        progress_background: cb!("rgb(255, 95, 0)"),
        width: LIGHT_THEME.progress_bar.width,
        height: LIGHT_THEME.progress_bar.height,
    },
    table: TableTheme {
        font_theme: FontTheme {
            color: cb!("white"),
        },
        background: cb!("rgb(25, 25, 25)"),
        arrow_fill: cb!("rgb(150, 150, 150)"),
        row_background: cb!("transparent"),
        alternate_row_background: cb!("rgb(50, 50, 50)"),
        divider_fill: cb!("rgb(100, 100, 100)"),
        height: LIGHT_THEME.table.height,
        corner_radius: LIGHT_THEME.table.corner_radius,
        shadow: LIGHT_THEME.table.shadow,
    },
    canvas: CanvasTheme {
        width: LIGHT_THEME.canvas.width,
        height: LIGHT_THEME.canvas.height,
        background: cb!("white"),
    },
    graph: GraphTheme {
        width: LIGHT_THEME.graph.width,
        height: LIGHT_THEME.graph.height,
    },
    network_image: NetworkImageTheme {
        width: LIGHT_THEME.network_image.width,
        height: LIGHT_THEME.network_image.height,
    },
    arrow_icon: ArrowIconTheme {
        width: LIGHT_THEME.arrow_icon.width,
        height: LIGHT_THEME.arrow_icon.height,
        margin: LIGHT_THEME.arrow_icon.margin,
    },
};
