use freya_animation::prelude::*;
use freya_components::{
    define_theme,
    theming::{
        component_themes::ColorsSheet,
        hooks::get_theme_or_default,
        macros::{
            Preference,
            ResolvablePreference,
        },
    },
};
use freya_core::prelude::*;
use torin::{
    position::Position,
    size::Size,
};

pub mod prelude {
    pub use crate::{
        Skeleton,
        SkeletonAnimation,
        SkeletonExt,
        SkeletonStyleThemePartial,
        SkeletonStyleThemePartialExt,
    };
}

const DEFAULT_DURATION_MS: u64 = 1000;
const DEFAULT_CORNER_RADIUS: f32 = 4.;

const SHIMMER_FROM: f32 = -250.;
const SHIMMER_TO: f32 = 900.;
const SHIMMER_WIDTH: f32 = 200.;

/// Animation style for the skeleton placeholder.
#[derive(PartialEq, Clone, Default, Debug)]
pub enum SkeletonAnimation {
    /// Fades opacity in and out repeatedly (default).
    #[default]
    Pulse,
    /// A bright band sweeps from left to right.
    Shimmer,
}

impl ResolvablePreference<SkeletonAnimation> for Preference<SkeletonAnimation> {
    fn resolve(&self, _: &ColorsSheet) -> SkeletonAnimation {
        match self {
            Self::Reference(_) => panic!("Only Colors support references."),
            Self::Specific(v) => v.clone(),
        }
    }
}

define_theme! {
    for = Skeleton;
    theme_field = theme;

    %[component]
    pub SkeletonStyle {
        %[fields]
        /// Background color of the placeholder. Defaults to the theme's `surface_primary`.
        background: Color,
        /// Shimmer highlight color. Defaults to `text_placeholder` at reduced opacity.
        shimmer_color: Color,
        /// Duration of one animation cycle in milliseconds.
        duration_ms: u64,
        /// Animation style: [`SkeletonAnimation::Pulse`] or [`SkeletonAnimation::Shimmer`].
        animation: SkeletonAnimation,
        /// Corner radius of the placeholder shape.
        corner_radius: CornerRadius,
    }
}

/// Skeleton loading placeholder with a configurable theme.
///
/// Uses the active theme's `surface_primary` and `text_placeholder` colors by default.
/// Override any field via the individual setters or `.theme()`.
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # use freya_skeletons::prelude::*;
/// fn app() -> impl IntoElement {
///     let loading = use_state(|| true);
///     rect().width(Size::px(200.)).height(Size::px(80.)).child(
///         Skeleton::new()
///             .loading(*loading.read())
///             .animation(SkeletonAnimation::Shimmer)
///             .duration_ms(1200u64)
///             .child("Some content"),
///     )
/// }
/// ```
#[derive(PartialEq)]
pub struct Skeleton {
    pub(crate) theme: Option<SkeletonStyleThemePartial>,
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
        Self::new()
    }
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            theme: None,
            loading: false,
            elements: Vec::new(),
            key: DiffKey::None,
        }
    }

    /// Whether to show the skeleton placeholder instead of real content.
    pub fn loading(mut self, loading: impl Into<bool>) -> Self {
        self.loading = loading.into();
        self
    }

    /// Override the full theme partial at once.
    /// Prefer individual setters (`.background()`, `.animation()`, etc.) for partial overrides.
    pub fn theme(mut self, theme: SkeletonStyleThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Component for Skeleton {
    fn render(&self) -> impl IntoElement {
        let loading = self.loading;
        let elements = self.elements.clone();

        let raw_theme = get_theme_or_default();
        let colors = raw_theme.read().colors.clone();

        // Build defaults from the active color sheet, then apply any user overrides.
        let mut preference = SkeletonStyleThemePreference {
            background: Preference::Reference("surface_primary"),
            shimmer_color: Preference::Specific(colors.text_placeholder.with_a(90)),
            duration_ms: Preference::Specific(DEFAULT_DURATION_MS),
            animation: Preference::Specific(SkeletonAnimation::Pulse),
            corner_radius: Preference::Specific(CornerRadius::new_all(DEFAULT_CORNER_RADIUS)),
        };

        if let Some(partial) = &self.theme {
            preference.apply_optional(partial);
        }

        let theme = preference.resolve(&colors);

        // Hook must be called unconditionally. Only `animation` and `duration_ms` affect it.
        let anim_key = (theme.animation.clone(), theme.duration_ms);
        let animation =
            use_animation_with_dependencies(&anim_key, |conf, (animation, duration_ms)| {
                conf.on_creation(OnCreation::Run);
                conf.on_change(OnChange::Rerun);
                match animation {
                    SkeletonAnimation::Pulse => {
                        conf.on_finish(OnFinish::reverse());
                        AnimNum::new(0.4, 1.0).time(*duration_ms)
                    }
                    SkeletonAnimation::Shimmer => {
                        conf.on_finish(OnFinish::restart());
                        AnimNum::new(SHIMMER_FROM, SHIMMER_TO).time(*duration_ms)
                    }
                }
            });

        let value = animation.get().value();
        let is_pulse = theme.animation == SkeletonAnimation::Pulse;

        rect()
            .expanded()
            .maybe(loading, |r| {
                r.background(theme.background)
                    .corner_radius(theme.corner_radius)
                    .overflow(Overflow::Clip)
                    .maybe(is_pulse, |r| r.opacity(value))
                    .maybe(!is_pulse, |r| {
                        r.child(
                            rect()
                                .position(Position::new_absolute().left(value))
                                .width(Size::px(SHIMMER_WIDTH))
                                .height(Size::fill())
                                .background(theme.shimmer_color),
                        )
                    })
            })
            .maybe(!loading, |r| r.children(elements))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Trait for applying a static (non-animated) skeleton style directly on builder elements.
///
/// When `loading` is true, clears children and applies the theme's `surface_primary` background.
/// Place `.skeleton()` **after** all `.child()` calls as it clears children when loading.
///
/// For animated skeletons, use the [`Skeleton`] component instead.
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # use freya_skeletons::prelude::*;
/// fn app() -> impl IntoElement {
///     let loading = use_state(|| true);
///     rect()
///         .width(Size::px(200.))
///         .height(Size::px(20.))
///         .child("content")
///         .skeleton(*loading.read())
/// }
/// ```
pub trait SkeletonExt: StyleExt + ChildrenExt + Sized {
    fn skeleton(mut self, loading: impl Into<bool>) -> Self {
        if loading.into() {
            self.get_children().clear();
            let theme = get_theme_or_default();
            let color = theme.read().colors.surface_primary;
            self.background(color)
        } else {
            self
        }
    }
}

impl<T: StyleExt + ChildrenExt> SkeletonExt for T {}
