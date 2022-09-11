use fermi::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ButtonTheme {
    pub background: &'static str,
    pub hover_background: &'static str,
    pub font_theme: FontTheme,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontTheme {
    pub color: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SwitchTheme {
    pub background: &'static str,
    pub thumb_background: &'static str,
    pub enabled_background: &'static str,
    pub enabled_thumb_background: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub button: ButtonTheme,
    pub switch: SwitchTheme,
}

pub static THEME: AtomRef<Theme> = |_| DARK_THEME.clone();

pub static LIGHT_THEME: Theme = Theme {
    button: ButtonTheme {
        background: "rgb(200, 200, 200)",
        hover_background: "rgb(140, 140, 140)",
        font_theme: FontTheme { color: "black" },
    },
    switch: SwitchTheme {
        background: "rgb(121, 116, 126)",
        thumb_background: "rgb(231, 224, 236)",
        enabled_background: "rgb(103, 80, 164)",
        enabled_thumb_background: "rgb(234, 221, 255)",
    },
};

pub const DARK_THEME: Theme = Theme {
    button: ButtonTheme {
        background: "rgb(35, 35, 35)",
        hover_background: "rgb(115, 115, 115)",
        font_theme: FontTheme { color: "white" },
    },
    switch: SwitchTheme {
        background: "rgb(121, 116, 126)",
        thumb_background: "rgb(231, 224, 236)",
        enabled_background: "rgb(247, 127, 0)",
        enabled_thumb_background: "rgb(234, 221, 255)",
    },
};
