use fermi::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ButtonTheme {
    pub background: &'static str,
    pub hover_background: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub button: ButtonTheme,
}

pub static THEME: AtomRef<Theme> = |_| DARK_THEME.clone();

pub const LIGHT_THEME: Theme = Theme {
    button: ButtonTheme {
        background: "rgb(200, 200, 200)",
        hover_background: "rgb(140, 140, 140)",
    },
};

pub const DARK_THEME: Theme = Theme {
    button: ButtonTheme {
        background: "rgb(35, 35, 35)",
        hover_background: "rgb(115, 115, 115)",
    },
};
