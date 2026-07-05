use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{
        Hash,
        Hasher,
    },
    path::PathBuf,
    rc::Rc,
    sync::LazyLock,
};

use anyhow::Context;
use async_lock::Semaphore;
use bytes::Bytes;
use freya_core::{
    elements::image::*,
    prelude::*,
};
use freya_engine::prelude::{
    Paint,
    SkData,
    SkImage,
    SkRect,
    raster_n32_premul,
};
#[cfg(feature = "remote-asset")]
use reqwest::{
    Url,
    blocking::Client,
};
use torin::prelude::{
    Size,
    Size2D,
};

#[cfg(feature = "remote-asset")]
use crate::http::Http;
use crate::{
    cache::*,
    loader::CircularLoader,
};

/// Supported image sources for [`ImageViewer`].
///
/// ### URI
///
/// Good to load remote images.
///
/// > Requires the `remote-asset` feature to be enabled.
///
/// ```rust
/// # use freya::prelude::*;
/// let source: ImageSource =
///     "https://upload.wikimedia.org/wikipedia/commons/8/8a/Gecarcinus_quadratus_%28Nosara%29.jpg"
///         .into();
/// ```
///
/// ### Path
///
/// Good for dynamic loading.
///
/// ```rust
/// # use freya::prelude::*;
/// # use std::path::PathBuf;
/// let source: ImageSource = PathBuf::from("./examples/rust_logo.png").into();
/// ```
/// ### Raw bytes
///
/// Good for embedded images.
///
/// ```rust
/// # use freya::prelude::*;
/// let source: ImageSource = (
///     "rust-logo",
///     include_bytes!("../../../examples/rust_logo.png"),
/// )
///     .into();
/// ```
///
/// ### Dynamic bytes
///
/// Good for rendering custom allocated images.
///
/// ```rust
/// # use freya::prelude::*;
/// # use bytes::Bytes;
/// fn app() -> impl IntoElement {
///     let image_data = use_state(|| (0, Bytes::from(vec![/* ... */])));
///     let source: ImageSource = image_data.read().clone().into();
///     ImageViewer::new(source)
/// }
/// ```
#[derive(PartialEq, Clone)]
pub enum ImageSource {
    /// Remote image loaded from a URI.
    ///
    /// Requires the `remote-asset` feature.
    #[cfg(feature = "remote-asset")]
    Uri(Url),

    Path(PathBuf),

    Bytes(u64, Bytes),
}

impl<H: Hash> From<(H, Bytes)> for ImageSource {
    fn from((id, bytes): (H, Bytes)) -> Self {
        let mut hasher = DefaultHasher::default();
        id.hash(&mut hasher);
        Self::Bytes(hasher.finish(), bytes)
    }
}

impl<H: Hash> From<(H, &'static [u8])> for ImageSource {
    fn from((id, bytes): (H, &'static [u8])) -> Self {
        (id, Bytes::from_static(bytes)).into()
    }
}

impl<const N: usize, H: Hash> From<(H, &'static [u8; N])> for ImageSource {
    fn from((id, bytes): (H, &'static [u8; N])) -> Self {
        (id, Bytes::from_static(bytes)).into()
    }
}

impl From<Bytes> for ImageSource {
    fn from(bytes: Bytes) -> Self {
        let mut hasher = DefaultHasher::default();
        bytes.hash(&mut hasher);
        Self::Bytes(hasher.finish(), bytes)
    }
}

impl From<&'static [u8]> for ImageSource {
    fn from(bytes: &'static [u8]) -> Self {
        Bytes::from_static(bytes).into()
    }
}

impl<const N: usize> From<&'static [u8; N]> for ImageSource {
    fn from(bytes: &'static [u8; N]) -> Self {
        Bytes::from_static(bytes).into()
    }
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "remote-asset")))]
#[cfg(feature = "remote-asset")]
impl From<Url> for ImageSource {
    fn from(uri: Url) -> Self {
        Self::Uri(uri)
    }
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "remote-asset")))]
#[cfg(feature = "remote-asset")]
impl From<&'static str> for ImageSource {
    fn from(src: &'static str) -> Self {
        Self::Uri(Url::parse(src).expect("Invalid URL"))
    }
}

impl From<PathBuf> for ImageSource {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl Hash for ImageSource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            #[cfg(feature = "remote-asset")]
            Self::Uri(uri) => uri.hash(state),
            Self::Path(path) => path.hash(state),
            Self::Bytes(id, _) => id.hash(state),
        }
    }
}

pub type DecodeSize = euclid::Size2D<u32, ()>;

/// Limit the amount of images that are loaded in parallel.
static DECODE_LIMIT: LazyLock<Semaphore> = LazyLock::new(|| Semaphore::new(4));

