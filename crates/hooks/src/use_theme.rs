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

/// Theming properties for DropdownItem components.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropdownItemTheme {
    pub background: &'static str,
    pub select_background: &'static str,
    pub hover_background: &'static str,
    pub font_theme: FontTheme,
}

/// Theming properties for Dropdown components.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropdownTheme {
    pub desplegable_background: &'static str,
    pub background_button: &'static str,
    pub hover_background: &'static str,
    pub font_theme: FontTheme,
}

/// Theming properties for Button components.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonTheme {
    pub background: &'static str,
    pub hover_background: &'static str,
    pub font_theme: FontTheme,
}

/// Theming properties for Fonts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontTheme {
    pub color: &'static str,
}

/// Theming properties the Switch components.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwitchTheme {
    pub background: &'static str,
    pub thumb_background: &'static str,
    pub enabled_background: &'static str,
    pub enabled_thumb_background: &'static str,
}

/// Theming properties the Scrollbar components.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrollbarTheme {
    pub background: &'static str,
    pub thumb_background: &'static str,
}

/// Theming properties for the App body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyTheme {
    pub background: &'static str,
    pub color: &'static str,
}

/// Theming properties for Slider components.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SliderTheme {
    pub background: &'static str,
    pub thumb_background: &'static str,
    pub thumb_inner_background: &'static str,
}

/// Theming properties for Tooltip components.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TooltipTheme {
    pub background: &'static str,
    pub color: &'static str,
}

/// Theming properties for ExternalLink components.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalLinkTheme {
    pub highlight_color: &'static str,
}

/// Theming properties for Accordion component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccordionTheme {
    pub color: &'static str,
    pub background: &'static str,
}

/// Theming properties for Loader component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoaderTheme {
    pub primary_color: &'static str,
    pub secondary_color: &'static str,
}

/// Theming properties for ProgressBar component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgressBarTheme {
    pub background: &'static str,
    pub progress_background: &'static str,
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
        background: "rgb(220, 220, 220)",
        hover_background: "rgb(200, 200, 200)",
        font_theme: FontTheme {
            color: "rgb(10, 10, 10)",
        },
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
    },
    tooltip: TooltipTheme {
        background: "rgb(230,230,230)",
        color: "rgb(25,25,25)",
    },
    external_link: ExternalLinkTheme {
        highlight_color: "rgb(43,106,208)",
    },
    dropdown: DropdownTheme {
        desplegable_background: "white",
        background_button: "rgb(240, 240, 240)",
        hover_background: "rgb(215, 215, 215)",
        font_theme: FontTheme {
            color: "rgb(10, 10, 10)",
        },
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
        background: "rgb(215, 215, 215)",
    },
    loader: LoaderTheme {
        primary_color: "rgb(50, 50, 50)",
        secondary_color: "rgb(150, 150, 150)",
    },
    progress_bar: ProgressBarTheme {
        background: "rgb(210, 210, 210)",
        progress_background: "rgb(103, 80, 164)",
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
        hover_background: "rgb(80, 80, 80)",
        font_theme: FontTheme { color: "white" },
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
    },
    tooltip: TooltipTheme {
        background: "rgb(35,35,35)",
        color: "rgb(240,240,240)",
    },
    external_link: ExternalLinkTheme {
        highlight_color: "rgb(43,106,208)",
    },
    dropdown: DropdownTheme {
        desplegable_background: "rgb(25, 25, 25)",
        background_button: "rgb(35, 35, 35)",
        hover_background: "rgb(80, 80, 80)",
        font_theme: FontTheme { color: "white" },
    },
    dropdown_item: DropdownItemTheme {
        background: "rgb(35, 35, 35)",
        select_background: "rgb(80, 80, 80)",
        hover_background: "rgb(55, 55, 55)",
        font_theme: FontTheme { color: "white" },
    },
    accordion: AccordionTheme {
        color: "white",
        background: "rgb(30, 30, 30)",
    },
    loader: LoaderTheme {
        primary_color: "rgb(150, 150, 150)",
        secondary_color: "rgb(255, 255, 255)",
    },
    progress_bar: ProgressBarTheme {
        background: "rgb(60, 60, 60)",
        progress_background: "rgb(255, 95, 0)",
    },
};
