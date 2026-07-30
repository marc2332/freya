use std::borrow::Cow;

pub fn default_fonts() -> Vec<Cow<'static, str>> {
    let mut fonts = vec!["Noto Sans".into(), "Arial".into()];
    if cfg!(target_os = "windows") {
        fonts.insert(0, "Segoe UI".into());
        fonts.insert(1, "Segoe UI Emoji".into());
    } else if cfg!(target_os = "macos") {
        fonts.insert(0, ".AppleSystemUIFont".into());
    } else if cfg!(target_os = "linux") {
        fonts.insert(0, "Ubuntu".into());
        fonts.insert(1, "Adwaita Sans".into());
    } else if cfg!(target_os = "android") {
        fonts.insert(0, "Roboto".into());
    }
    fonts
}

/// OS-specific monospace fonts.
pub fn default_monospace_fonts() -> Vec<Cow<'static, str>> {
    let mut fonts = vec!["Noto Sans Mono".into(), "Courier New".into()];
    if cfg!(target_os = "windows") {
        fonts.insert(0, "Cascadia Mono".into());
        fonts.insert(1, "Consolas".into());
    } else if cfg!(target_os = "macos") {
        fonts.insert(0, "Menlo".into());
    } else if cfg!(target_os = "linux") {
        fonts.insert(0, "Ubuntu Mono".into());
        fonts.insert(1, "Adwaita Mono".into());
        fonts.insert(2, "DejaVu Sans Mono".into());
    } else if cfg!(target_os = "android") {
        fonts.insert(0, "monospace".into());
    }
    fonts
}
