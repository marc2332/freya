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

macro_rules! define_theme {
    (
        $(#[$attrs:meta])*
        $vis:vis $name:ident {
        $(
            $(#[$field_attrs:meta])*
            $field_name:ident: $field_ty:ty,
        )*
        $(
            ..owned..
            $(
                $(#[$owned_field_attrs:meta])*
                $owned_field_name:ident: $owned_field_ty:ty,
            )*
        )?
    }) => {
        ::paste::paste! {
            #[derive(Default, Clone, Debug, PartialEq, Eq)]
            $(#[$attrs])*
            #[doc = "You can use this to change a theme for only one component, with the `theme` property."]
            $vis struct [<$name With>] {
                $(
                    $(#[$field_attrs])*
                    pub $field_name: Option<$field_ty>,
                )*
                $($(
                    $(#[$owned_field_attrs])*
                    pub $owned_field_name: Option<$owned_field_ty>,
                )*)?
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        $(#[$attrs])*
        $vis struct $name {
            $(
                $(#[$field_attrs])*
                pub $field_name: $field_ty,
            )*
            $($(
                $(#[$owned_field_attrs])*
                pub $owned_field_name: $owned_field_ty,
            )*)?
        }

        ::paste::paste! {
            impl $name {
                pub fn apply_optional(&mut self, optional: &[<$name With>]) {
                    $(
                        if let Some($field_name) = optional.$field_name {
                            self.$field_name = $field_name;
                        }
                    )*

                    $($(
                        if let Some($owned_field_name) = &optional.$owned_field_name {
                            self.$owned_field_name = $owned_field_name.clone();
                        }
                    )*)?
                }
            }
        }
    };
}

define_theme! {
    /// Theming properties for DropdownItem components.
    pub DropdownItemTheme {
        background: &'static str,
        select_background: &'static str,
        hover_background: &'static str,
        ..owned..
        font_theme: FontTheme,
    }
}

define_theme! {
    /// Theming properties for Dropdown components.
    pub DropdownTheme {
        desplegable_background: &'static str,
        background_button: &'static str,
        hover_background: &'static str,
        border_fill: &'static str,
        arrow_fill: &'static str,
        ..owned..
        font_theme: FontTheme,
    }
}

define_theme! {
    /// Theming properties for Button components.
    pub ButtonTheme {
        background: &'static str,
        hover_background: &'static str,
        border_fill: &'static str,
        ..owned..
        font_theme: FontTheme,
    }
}

define_theme! {
    /// Theming properties for Input components.
    pub InputTheme {
        background: &'static str,
        hover_background: &'static str,
        border_fill: &'static str,
        ..owned..
        font_theme: FontTheme,
    }
}

define_theme! {
    /// Theming properties for Fonts.
    pub FontTheme {
        color: &'static str,
    }
}

define_theme! {
    /// Theming properties the Switch components.
    pub SwitchTheme {
        background: &'static str,
        thumb_background: &'static str,
        enabled_background: &'static str,
        enabled_thumb_background: &'static str,
    }
}

define_theme! {
    /// Theming properties the Scrollbar components.
    pub ScrollbarTheme {
        background: &'static str,
        thumb_background: &'static str,
        hover_thumb_background: &'static str,
        active_thumb_background: &'static str,
    }
}

define_theme! {
    /// Theming properties for the App body.
    pub BodyTheme {
        background: &'static str,
        color: &'static str,
    }
}

define_theme! {
    /// Theming properties for Slider components.
    pub SliderTheme {
        background: &'static str,
        thumb_background: &'static str,
        thumb_inner_background: &'static str,
    }
}

define_theme! {
    /// Theming properties for Tooltip components.
    pub TooltipTheme {
        background: &'static str,
        color: &'static str,
        border_fill: &'static str,
    }
}

define_theme! {
    /// Theming properties for ExternalLink components.
    pub ExternalLinkTheme {
        highlight_color: &'static str,
    }
}

define_theme! {
    /// Theming properties for Accordion component.
    pub AccordionTheme {
        color: &'static str,
        background: &'static str,
        border_fill: &'static str,
    }
}

define_theme! {
    /// Theming properties for Loader component.
    pub LoaderTheme {
        primary_color: &'static str,
        secondary_color: &'static str,
    }
}

define_theme! {
    /// Theming properties for ProgressBar component.
    pub ProgressBarTheme {
        color: &'static str,
        background: &'static str,
        progress_background: &'static str,
    }
}

define_theme! {
    /// Theming properties for Table component.
    pub TableTheme {
        background: &'static str,
        arrow_fill: &'static str,
        alternate_row_background: &'static str,
        row_background: &'static str,
        divider_fill: &'static str,
        ..owned..
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
