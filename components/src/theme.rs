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
pub struct ScrollbarTheme {
    pub background: &'static str,
    pub thumb_background: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BodyTheme {
    pub background: &'static str,
    pub color: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SliderTheme {
    pub background: &'static str,
    pub thumb_background: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub body: BodyTheme,
    pub button: ButtonTheme,
    pub switch: SwitchTheme,
    pub scrollbar: ScrollbarTheme,
    pub slider: SliderTheme,
}

pub static THEME: AtomRef<Theme> = |_| DARK_THEME.clone();

pub static LIGHT_THEME: Theme = Theme {
    body: BodyTheme {
        background: "white",
        color: "black",
    },
    slider: SliderTheme {
        background: "rgb(225, 225, 225)",
        thumb_background: "rgb(103, 80, 164)",
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
};

pub const DARK_THEME: Theme = Theme {
    body: BodyTheme {
        background: "rgb(25, 25, 25)",
        color: "white",
    },
    slider: SliderTheme {
        background: "rgb(60, 60, 60)",
        thumb_background: "rgb(247, 127, 0)",
    },
    button: ButtonTheme {
        background: "rgb(35, 35, 35)",
        hover_background: "rgb(80, 80, 80)",
        font_theme: FontTheme { color: "white" },
    },
    switch: SwitchTheme {
        background: "rgb(121, 116, 126)",
        thumb_background: "rgb(231, 224, 236)",
        enabled_background: "rgb(247, 127, 0)",
        enabled_thumb_background: "rgb(234, 221, 255)",
    },
    scrollbar: ScrollbarTheme {
        background: "rgb(35, 35, 35)",
        thumb_background: "rgb(100, 100, 100)",
    },
};
