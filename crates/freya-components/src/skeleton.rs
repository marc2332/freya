use std::time::Duration;

use freya_animation::prelude::*;
use freya_core::prelude::*;
use torin::{
    position::Position,
    size::Size,
};

use crate::{
    define_theme,
    get_theme,
    theming::{
        component_themes::ColorsSheet,
        macros::{
            Preference,
            ResolvablePreference,
        },
    },
};

/// Animation style for the skeleton placeholder.
#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub enum SkeletonAnimation {
    #[default]
    Pulse,
    Shimmer,
}

impl ResolvablePreference<SkeletonAnimation> for Preference<SkeletonAnimation> {
    fn resolve(&self, _: &ColorsSheet) -> SkeletonAnimation {
        match self {
            Self::Reference(_) => panic!("Only Colors support references."),
            Self::Specific(v) => *v,
        }
    }
}

define_theme! {
    %[component]
    pub Skeleton {
        %[fields]
        /// Background color of the placeholder.
        background: Color,
        /// Shimmer highlight color.
        shimmer_color: Color,
        /// Duration of one animation cycle.
        duration: Duration,
        /// Animation style: [`SkeletonAnimation::Pulse`] or [`SkeletonAnimation::Shimmer`].
        animation: SkeletonAnimation,
        /// Corner radius of the placeholder shape.
        corner_radius: CornerRadius,
        /// Starting X position of the shimmer band (pixels, can be negative).
        shimmer_from: f32,
        /// Ending X position of the shimmer band (pixels).
        shimmer_to: f32,
        /// Width of the shimmer band in pixels.
        shimmer_width: f32,
    }
}

/// Skeleton loading placeholder with a configurable theme.
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # use std::time::Duration;
/// fn app() -> impl IntoElement {
///     let loading = use_state(|| true);
///     rect().width(Size::px(200.)).height(Size::px(80.)).child(
///         Skeleton::new(*loading.read())
///             .animation(SkeletonAnimation::Shimmer)
///             .duration(Duration::from_millis(1200))
///             .child("Some content"),
///     )
/// }
/// ```
#[derive(PartialEq)]
pub struct Skeleton {
    pub(crate) theme: Option<SkeletonThemePartial>,
    loading: bool,
    elements: Vec<Element>,
    key: DiffKey,
}

impl KeyExt for Skeleton {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ChildrenExt for Skeleton {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Skeleton {
    pub fn new(loading: bool) -> Self {
        Self {
            theme: None,
            loading,
            elements: Vec::new(),
            key: DiffKey::None,
        }
    }

    /// Override the full theme partial at once.
    pub fn theme(mut self, theme: SkeletonThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Component for Skeleton {
    fn render(&self) -> impl IntoElement {
        let loading = self.loading;
        let elements = self.elements.clone();

        let theme = get_theme!(&self.theme, SkeletonThemePreference, "skeleton");

        let animation = use_animation_with_dependencies(&theme, |conf, theme| {
            conf.on_creation(OnCreation::Run);
            conf.on_change(OnChange::Rerun);
            match theme.animation {
                SkeletonAnimation::Pulse => {
                    conf.on_finish(OnFinish::reverse());
                    AnimNum::new(0.4, 1.0).duration(theme.duration)
                }
                SkeletonAnimation::Shimmer => {
                    conf.on_finish(OnFinish::restart());
                    AnimNum::new(theme.shimmer_from, theme.shimmer_to).duration(theme.duration)
                }
            }
        });

        let value = animation.get().value();
        let is_pulse = theme.animation == SkeletonAnimation::Pulse;

        rect()
            .expanded()
            .maybe(loading, |el| {
                el.background(theme.background)
                    .corner_radius(theme.corner_radius)
                    .overflow(Overflow::Clip)
                    .maybe(is_pulse, |el| el.opacity(value))
                    .maybe(!is_pulse, |el| {
                        el.child(
                            rect()
                                .position(Position::new_absolute().left(value))
                                .width(Size::px(theme.shimmer_width))
                                .height(Size::fill())
                                .background(theme.shimmer_color),
                        )
                    })
            })
            .maybe(!loading, |el| el.children(elements))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
