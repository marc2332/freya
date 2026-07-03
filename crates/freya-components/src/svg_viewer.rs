use std::{
    rc::Rc,
    sync::LazyLock,
};

use anyhow::Context;
use async_lock::Semaphore;
use bytes::Bytes;
use freya_core::{
    element::EventHandlerType,
    elements::image::*,
    events::name::EventName,
    prelude::*,
};
use freya_engine::prelude::{
    FontMgr,
    SkImage,
    raster_n32_premul,
    svg,
};
use rustc_hash::FxHashMap;
use torin::prelude::{
    Size,
    Size2D,
};

#[cfg(feature = "remote-asset")]
use crate::http::Http;
use crate::{
    cache::*,
    image_viewer::{
        DecodeSize,
        ImageSource,
    },
    loader::CircularLoader,
    theming::hooks::get_theme_or_default,
};

/// Limit the amount of SVGs rasterized in parallel.
static RASTER_LIMIT: LazyLock<Semaphore> = LazyLock::new(|| Semaphore::new(4));

/// Color and stroke overrides applied to an SVG before it is rasterized.
#[derive(Default, Clone, Copy, PartialEq)]
struct SvgStyle {
    color: Option<Color>,
    fill: Option<Color>,
    stroke: Option<Color>,
    stroke_width: Option<f32>,
}

impl SvgStyle {
    /// A hashable representation, since `f32` is not [`Hash`].
    fn as_key(&self) -> (Option<Color>, Option<Color>, Option<Color>, Option<u32>) {
        (
            self.color,
            self.fill,
            self.stroke,
            self.stroke_width.map(f32::to_bits),
        )
    }
}

/// Fetch, parse and rasterize an SVG at `size` off the main thread.
async fn rasterize(
    source: ImageSource,
    size: DecodeSize,
    style: SvgStyle,
) -> anyhow::Result<SkImage> {
    #[cfg(feature = "remote-asset")]
    let bytes = {
        let client = Http::get();
        blocking::unblock(move || source.fetch(&client)).await?
    };
    #[cfg(not(feature = "remote-asset"))]
    let bytes = blocking::unblock(move || source.fetch()).await?;

    let _permit = RASTER_LIMIT.acquire().await;
    blocking::unblock(move || {
        let width = size.width.max(1) as i32;
        let height = size.height.max(1) as i32;

        let mut dom = svg::Dom::from_bytes(&bytes, FontMgr::empty())
            .map_err(|err| anyhow::anyhow!("Failed to parse SVG: {err:?}"))?;
        dom.set_container_size((width, height));

        let mut root = dom.root();
        root.set_width(svg::Length::new(width as f32, svg::LengthUnit::PX));
        root.set_height(svg::Length::new(height as f32, svg::LengthUnit::PX));
        root.set_color(style.color.unwrap_or(Color::BLACK).into());
        if let Some(fill) = style.fill {
            root.set_fill(svg::Paint::from_color(fill.into()));
        }
        if let Some(stroke) = style.stroke {
            root.set_stroke(svg::Paint::from_color(stroke.into()));
        }
        if let Some(stroke_width) = style.stroke_width {
            root.set_stroke_width(svg::Length::new(stroke_width, svg::LengthUnit::PX));
        }

        let mut surface =
            raster_n32_premul((width, height)).context("Failed to create the SVG surface.")?;
        dom.render(surface.canvas());
        Ok(surface.image_snapshot())
    })
    .await
}

/// SVG viewer component.
///
/// Rasterizes the SVG asynchronously to the size measured on its container, caching the result.
/// See [`ImageSource`] for all supported sources.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     SvgViewer::new(include_bytes!("../../../examples/ferris.svg"))
///         .width(Size::px(300.))
///         .height(Size::px(300.))
/// }
/// ```
#[derive(PartialEq)]
pub struct SvgViewer {
    source: ImageSource,
    asset_age: AssetAge,

    layout: LayoutData,
    image_data: ImageData,
    accessibility: AccessibilityData,
    effect: EffectData,
    event_handlers: FxHashMap<EventName, EventHandlerType>,
    style: SvgStyle,
    show_loader: bool,

    error_renderer: Option<Callback<String, Element>>,

    key: DiffKey,
}

impl SvgViewer {
    pub fn new(source: impl Into<ImageSource>) -> Self {
        let mut accessibility = AccessibilityData::default();
        accessibility.builder.set_role(AccessibilityRole::SvgRoot);

        SvgViewer {
            source: source.into(),
            asset_age: AssetAge::default(),
            layout: LayoutData::default(),
            image_data: ImageData::default(),
            accessibility,
            effect: EffectData::default(),
            event_handlers: FxHashMap::default(),
            style: SvgStyle::default(),
            show_loader: true,
            error_renderer: None,
            key: DiffKey::None,
        }
    }

    /// Whether to render a loading indicator while the SVG is being rasterized. Defaults to `true`.
    pub fn show_loader(mut self, show_loader: bool) -> Self {
        self.show_loader = show_loader;
        self
    }

