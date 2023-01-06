use dioxus_core::ScopeState;
use dioxus_hooks::{use_shared_state, use_shared_state_provider, UseSharedState};

pub fn use_init_theme(cx: &ScopeState, theme: Theme) {
    use_shared_state_provider(cx, || theme);
}

pub fn use_init_default_theme(cx: &ScopeState) -> Theme {
    use_shared_state_provider(cx, || DARK_THEME);
    DARK_THEME
}

pub fn use_theme(cx: &ScopeState) -> UseSharedState<Theme> {
    use_shared_state::<Theme>(cx).unwrap()
}

pub fn use_get_theme(cx: &ScopeState) -> Theme {
    use_shared_state::<Theme>(cx)
        .map(|v| v.read().clone())
        .unwrap_or(DARK_THEME)
}

/// Theming properties for the Button component.
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

/// Theming properties for the Switch component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwitchTheme {
    pub background: &'static str,
    pub thumb_background: &'static str,
    pub enabled_background: &'static str,
    pub enabled_thumb_background: &'static str,
}

/// Theming properties for the Scrollbar component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrollbarTheme {
    pub background: &'static str,
    pub thumb_background: &'static str,
}

/// Theming properties for the window body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyTheme {
    pub background: &'static str,
    pub color: &'static str,
}

/// Theming properties for the Slider component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SliderTheme {
    pub background: &'static str,
    pub thumb_background: &'static str,
    pub thumb_inner_background: &'static str,
}

/// Theming properties for the Tooltip component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TooltipTheme {
    pub background: &'static str,
    pub color: &'static str,
}

/// Theming properties for the ExternalLink component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalLinkTheme {
    pub highlight_color: &'static str,
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
}

/// Light theme
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
};

/// Dark theme
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
};
