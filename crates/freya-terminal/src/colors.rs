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

/// Map VT100 color to Skia Color
///
/// If `is_bg` is true, Default maps to background color instead of foreground
pub fn map_vt100_color(
    c: vt100::Color,
    is_bg: bool,
    default_fg: Color,
    default_bg: Color,
) -> Color {
    match c {
        vt100::Color::Default => {
            if is_bg {
                default_bg
            } else {
                default_fg
            }
        }
        vt100::Color::Rgb(r, g, b) => Color::from_rgb(r, g, b),
        vt100::Color::Idx(idx) => {
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

            // Fallback
            if is_bg { default_bg } else { default_fg }
        }
    }
}

/// Map VT100 foreground color to Skia Color
pub fn map_vt100_fg_color(c: vt100::Color, default_fg: Color, _default_bg: Color) -> Color {
    map_vt100_color(c, false, default_fg, _default_bg)
}

/// Map VT100 background color to Skia Color
pub fn map_vt100_bg_color(c: vt100::Color, _default_fg: Color, default_bg: Color) -> Color {
    map_vt100_color(c, true, _default_fg, default_bg)
}