    /// Override the SVG's `currentColor`, used by shapes that inherit their color.
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.style.color = Some(color.into());
        self
    }

    /// Override the fill color of the SVG's shapes.
    pub fn fill(mut self, fill: impl Into<Color>) -> Self {
        self.style.fill = Some(fill.into());
        self
    }

    /// Override the stroke color of the SVG's shapes.
    pub fn stroke(mut self, stroke: impl Into<Color>) -> Self {
        self.style.stroke = Some(stroke.into());
        self
    }

    /// Override the SVG stroke width.
    pub fn stroke_width(mut self, stroke_width: impl Into<f32>) -> Self {
        self.style.stroke_width = Some(stroke_width.into());
        self
    }

    /// Customize how long the raster remains cached after no longer being used.
    pub fn asset_age(mut self, asset_age: impl Into<AssetAge>) -> Self {
        self.asset_age = asset_age.into();
        self
    }

    /// Custom element rendered when the SVG fails to load.
    pub fn error_renderer(mut self, renderer: impl Into<Callback<String, Element>>) -> Self {
        self.error_renderer = Some(renderer.into());
        self
    }
}

impl KeyExt for SvgViewer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for SvgViewer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for SvgViewer {}

impl ImageExt for SvgViewer {
    fn get_image_data(&mut self) -> &mut ImageData {
        &mut self.image_data
    }
}

impl AccessibilityExt for SvgViewer {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl EffectExt for SvgViewer {
    fn get_effect(&mut self) -> &mut EffectData {
        &mut self.effect
    }
}

impl EventHandlersExt for SvgViewer {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.event_handlers
    }
}

/// The logical size implied by a pixel-sized layout.
fn layout_pixel_size(layout: &LayoutData) -> Option<Size2D> {
    match (&layout.width, &layout.height) {
        (Size::Pixels(width), Size::Pixels(height)) => Some(Size2D::new(width.get(), height.get())),
        _ => None,
    }
}

impl Component for SvgViewer {
    fn render(&self) -> impl IntoElement {
        let scale_factor = *Platform::get().scale_factor.read();
        let seed = layout_pixel_size(&self.layout);
        let mut measured = use_state(|| seed);
        let mut asset_cacher = use_hook(AssetCacher::get);

        let target = measured().map(|logical| {
            DecodeSize::new(
                (logical.width * scale_factor as f32).round().max(1.) as u32,
                (logical.height * scale_factor as f32).round().max(1.) as u32,
            )
        });

        let style = self.style;
        let asset_config =
            AssetConfiguration::new((&self.source, target, style.as_key()), self.asset_age);
        let asset = use_asset(&asset_config);

        use_side_effect_with_deps(
            &(self.source.clone(), asset_config, target, style),
            move |(source, asset_config, target, style)| {
                let Some(target) = *target else {
                    return;
                };
                if matches!(
                    asset_cacher.read_asset(asset_config),
                    Some(Asset::Pending) | Some(Asset::Error(_))
                ) {
                    asset_cacher.update_asset(asset_config.clone(), Asset::Loading);

                    let source = source.clone();
                    let asset_config = asset_config.clone();
                    let style = *style;
                    spawn_forever(async move {
                        match rasterize(source, target, style).await {
                            Ok(image) => {
                                asset_cacher.update_asset(
                                    asset_config,
                                    Asset::Cached(Rc::new(ImageHandle::new(image, Bytes::new()))),
                                );
                            }
                            Err(err) => {
                                asset_cacher
                                    .update_asset(asset_config, Asset::Error(err.to_string()));
                            }
                        }
                    });
                }
            },
        );

        match asset {
            Asset::Cached(asset) => {
                let handle = asset.downcast_ref::<ImageHandle>().unwrap().clone();
                image(handle)
                    .accessibility(self.accessibility.clone())
                    .layout(self.layout.clone())
                    .image_data(self.image_data.clone())
                    .effect(self.effect.clone())
                    .with_event_handlers(self.event_handlers.clone())
                    .on_sized(move |event: Event<SizedEventData>| {
                        measured.set_if_modified(Some(event.area.size));
                    })
                    .into_element()
            }
            Asset::Error(err) => match &self.error_renderer {
                Some(renderer) => renderer.call(err),
                None => err.into(),
            },
            Asset::Pending | Asset::Loading => rect()
                .layout(self.layout.clone())
                .on_sized(move |event: Event<SizedEventData>| {
                    measured.set_if_modified(Some(event.area.size));
                })
                .center()
                .maybe(self.show_loader, |loading| {
                    loading.child(CircularLoader::new())
                })
                .into_element(),
        }
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Theme-aware color, fill and stroke shortcuts for [`SvgViewer`].
pub trait SvgThemeExt {
    fn theme_color(self) -> Self;
    fn theme_accent_color(self) -> Self;
    fn theme_fill(self) -> Self;
    fn theme_stroke(self) -> Self;
    fn theme_accent_fill(self) -> Self;
    fn theme_accent_stroke(self) -> Self;
}

impl SvgThemeExt for SvgViewer {
    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.read().colors.text_primary)
    }

    fn theme_accent_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.read().colors.primary)
    }

    fn theme_fill(self) -> Self {
        let theme = get_theme_or_default();
        self.fill(theme.read().colors.text_primary)
    }

    fn theme_stroke(self) -> Self {
        let theme = get_theme_or_default();
        self.stroke(theme.read().colors.text_primary)
    }

    fn theme_accent_fill(self) -> Self {
        let theme = get_theme_or_default();
        self.fill(theme.read().colors.primary)
    }

    fn theme_accent_stroke(self) -> Self {
        let theme = get_theme_or_default();
        self.stroke(theme.read().colors.primary)
    }
}