impl ImageSource {
    /// Read the source's raw encoded bytes. Blocking, meant to run inside `unblock`.
    pub(crate) fn fetch(
        self,
        #[cfg(feature = "remote-asset")] client: &Client,
    ) -> anyhow::Result<Bytes> {
        Ok(match self {
            #[cfg(feature = "remote-asset")]
            Self::Uri(uri) => client.get(uri).send()?.error_for_status()?.bytes()?,
            Self::Path(path) => fs::read(path).map(Bytes::from)?,
            Self::Bytes(_, bytes) => bytes,
        })
    }

    /// Fetch the source's encoded bytes and decode them into a Skia image.
    pub async fn load(
        &self,
        decode_size: Option<DecodeSize>,
        sampling_mode: SamplingMode,
    ) -> anyhow::Result<(SkImage, Bytes)> {
        let source = self.clone();
        #[cfg(feature = "remote-asset")]
        let client = Http::get();
        let _decode_permit = DECODE_LIMIT.acquire().await;
        blocking::unblock(move || {
            #[cfg(feature = "remote-asset")]
            let bytes = source.fetch(&client)?;
            #[cfg(not(feature = "remote-asset"))]
            let bytes = source.fetch()?;
            let image = SkImage::from_encoded(unsafe { SkData::new_bytes(&bytes) })
                .context("Failed to decode Image.")?;
            let image = image.make_raster_image(None, None).unwrap_or(image);
            let image = decode_size
                .and_then(|target| Self::downsample(&image, target, &sampling_mode))
                .unwrap_or(image);
            Ok((image, bytes))
        })
        .await
    }

    fn downsample(
        encoded: &SkImage,
        target: DecodeSize,
        sampling_mode: &SamplingMode,
    ) -> Option<SkImage> {
        let natural_width = encoded.width() as f32;
        let natural_height = encoded.height() as f32;
        let target_width = target.width as f32;
        let target_height = target.height as f32;
        if natural_width <= target_width && natural_height <= target_height {
            return None;
        }
        let ratio = (target_width / natural_width).min(target_height / natural_height);
        let width = (natural_width * ratio).round().max(1.);
        let height = (natural_height * ratio).round().max(1.);

        let mut surface = raster_n32_premul((width as i32, height as i32))?;
        let destination = SkRect::from_xywh(0., 0., width, height);
        let sampling = sampling_mode.sampling_options();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        surface.canvas().draw_image_rect_with_sampling_options(
            encoded,
            None,
            destination,
            sampling,
            &paint,
        );
        Some(surface.image_snapshot())
    }
}

/// How an [`ImageViewer`] picks its decode dimensions.
#[derive(Default, Clone, Debug, PartialEq, Copy)]
pub enum DecodeMode {
    /// Default. Decodes to the pixel-sized layout scaled by the window scale factor,
    /// falling back to the natural size for any other sizing (fill, percentages, auto).
    #[default]
    FromLayout,
    /// Decode at the image's natural size.
    Source,
    /// Decode to fit within the given size, preserving aspect ratio and never upscaling.
    Custom(Size2D),
}

impl DecodeMode {
    fn resolve(&self, layout: &LayoutData, scale_factor: f64) -> Option<DecodeSize> {
        let scale = scale_factor as f32;
        let size = match self {
            Self::Source => return None,
            Self::FromLayout => match (&layout.width, &layout.height) {
                (Size::Pixels(width), Size::Pixels(height)) => {
                    Size2D::new(width.get() * scale, height.get() * scale)
                }
                _ => return None,
            },
            Self::Custom(size) => *size,
        };
        Some(DecodeSize::new(
            size.width.round().max(1.) as u32,
            size.height.round().max(1.) as u32,
        ))
    }
}

/// Image viewer component.
///
/// Handles async loading, caching, and error states for images.
/// See [`ImageSource`] for all supported image sources.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let source: ImageSource = (
///         "rust-logo",
///         include_bytes!("../../../examples/rust_logo.png"),
///     )
///         .into();
///
///     ImageViewer::new(source)
/// }
/// # use freya::prelude::*;
/// # use freya_testing::prelude::*;
/// # use std::path::PathBuf;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(ImageViewer::new(("rust-logo", include_bytes!("../../../examples/rust_logo.png"))))
/// # }, "./images/gallery_image_viewer.png").with_hook(|t| { t.poll(std::time::Duration::from_millis(1), std::time::Duration::from_millis(300)); t.sync_and_update(); }).with_scale_factor(1.).render();
/// ```
///
/// # Preview
/// ![ImageViewer Preview][image_viewer]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("image_viewer", "images/gallery_image_viewer.png")
)]
#[derive(PartialEq)]
pub struct ImageViewer {
    source: ImageSource,
    asset_age: AssetAge,

