use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_hooks::{
    use_animation,
    use_applied_theme,
    use_surface_theme_indicator,
    AnimNum,
    LoaderThemeWith,
    OnFinish,
    SurfaceThemeIndicator,
};

/// Properties for the [`Loader`] component.
#[derive(Props, Clone, PartialEq)]
pub struct LoaderProps {
    /// Theme override.
    pub theme: Option<LoaderThemeWith>,
    #[props(default = "48".to_string())]
    pub size: String,
}

/// # Styling
/// Inherits the [`LoaderTheme`](freya_hooks::LoaderTheme) theme.
///
/// Use cases: showing the progress of an external task (http calls for example), etc.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(Loader {})
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           {app()}
/// #       }
/// #   )
/// # }, (250., 250.).into(), "./images/gallery_loader.png");
/// ```
///
/// # Preview
/// ![Loader Preview][loader]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("loader", "images/gallery_loader.png")
)]
#[allow(non_snake_case)]
pub fn Loader(props: LoaderProps) -> Element {
    let theme = use_applied_theme!(&props.theme, loader);
    let animation = use_animation(|conf| {
        conf.auto_start(true);
        conf.on_finish(OnFinish::Restart);
        AnimNum::new(0.0, 360.0).time(650)
    });
    let contextual_theme = use_surface_theme_indicator();

    let color = match contextual_theme {
        SurfaceThemeIndicator::Primary => theme.opposite_color,
        SurfaceThemeIndicator::Opposite => theme.primary_color,
    };

    let degrees = animation.get().read().read();

    rsx!(svg {
        rotate: "{degrees}deg",
        width: "{props.size}",
        height: "{props.size}",
        svg_content: r#"
            <svg viewBox="0 0 600 600" xmlns="http://www.w3.org/2000/svg">
                <circle class="spin" cx="300" cy="300" fill="none"
                r="250" stroke-width="64" stroke="{color}"
                stroke-dasharray="256 1400"
                stroke-linecap="round" />
            </svg>
        "#
    })
}
