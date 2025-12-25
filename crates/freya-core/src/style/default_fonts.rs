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
    }
    fonts
}
