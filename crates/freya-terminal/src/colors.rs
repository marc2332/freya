use freya_core::prelude::Color;

/// ANSI 16-color palette
const ANSI_COLORS: [(u8, u8, u8); 16] = [
    (0, 0, 0),       // Black
    (128, 0, 0),     // Dark Red
    (0, 128, 0),     // Dark Green
    (128, 128, 0),   // Dark Yellow
    (0, 0, 128),     // Dark Blue
    (128, 0, 128),   // Dark Magenta
    (0, 128, 128),   // Dark Cyan
    (192, 192, 192), // Light Gray
    (128, 128, 128), // Gray
    (255, 0, 0),     // Red
    (0, 255, 0),     // Green
    (255, 255, 0),   // Yellow
    (0, 0, 255),     // Blue
    (255, 0, 255),   // Magenta
    (0, 255, 255),   // Cyan
    (255, 255, 255), // White
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
            if i >= 16 && i <= 231 {
                let v = i - 16;
                let r = v / 36;
                let g = (v / 6) % 6;
                let b = v % 6;
                return Color::from_rgb(RGB_LEVELS[r], RGB_LEVELS[g], RGB_LEVELS[b]);
            }

            // Grayscale (24 colors, indices 232-255)
            if i >= 232 && i <= 255 {
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
