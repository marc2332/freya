use crate::define_theme;

define_theme! {
    /// Theme-level typography sizes consumed by
    /// [`LabelThemeExt`](crate::element_expansions::LabelThemeExt) methods such
    /// as `.title()`, `.subtitle()`, `.body()`, `.caption()`, `.overline()`.
    %[no_ext]
    pub Typography {
        %[fields]
        title: f32,
        subtitle: f32,
        body: f32,
        caption: f32,
        overline: f32,
    }
}
