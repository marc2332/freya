use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    use_animation,
    use_applied_theme,
    AnimNum,
    LoaderTheme,
    LoaderThemeWith,
    OnFinish,
};

/// Properties for the [`Loader`] component.
#[derive(Props, Clone, PartialEq)]
pub struct LoaderProps {
    /// Theme override.
    pub theme: Option<LoaderThemeWith>,
}

/// # Styling
/// Inherits the [`LoaderTheme`](freya_hooks::LoaderTheme) theme.
#[allow(non_snake_case)]
pub fn Loader(props: LoaderProps) -> Element {
    let theme = use_applied_theme!(&props.theme, loader);
    let animation = use_animation(|conf| {
        conf.auto_start(true);
        conf.on_finish(OnFinish::Restart);
        AnimNum::new(0.0, 360.0).time(650)
    });

    let LoaderTheme { primary_color } = theme;

    let degrees = animation.get().read().read();

    rsx!(svg {
        rotate: "{degrees}deg",
        width: "48",
        height: "48",
        svg_content: r#"
            <svg viewBox="0 0 600 600" xmlns="http://www.w3.org/2000/svg">
                <circle class="spin" cx="300" cy="300" fill="none"
                r="250" stroke-width="64" stroke="{primary_color}"
                stroke-dasharray="256 1400"
                stroke-linecap="round" />
            </svg>
        "#
    })
}
