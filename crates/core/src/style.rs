pub fn default_fonts() -> Vec<String> {
    let mut fonts = vec!["Noto Sans".to_string(), "Arial".to_string()];
    if cfg!(target_os = "windows") {
        fonts.insert(0, "Segoe UI".to_string());
        fonts.insert(1, "Segoe UI Emoji".to_string());
    } else if cfg!(target_os = "macos") {
        fonts.insert(0, "San Francisco (SF)".to_string());
    } else if cfg!(target_os = "linux") {
        fonts.insert(0, "Ubuntu".to_string());
    }
    fonts
}