    layout: LayoutData,
    image_data: ImageData,
    accessibility: AccessibilityData,
    effect: EffectData,
    corner_radius: Option<CornerRadius>,
    decode_mode: DecodeMode,

    children: Vec<Element>,
    loading_placeholder: Option<Element>,
    error_renderer: Option<Callback<String, Element>>,

    key: DiffKey,
}

impl ImageViewer {
    pub fn new(source: impl Into<ImageSource>) -> Self {
        ImageViewer {
            source: source.into(),
            asset_age: AssetAge::default(),
            layout: LayoutData::default(),
            image_data: ImageData::default(),
            accessibility: AccessibilityData::default(),
            effect: EffectData::default(),
            corner_radius: None,
            decode_mode: DecodeMode::default(),
            children: Vec::new(),
            loading_placeholder: None,
            error_renderer: None,
            key: DiffKey::None,
        }
    }
}

impl KeyExt for ImageViewer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for ImageViewer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for ImageViewer {}
impl ContainerWithContentExt for ImageViewer {}
impl ContainerPositionExt for ImageViewer {}

impl ImageExt for ImageViewer {
    fn get_image_data(&mut self) -> &mut ImageData {
        &mut self.image_data
    }
}

impl AccessibilityExt for ImageViewer {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl ChildrenExt for ImageViewer {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl EffectExt for ImageViewer {
    fn get_effect(&mut self) -> &mut EffectData {
        &mut self.effect
    }
}

impl ImageViewer {
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(corner_radius.into());
        self
    }

    /// Custom element rendered while loading.
    pub fn loading_placeholder(mut self, placeholder: impl Into<Element>) -> Self {
        self.loading_placeholder = Some(placeholder.into());
        self
    }

    /// Pick how the image is decoded. See [`DecodeMode`].
    pub fn decode_mode(mut self, decode_mode: DecodeMode) -> Self {
        self.decode_mode = decode_mode;
        self
    }

    /// Customize how long the image will remain cached after no longer being used.
    ///
    /// Defaults to [`AssetAge::default`] (1h).
    pub fn asset_age(mut self, asset_age: impl Into<AssetAge>) -> Self {
        self.asset_age = asset_age.into();
        self
    }

    /// Custom element rendered when the image fails to load.
    pub fn error_renderer(mut self, renderer: impl Into<Callback<String, Element>>) -> Self {
        self.error_renderer = Some(renderer.into());
        self
    }
}

impl Component for ImageViewer {
    fn render(&self) -> impl IntoElement {
        let target = self
            .decode_mode
            .resolve(&self.layout, *Platform::get().scale_factor.read());
        let sampling_mode = self.image_data.sampling_mode.clone();
        let asset_config =
            AssetConfiguration::new((&self.source, target, &sampling_mode), self.asset_age);
        let asset = use_asset(&asset_config);
        let mut asset_cacher = use_hook(AssetCacher::get);

        use_side_effect_with_deps(
            &(self.source.clone(), asset_config, target, sampling_mode),
            move |(source, asset_config, target, sampling_mode)| {
                if matches!(
                    asset_cacher.read_asset(asset_config),
                    Some(Asset::Pending) | Some(Asset::Error(_))
                ) {
                    asset_cacher.update_asset(asset_config.clone(), Asset::Loading);

                    let source = source.clone();
                    let asset_config = asset_config.clone();
                    let target = *target;
                    let sampling_mode = sampling_mode.clone();
                    spawn_forever(async move {
                        match source.load(target, sampling_mode).await {
                            Ok((image, bytes)) => {
                                asset_cacher.update_asset(
                                    asset_config,
                                    Asset::Cached(Rc::new(ImageHandle::new(image, bytes))),
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
                let asset = asset.downcast_ref::<ImageHandle>().unwrap().clone();
                image(asset)
                    .accessibility(self.accessibility.clone())
                    .a11y_role(AccessibilityRole::Image)
                    .layout(self.layout.clone())
                    .image_data(self.image_data.clone())
                    .effect(self.effect.clone())
                    .children(self.children.clone())
                    .map(self.corner_radius, |img, corner_radius| {
                        img.corner_radius(corner_radius)
                    })
                    .into_element()
            }
            Asset::Pending | Asset::Loading => rect()
                .layout(self.layout.clone())
                .center()
                .child(
                    self.loading_placeholder
                        .clone()
                        .unwrap_or_else(|| CircularLoader::new().into_element()),
                )
                .into(),
            Asset::Error(err) => match &self.error_renderer {
                Some(renderer) => renderer.call(err),
                None => err.into(),
            },
        }
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
