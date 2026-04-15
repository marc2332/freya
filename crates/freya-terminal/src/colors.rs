use alacritty_terminal::vte::ansi::{
    Color as AnsiColor,
    NamedColor,
};
use freya_core::prelude::Color;

/// ANSI 16-color palette (matches WezTerm defaults)
const ANSI_COLORS: [(u8, u8, u8); 16] = [
    (0, 0, 0),       // Black
    (204, 85, 85),   // Red
    (85, 204, 85),   // Green
    (215, 215, 0),   // Yellow
    (84, 85, 203),   // Blue
    (204, 85, 204),  // Magenta
    (122, 202, 202), // Cyan
    (204, 204, 204), // White
    (85, 85, 85),    // Bright Black
    (255, 85, 85),   // Bright Red
    (85, 255, 85),   // Bright Green
    (255, 255, 0),   // Bright Yellow
    (85, 85, 255),   // Bright Blue
    (255, 85, 255),  // Bright Magenta
    (85, 255, 255),  // Bright Cyan
    (255, 255, 255), // Bright White
];

/// 6x6x6 RGB cube levels for 256-color palette
const RGB_LEVELS: [u8; 6] = [0u8, 95u8, 135u8, 175u8, 215u8, 255u8];

/// Map an alacritty `Color` to a Freya `Color`. `default_fg` / `default_bg`
/// resolve `NamedColor::Foreground` / `Background` to the configured colors.
pub fn map_ansi_color(c: AnsiColor, default_fg: Color, default_bg: Color) -> Color {
    match c {
        AnsiColor::Named(name) => named_color(name, default_fg, default_bg),
        AnsiColor::Spec(rgb) => Color::from_rgb(rgb.r, rgb.g, rgb.b),
        AnsiColor::Indexed(idx) => indexed_color(idx, default_fg),
    }
}

fn named_color(name: NamedColor, default_fg: Color, default_bg: Color) -> Color {
    match name {
        NamedColor::Foreground | NamedColor::BrightForeground | NamedColor::DimForeground => {
            default_fg
        }
        NamedColor::Background => default_bg,
        NamedColor::Cursor => default_fg,
        other => {
            // NamedColor::Black..BrightWhite are 0..=15.
            let idx = other as u8;
            if idx < 16 {
                let (r, g, b) = ANSI_COLORS[idx as usize];
                Color::from_rgb(r, g, b)
            } else {
                default_fg
            }
        }
    }
}

fn indexed_color(idx: u8, default: Color) -> Color {
    let i = idx as usize;

    // ANSI 16 colors
    if i < 16 {
        let (r, g, b) = ANSI_COLORS[i];
        return Color::from_rgb(r, g, b);
    }

    // 6x6x6 RGB cube (216 colors, indices 16-231)
    if (16..=231).contains(&i) {
        let v = i - 16;
        let r = v / 36;
        let g = (v / 6) % 6;
        let b = v % 6;
        return Color::from_rgb(RGB_LEVELS[r], RGB_LEVELS[g], RGB_LEVELS[b]);
    }

    // Grayscale (24 colors, indices 232-255)
    if (232..=255).contains(&i) {
        let shade = 8 + ((i - 232) * 10) as u8;
        return Color::from_rgb(shade, shade, shade);
    }

    default
}
