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

#[doc(hidden)]
pub use ::paste::paste;

#[doc(hidden)]
pub use ::core::default::Default;

/// Example usage:
/// ```rust,ignore
/// define_theme! {
///     pub TestTheme {
///         ..borrowed..
///         borrowed_string: &'static str,
///         ..owned..
///         owned_string: String,
///         ..subthemes..
///         font_theme: FontTheme,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_theme {
    (
        $(#[$attrs:meta])*
        $vis:vis $name:ident {
        $(
            ..borrowed..
            $(
                $(#[$borrowed_field_attrs:meta])*
                $borrowed_field_name:ident: $borrowed_field_ty:ty,
            )*
        )?
        $(
            ..owned..
            $(
                $(#[$owned_field_attrs:meta])*
                $owned_field_name:ident: $owned_field_ty:ty,
            )*
        )?
        $(
            ..subthemes..
            $(
                $(#[$subtheme_field_attrs:meta])*
                $subtheme_field_name:ident: $subtheme_field_ty:ty,
            )*
        )?
    }) => {
        $crate::paste! {
            #[derive(Default, Clone, Debug, PartialEq, Eq)]
            $(#[$attrs])*
            #[doc = "You can use this to change a theme for only one component, with the `theme` property."]
            $vis struct [<$name With>] {
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
                    pub $subtheme_field_name: Option<[<$subtheme_field_ty With>]>,
                )*)?
            }

            #[derive(Clone, Debug, PartialEq, Eq)]
            $(#[$attrs])*
            $vis struct $name {
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
                    pub $subtheme_field_name: $subtheme_field_ty,
                )*)?
            }

            impl $name {
                pub fn apply_optional(&mut self, optional: &[<$name With>]) {
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
            [<$theme_name With>] {
                $($theme_field_name: Some($theme_field_val),)*
                ..$crate::Default::default()
            }
        }
    };
}

define_theme! {
    /// Theming properties for Dropdown components.
    pub DropdownTheme {
        ..borrowed..
        desplegable_background: &'static str,
        background_button: &'static str,
        hover_background: &'static str,
        border_fill: &'static str,
        arrow_fill: &'static str,
        ..subthemes..
        font_theme: FontTheme,
    }
}

define_theme! {
    /// Theming properties for DropdownItem components.
    pub DropdownItemTheme {
        ..borrowed..
        background: &'static str,
        select_background: &'static str,
        hover_background: &'static str,
        ..subthemes..
        font_theme: FontTheme,
    }
}

define_theme! {
    /// Theming properties for Button components.
    pub ButtonTheme {
        ..borrowed..
        background: &'static str,
        hover_background: &'static str,
        border_fill: &'static str,
        ..subthemes..
        font_theme: FontTheme,
    }
}

define_theme! {
    /// Theming properties for Input components.
    pub InputTheme {
        ..borrowed..
        background: &'static str,
        hover_background: &'static str,
        border_fill: &'static str,
        ..subthemes..
        font_theme: FontTheme,
    }
}

define_theme! {
    /// Theming properties for Fonts.
    pub FontTheme {
        ..borrowed..
        color: &'static str,
    }
}

define_theme! {
    /// Theming properties the Switch components.
    pub SwitchTheme {
        ..borrowed..
        background: &'static str,
        thumb_background: &'static str,
        enabled_background: &'static str,
        enabled_thumb_background: &'static str,
    }
}

define_theme! {
    /// Theming properties the Scrollbar components.
    pub ScrollbarTheme {
        ..borrowed..
        background: &'static str,
        thumb_background: &'static str,
        hover_thumb_background: &'static str,
        active_thumb_background: &'static str,
    }
}

define_theme! {
    /// Theming properties for the App body.
    pub BodyTheme {
        ..borrowed..
        background: &'static str,
        color: &'static str,
    }
}

define_theme! {
    /// Theming properties for Slider components.
    pub SliderTheme {
        ..borrowed..
        background: &'static str,
        thumb_background: &'static str,
        thumb_inner_background: &'static str,
    }
}

define_theme! {
    /// Theming properties for Tooltip components.
    pub TooltipTheme {
        ..borrowed..
        background: &'static str,
        color: &'static str,
        border_fill: &'static str,
    }
}

define_theme! {
    /// Theming properties for ExternalLink components.
    pub ExternalLinkTheme {
        ..borrowed..
        highlight_color: &'static str,
    }
}

define_theme! {
    /// Theming properties for Accordion component.
    pub AccordionTheme {
        ..borrowed..
        color: &'static str,
        background: &'static str,
        border_fill: &'static str,
    }
}

define_theme! {
    /// Theming properties for Loader component.
    pub LoaderTheme {
        ..borrowed..
        primary_color: &'static str,
        secondary_color: &'static str,
    }
}

define_theme! {
    /// Theming properties for ProgressBar component.
    pub ProgressBarTheme {
        ..borrowed..
        color: &'static str,
        background: &'static str,
        progress_background: &'static str,
    }
}

define_theme! {
    /// Theming properties for Table component.
    pub TableTheme {
        ..borrowed..
        background: &'static str,
        arrow_fill: &'static str,
        alternate_row_background: &'static str,
        row_background: &'static str,
        divider_fill: &'static str,
        ..subthemes..
        font_theme: FontTheme,
    }
}

/// Theming properties for Themes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    pub name: &'static str,
    pub body: BodyTheme,
    pub button: ButtonTheme,
    pub switch: SwitchTheme,
    pub scrollbar: ScrollbarTheme,
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
}

impl Default for Theme {
    fn default() -> Self {
        LIGHT_THEME
    }
}

/// `Light` theme
pub const LIGHT_THEME: Theme = Theme {
    name: "light",
    body: BodyTheme {
        background: "white",
        color: "black",
    },
    slider: SliderTheme {
        background: "rgb(210, 210, 210)",
        thumb_background: "rgb(210, 210, 210)",
        thumb_inner_background: "rgb(103, 80, 164)",
    },
    button: ButtonTheme {
        background: "rgb(245, 245, 245)",
        hover_background: "rgb(235, 235, 235)",
        font_theme: FontTheme {
            color: "rgb(10, 10, 10)",
        },
        border_fill: "rgb(210, 210, 210)",
    },
    input: InputTheme {
        background: "rgb(245, 245, 245)",
        hover_background: "rgb(235, 235, 235)",
        font_theme: FontTheme {
            color: "rgb(10, 10, 10)",
        },
        border_fill: "rgb(210, 210, 210)",
    },
    switch: SwitchTheme {
        background: "rgb(121, 116, 126)",
        thumb_background: "rgb(231, 224, 236)",
        enabled_background: "rgb(103, 80, 164)",
        enabled_thumb_background: "rgb(234, 221, 255)",
    },
    scrollbar: ScrollbarTheme {
        background: "rgb(225, 225, 225)",
        thumb_background: "rgb(135, 135, 135)",
        hover_thumb_background: "rgb(115, 115, 115)",
        active_thumb_background: "rgb(95, 95, 95)",
    },
    tooltip: TooltipTheme {
        background: "rgb(245, 245, 245)",
        color: "rgb(25,25,25)",
        border_fill: "rgb(210, 210, 210)",
    },
    external_link: ExternalLinkTheme {
        highlight_color: "rgb(43,106,208)",
    },
    dropdown: DropdownTheme {
        desplegable_background: "white",
        background_button: "rgb(245, 245, 245)",
        hover_background: "rgb(235, 235, 235)",
        font_theme: FontTheme {
            color: "rgb(10, 10, 10)",
        },
        border_fill: "rgb(210, 210, 210)",
        arrow_fill: "rgb(40, 40, 40)",
    },
    dropdown_item: DropdownItemTheme {
        background: "white",
        select_background: "rgb(240, 240, 240)",
        hover_background: "rgb(220, 220, 220)",
        font_theme: FontTheme {
            color: "rgb(10, 10, 10)",
        },
    },
    accordion: AccordionTheme {
        color: "black",
        background: "rgb(245, 245, 245)",
        border_fill: "rgb(210, 210, 210)",
    },
    loader: LoaderTheme {
        primary_color: "rgb(50, 50, 50)",
        secondary_color: "rgb(150, 150, 150)",
    },
    progress_bar: ProgressBarTheme {
        color: "white",
        background: "rgb(210, 210, 210)",
        progress_background: "rgb(103, 80, 164)",
    },
    table: TableTheme {
        font_theme: FontTheme { color: "black" },
        background: "white",
        arrow_fill: "rgb(40, 40, 40)",
        row_background: "transparent",
        alternate_row_background: "rgb(240, 240, 240)",
        divider_fill: "rgb(200, 200, 200)",
    },
};

/// `Dark` theme
pub const DARK_THEME: Theme = Theme {
    name: "dark",
    body: BodyTheme {
        background: "rgb(25, 25, 25)",
        color: "white",
    },
    slider: SliderTheme {
        background: "rgb(60, 60, 60)",
        thumb_background: "rgb(60, 60, 60)",
        thumb_inner_background: "rgb(255, 95, 0)",
    },
    button: ButtonTheme {
        background: "rgb(35, 35, 35)",
        hover_background: "rgb(45, 45, 45)",
        font_theme: FontTheme { color: "white" },
        border_fill: "rgb(80, 80, 80)",
    },
    input: InputTheme {
        background: "rgb(35, 35, 35)",
        hover_background: "rgb(45, 45, 45)",
        font_theme: FontTheme { color: "white" },
        border_fill: "rgb(80, 80, 80)",
    },
    switch: SwitchTheme {
        background: "rgb(60, 60, 60)",
        thumb_background: "rgb(200, 200, 200)",
        enabled_background: "rgb(255, 95, 0)",
        enabled_thumb_background: "rgb(234, 221, 255)",
    },
    scrollbar: ScrollbarTheme {
        background: "rgb(35, 35, 35)",
        thumb_background: "rgb(100, 100, 100)",
        hover_thumb_background: "rgb(120, 120, 120)",
        active_thumb_background: "rgb(140, 140, 140)",
    },
    tooltip: TooltipTheme {
        background: "rgb(35,35,35)",
        color: "rgb(240,240,240)",
        border_fill: "rgb(80, 80, 80)",
    },
    external_link: ExternalLinkTheme {
        highlight_color: "rgb(43,106,208)",
    },
    dropdown: DropdownTheme {
        desplegable_background: "rgb(25, 25, 25)",
        background_button: "rgb(35, 35, 35)",
        hover_background: "rgb(45, 45, 45)",
        font_theme: FontTheme { color: "white" },
        border_fill: "rgb(80, 80, 80)",
        arrow_fill: "rgb(40, 40, 40)",
    },
    dropdown_item: DropdownItemTheme {
        background: "rgb(35, 35, 35)",
        select_background: "rgb(80, 80, 80)",
        hover_background: "rgb(55, 55, 55)",
        font_theme: FontTheme { color: "white" },
    },
    accordion: AccordionTheme {
        color: "white",
        background: "rgb(60, 60, 60)",
        border_fill: "rgb(80, 80, 80)",
    },
    loader: LoaderTheme {
        primary_color: "rgb(150, 150, 150)",
        secondary_color: "rgb(255, 255, 255)",
    },
    progress_bar: ProgressBarTheme {
        color: "white",
        background: "rgb(60, 60, 60)",
        progress_background: "rgb(255, 95, 0)",
    },
    table: TableTheme {
        font_theme: FontTheme { color: "white" },
        background: "rgb(25, 25, 25)",
        arrow_fill: "rgb(150, 150, 150)",
        row_background: "transparent",
        alternate_row_background: "rgb(50, 50, 50)",
        divider_fill: "rgb(100, 100, 100)",
    },
};
