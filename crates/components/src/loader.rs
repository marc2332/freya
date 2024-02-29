use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_animation, use_applied_theme, AnimNum, LoaderTheme, LoaderThemeWith};

/// [`Loader`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct LoaderProps {
    /// Theme override.
    pub theme: Option<LoaderThemeWith>,
}

/// `Loader` component.
///
/// # Props
/// See [`LoaderProps`].
///
/// # Styling
/// Inherits the [`LoaderTheme`](freya_hooks::LoaderTheme) theme.
///
#[allow(non_snake_case)]
pub fn Loader(props: LoaderProps) -> Element {
    let theme = use_applied_theme!(&props.theme, loader);
    let anim = use_animation(|ctx| {
        ctx.auto_start(true);
        ctx.with(AnimNum::new(0.0, 360.0).time(650))
    });

    let LoaderTheme { primary_color } = theme;

    let degrees = anim.read().get().read().as_f32();

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
