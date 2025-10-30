use std::{
    cell::RefCell,
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

/// ### URI
///
/// Good to load remote images.
///
/// > Needs the `remote-asset` feature enabled.
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
#[derive(PartialEq, Clone)]
pub enum ImageSource {
    #[cfg(feature = "remote-asset")]
    Uri(Uri),

    Path(PathBuf),

    Bytes(&'static str, Bytes),
}

impl From<(&'static str, Bytes)> for ImageSource {
    fn from((id, bytes): (&'static str, Bytes)) -> Self {
        Self::Bytes(id, bytes)
    }
}

impl From<(&'static str, &'static [u8])> for ImageSource {
    fn from((id, bytes): (&'static str, &'static [u8])) -> Self {
        Self::Bytes(id, Bytes::from_static(bytes))
    }
}

impl<const N: usize> From<(&'static str, &'static [u8; N])> for ImageSource {
    fn from((id, bytes): (&'static str, &'static [u8; N])) -> Self {
        Self::Bytes(id, Bytes::from_static(bytes))
    }
}

#[cfg(feature = "remote-asset")]
impl From<Uri> for ImageSource {
    fn from(uri: Uri) -> Self {
        Self::Uri(uri)
    }
}

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
    pub async fn bytes(&self) -> anyhow::Result<SkImage> {
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
                Self::Bytes(_, bytes) => bytes.clone(),
            };
            let image = SkImage::from_encoded(unsafe { SkData::new_bytes(&bytes) })
                .context("Failed to decode Image.")?;
            Ok(image)
        })
        .await
    }
}

/// Image viewer component.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let source: ImageSource =
///         "https://upload.wikimedia.org/wikipedia/commons/8/8a/Gecarcinus_quadratus_%28Nosara%29.jpg"
///             .into();
///
///     ImageViewer::new(source)
///         .into()
/// }
///
/// # use freya_testing::prelude::*;
/// # use std::path::PathBuf;
/// # launch_doc_hook(|| {
/// #   rect().center().expanded().child(ImageViewer::new(("rust-logo", include_bytes!("../../../examples/rust_logo.png")))).into()
/// # }, (250., 250.).into(), "./images/gallery_image_viewer.png", |t| {
/// #   t.poll(std::time::Duration::from_millis(1),std::time::Duration::from_millis(50));
/// #   t.sync_and_update();
/// # });
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
}

impl ImageViewer {
    pub fn new(source: impl Into<ImageSource>) -> Self {
        ImageViewer {
            source: source.into(),
            layout: LayoutData::default(),
            image_data: ImageData::default(),
            accessibility: AccessibilityData::default(),
        }
    }
}

impl LayoutExt for ImageViewer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

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

impl Render for ImageViewer {
    fn render(&self) -> Element {
        let asset_config = AssetConfiguration::new(&self.source, AssetAge::default());
        let asset = use_asset(&asset_config);
        let mut asset_cacher = use_hook(AssetCacher::get);
        let mut assets_tasks = use_state::<Vec<TaskHandle>>(Vec::new);

        use_side_effect_with_deps(&self.source, move |source| {
            let source = source.clone();

            // Cancel previous asset fetching requests
            for asset_task in assets_tasks.write().drain(..) {
                asset_task.cancel();
            }

            // Fetch asset if still pending or errored
            if matches!(
                asset_cacher.read_asset(&asset_config),
                Some(Asset::Pending) | Some(Asset::Error(_))
            ) {
                // Mark asset as loading
                asset_cacher.update_asset(asset_config.clone(), Asset::Loading);

                let asset_config = asset_config.clone();
                let asset_task = spawn(async move {
                    match source.bytes().await {
                        Ok(image) => {
                            // Image loaded
                            let image_holder = ImageHolder(Rc::new(RefCell::new(image)));
                            asset_cacher.update_asset(
                                asset_config.clone(),
                                Asset::Cached(Rc::new(image_holder)),
                            );
                        }
                        Err(err) => {
                            // Image errored asset_cacher
                            asset_cacher.update_asset(asset_config, Asset::Error(err.to_string()));
                        }
                    }
                });

                assets_tasks.write().push(asset_task);
            }
        });

        match asset {
            Asset::Cached(asset) => {
                let asset = asset.downcast_ref::<ImageHolder>().unwrap().clone();
                image(asset)
                    .accessibility(self.accessibility.clone())
                    .a11y_role(AccessibilityRole::Image)
                    .a11y_focusable(true)
                    .layout(self.layout.clone())
                    .image_data(self.image_data.clone())
                    .into()
            }
            Asset::Pending | Asset::Loading => rect()
                .layout(self.layout.clone())
                .center()
                .child(CircularLoader::new())
                .into(),
            Asset::Error(err) => err.into(),
        }
    }
}
