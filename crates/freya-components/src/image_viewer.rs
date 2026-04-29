use std::{
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    fs,
    hash::{
        Hash,
        Hasher,
    },
    path::PathBuf,
    rc::Rc,
};

use anyhow::Context;
use bytes::Bytes;
use freya_core::{
    elements::image::*,
    prelude::*,
};
use freya_engine::prelude::{
    SkData,
    SkImage,
};
#[cfg(feature = "remote-asset")]
use ureq::http::Uri;

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
    Uri(Uri),

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
        let mut hasher = DefaultHasher::default();
        id.hash(&mut hasher);
        Self::Bytes(hasher.finish(), Bytes::from_static(bytes))
    }
}

impl<const N: usize, H: Hash> From<(H, &'static [u8; N])> for ImageSource {
    fn from((id, bytes): (H, &'static [u8; N])) -> Self {
        let mut hasher = DefaultHasher::default();
        id.hash(&mut hasher);
        Self::Bytes(hasher.finish(), Bytes::from_static(bytes))
    }
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "remote-asset")))]
#[cfg(feature = "remote-asset")]
impl From<Uri> for ImageSource {
    fn from(uri: Uri) -> Self {
        Self::Uri(uri)
    }
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "remote-asset")))]
#[cfg(feature = "remote-asset")]
impl From<&'static str> for ImageSource {
    fn from(src: &'static str) -> Self {
        Self::Uri(Uri::from_static(src))
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

impl ImageSource {
    pub async fn bytes(&self) -> anyhow::Result<(SkImage, Bytes)> {
        let source = self.clone();
        blocking::unblock(move || {
            let bytes = match source {
                #[cfg(feature = "remote-asset")]
                Self::Uri(uri) => ureq::get(uri)
                    .call()?
                    .body_mut()
                    .read_to_vec()
                    .map(Bytes::from)?,
                Self::Path(path) => fs::read(path).map(Bytes::from)?,
                Self::Bytes(_, bytes) => bytes,
            };
            let image = SkImage::from_encoded(unsafe { SkData::new_bytes(&bytes) })
                .context("Failed to decode Image.")?;
            let image = image.make_raster_image(None, None).unwrap_or(image);
            Ok((image, bytes))
        })
        .await
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
/// # }, "./images/gallery_image_viewer.png").with_hook(|t| { t.poll(std::time::Duration::from_millis(1), std::time::Duration::from_millis(50)); t.sync_and_update(); }).with_scale_factor(1.).render();
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

    layout: LayoutData,
    image_data: ImageData,
    accessibility: AccessibilityData,
    effect: EffectData,
    corner_radius: Option<CornerRadius>,

    children: Vec<Element>,
    loading_placeholder: Option<Element>,
    error_renderer: Option<Callback<String, Element>>,

    key: DiffKey,
}

impl ImageViewer {
    pub fn new(source: impl Into<ImageSource>) -> Self {
        ImageViewer {
            source: source.into(),
            layout: LayoutData::default(),
            image_data: ImageData::default(),
            accessibility: AccessibilityData::default(),
            effect: EffectData::default(),
            corner_radius: None,
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

    /// Custom element rendered when the image fails to load.
    pub fn error_renderer(mut self, renderer: impl Into<Callback<String, Element>>) -> Self {
        self.error_renderer = Some(renderer.into());
        self
    }
}

impl Component for ImageViewer {
    fn render(&self) -> impl IntoElement {
        let asset_config = AssetConfiguration::new(&self.source, AssetAge::default());
        let asset = use_asset(&asset_config);
        let mut asset_cacher = use_hook(AssetCacher::get);

        use_side_effect_with_deps(
            &(self.source.clone(), asset_config),
            move |(source, asset_config): &(ImageSource, AssetConfiguration)| {
                // Fetch asset if still pending or errored. The Loading state
                // guards against duplicate in-flight fetches.
                if matches!(
                    asset_cacher.read_asset(asset_config),
                    Some(Asset::Pending) | Some(Asset::Error(_))
                ) {
                    asset_cacher.update_asset(asset_config.clone(), Asset::Loading);

                    let source = source.clone();
                    let asset_config = asset_config.clone();
                    spawn_forever(async move {
                        match source.bytes().await {
                            Ok((image, bytes)) => {
                                // Image loaded
                                let image_holder = ImageHolder {
                                    bytes,
                                    image: Rc::new(RefCell::new(image)),
                                };
                                asset_cacher.update_asset(
                                    asset_config,
                                    Asset::Cached(Rc::new(image_holder)),
                                );
                            }
                            Err(err) => {
                                // Image errored
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
                let asset = asset.downcast_ref::<ImageHolder>().unwrap().clone();
                image(asset)
                    .accessibility(self.accessibility.clone())
                    .a11y_role(AccessibilityRole::Image)
                    .a11y_focusable(true)
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
