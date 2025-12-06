use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    fs,
    hash::{
        Hash,
        Hasher,
    },
    path::PathBuf,
    rc::Rc,
    time::Duration,
};

use anyhow::Context;
use async_io::Timer;
use blocking::unblock;
use bytes::Bytes;
use freya_core::{
    elements::image::{
        AspectRatio,
        ImageData,
        SamplingMode,
    },
    integration::*,
    prelude::*,
};
use freya_engine::prelude::{
    AlphaType,
    ClipOp,
    Color,
    ColorType,
    CubicResampler,
    Data,
    FilterMode,
    ISize,
    ImageInfo,
    MipmapMode,
    Paint,
    Rect,
    SamplingOptions,
    SkImage,
    SkRect,
    raster_from_data,
    raster_n32_premul,
};
use gif::DisposalMethod;
use torin::prelude::Size2D;
#[cfg(feature = "remote-asset")]
use ureq::http::Uri;

use crate::{
    cache::*,
    loader::CircularLoader,
};

/// ### URI
///
/// Good to load remote GIFs.
///
/// > Needs the `remote-asset` feature enabled.
///
/// ```rust
/// # use freya::prelude::*;
/// let source: GifSource =
///     "https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExeXh5YWhscmo0YmF3OG1oMmpnMzBnbXFjcDR5Y2xoODE2ZnRpc2FhZiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/HTZVeK0esRjyw/giphy.gif"
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
/// let source: GifSource = PathBuf::from("./examples/frog_typing.gif").into();
/// ```
/// ### Raw bytes
///
/// Good for embedded GIFs.
///
/// ```rust
/// # use freya::prelude::*;
/// let source: GifSource = (
///     "frog-typing",
///     include_bytes!("../../../examples/frog_typing.gif"),
/// )
///     .into();
/// ```
#[derive(PartialEq, Clone)]
pub enum GifSource {
    #[cfg(feature = "remote-asset")]
    Uri(Uri),

    Path(PathBuf),

    Bytes(&'static str, Bytes),
}

impl From<(&'static str, Bytes)> for GifSource {
    fn from((id, bytes): (&'static str, Bytes)) -> Self {
        Self::Bytes(id, bytes)
    }
}

impl From<(&'static str, &'static [u8])> for GifSource {
    fn from((id, bytes): (&'static str, &'static [u8])) -> Self {
        Self::Bytes(id, Bytes::from_static(bytes))
    }
}

impl<const N: usize> From<(&'static str, &'static [u8; N])> for GifSource {
    fn from((id, bytes): (&'static str, &'static [u8; N])) -> Self {
        Self::Bytes(id, Bytes::from_static(bytes))
    }
}

#[cfg(feature = "remote-asset")]
impl From<Uri> for GifSource {
    fn from(uri: Uri) -> Self {
        Self::Uri(uri)
    }
}

#[cfg(feature = "remote-asset")]
impl From<&'static str> for GifSource {
    fn from(src: &'static str) -> Self {
        Self::Uri(Uri::from_static(src))
    }
}

impl From<PathBuf> for GifSource {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl Hash for GifSource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            #[cfg(feature = "remote-asset")]
            Self::Uri(uri) => uri.hash(state),
            Self::Path(path) => path.hash(state),
            Self::Bytes(id, _) => id.hash(state),
        }
    }
}

impl GifSource {
    pub async fn bytes(&self) -> anyhow::Result<Bytes> {
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
            Ok(bytes)
        })
        .await
    }
}

/// GIF viewer component.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let source: GifSource =
///         "https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExeXh5YWhscmo0YmF3OG1oMmpnMzBnbXFjcDR5Y2xoODE2ZnRpc2FhZiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/HTZVeK0esRjyw/giphy.gif"
///             .into();
///
///     GifViewer::new(source)
/// }
///
/// # use freya_testing::prelude::*;
/// # use std::path::PathBuf;
/// # launch_doc_hook(|| {
/// #   rect().center().expanded().child(GifViewer::new(("frog-typing", include_bytes!("../../../examples/frog_typing.gif"))))
/// # }, (250., 250.).into(), "./images/gallery_gif_viewer.png", |t| {
/// #   t.poll(std::time::Duration::from_millis(1),std::time::Duration::from_millis(50));
/// #   t.sync_and_update();
/// # });
/// ```
///
/// # Preview
/// ![Gif Preview][gif_viewer]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("gif_viewer", "images/gallery_gif_viewer.png")
)]
#[derive(PartialEq)]
pub struct GifViewer {
    source: GifSource,

    layout: LayoutData,
    image_data: ImageData,
    accessibility: AccessibilityData,
}

impl GifViewer {
    pub fn new(source: impl Into<GifSource>) -> Self {
        GifViewer {
            source: source.into(),
            layout: LayoutData::default(),
            image_data: ImageData::default(),
            accessibility: AccessibilityData::default(),
        }
    }
}

impl LayoutExt for GifViewer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ImageExt for GifViewer {
    fn get_image_data(&mut self) -> &mut ImageData {
        &mut self.image_data
    }
}

impl AccessibilityExt for GifViewer {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl Render for GifViewer {
    fn render(&self) -> impl IntoElement {
        let asset_config = AssetConfiguration::new(&self.source, AssetAge::default());
        let asset_data = use_asset(&asset_config);
        let mut asset = use_state::<Option<GifData>>(|| None);
        let mut asset_cacher = use_hook(AssetCacher::get);
        let mut assets_tasks = use_state::<Vec<TaskHandle>>(Vec::new);

        let mut stream_gif = async move |bytes: Bytes| -> anyhow::Result<()> {
            loop {
                let mut decoder_options = gif::DecodeOptions::new();
                decoder_options.set_color_output(gif::ColorOutput::RGBA);
                let cursor = std::io::Cursor::new(&bytes);
                let mut decoder = decoder_options.read_info(cursor.clone())?;
                let surface = raster_n32_premul((decoder.width() as i32, decoder.height() as i32))
                    .context("Failed to create GIF surface")?;
                loop {
                    match decoder.read_next_frame() {
                        Ok(Some(frame)) => {
                            // Render new frame
                            let row_bytes = (frame.width * 4) as usize;
                            let data = unsafe { Data::new_bytes(&frame.buffer) };
                            let isize = ISize::new(frame.width as i32, frame.height as i32);
                            let gif = unblock(move || {
                                raster_from_data(
                                    &ImageInfo::new(
                                        isize,
                                        ColorType::RGBA8888,
                                        AlphaType::Unpremul,
                                        None,
                                    ),
                                    data,
                                    row_bytes,
                                )
                                .context("Failed to crate GIF Frame.")
                            })
                            .await?;
                            *asset.write() = Some(GifData {
                                holder: Rc::new(RefCell::new(gif)),
                                surface: Rc::new(RefCell::new(surface.clone())),
                                dispose: frame.dispose,
                                left: frame.left as f32,
                                top: frame.top as f32,
                                width: frame.width as f32,
                                height: frame.height as f32,
                            });

                            let duration = Duration::from_millis(frame.delay as u64 * 10);
                            Timer::after(duration).await;
                        }

                        Ok(None) => {
                            // No more framess, so we repeat
                            break;
                        }
                        // TODO: Something went wrong
                        Err(_e) => {
                            break;
                        }
                    }
                }
            }
        };

        use_side_effect_with_deps(&self.source, {
            let asset_config = asset_config.clone();
            move |source| {
                let source = source.clone();

                // Cancel previous tasks
                for asset_task in assets_tasks.write().drain(..) {
                    asset_task.cancel();
                }

                match asset_cacher.read_asset(&asset_config) {
                    Some(Asset::Pending) | Some(Asset::Error(_)) => {
                        // Mark asset as loading
                        asset_cacher.update_asset(asset_config.clone(), Asset::Loading);

                        let asset_config = asset_config.clone();
                        let asset_task = spawn(async move {
                            match source.bytes().await {
                                Ok(bytes) => {
                                    // Cache the GIF bytes
                                    asset_cacher.update_asset(
                                        asset_config,
                                        Asset::Cached(Rc::new(bytes.clone())),
                                    );
                                }
                                Err(err) => {
                                    asset_cacher
                                        .update_asset(asset_config, Asset::Error(err.to_string()));
                                }
                            }
                        });

                        assets_tasks.write().push(asset_task);
                    }
                    _ => {}
                }
            }
        });

        use_side_effect(move || {
            if let Some(Asset::Cached(asset)) = asset_cacher.subscribe_asset(&asset_config) {
                if let Some(bytes) = asset.downcast_ref::<Bytes>().cloned() {
                    let asset_task = spawn(async move {
                        if let Err(err) = stream_gif(bytes).await {
                            #[cfg(debug_assertions)]
                            tracing::error!(
                                "Failed to render GIF by ID <{}>, error: {err:?}",
                                asset_config.id
                            )
                        }
                    });
                    assets_tasks.write().push(asset_task);
                } else {
                    #[cfg(debug_assertions)]
                    tracing::error!(
                        "Failed to downcast asset of GIF by ID <{}>",
                        asset_config.id
                    )
                }
            }
        });

        match (asset_data, asset.read().clone()) {
            (Asset::Cached(_), Some(asset)) => gif(asset)
                .accessibility(self.accessibility.clone())
                .a11y_role(AccessibilityRole::Image)
                .a11y_focusable(true)
                .layout(self.layout.clone())
                .image_data(self.image_data.clone())
                .into_element(),
            (Asset::Cached(_), _) | (Asset::Pending | Asset::Loading, _) => rect()
                .layout(self.layout.clone())
                .center()
                .child(CircularLoader::new())
                .into(),
            (Asset::Error(err), _) => err.into(),
        }
    }
}

pub struct Gif {
    key: DiffKey,
    element: GifElement,
}

impl Gif {
    pub fn try_downcast(element: &dyn ElementExt) -> Option<GifElement> {
        (element as &dyn Any).downcast_ref::<GifElement>().cloned()
    }
}

impl From<Gif> for Element {
    fn from(value: Gif) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: vec![],
        }
    }
}

fn gif(gif_data: GifData) -> Gif {
    Gif {
        key: DiffKey::None,
        element: GifElement {
            gif_data,
            accessibility: AccessibilityData::default(),
            layout: LayoutData::default(),
            event_handlers: HashMap::default(),
            image_data: ImageData::default(),
        },
    }
}

impl LayoutExt for Gif {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerExt for Gif {}

impl ImageExt for Gif {
    fn get_image_data(&mut self) -> &mut ImageData {
        &mut self.element.image_data
    }
}

impl KeyExt for Gif {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl EventHandlersExt for Gif {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.element.event_handlers
    }
}

impl AccessibilityExt for Gif {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.element.accessibility
    }
}
impl MaybeExt for Gif {}

#[derive(PartialEq, Clone)]
pub struct GifElement {
    accessibility: AccessibilityData,
    layout: LayoutData,
    event_handlers: FxHashMap<EventName, EventHandlerType>,
    gif_data: GifData,
    image_data: ImageData,
}

impl ElementExt for GifElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(image) = (other.as_ref() as &dyn Any).downcast_ref::<GifElement>() else {
            return false;
        };
        self != image
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(image) = (other.as_ref() as &dyn Any).downcast_ref::<GifElement>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.accessibility != image.accessibility {
            diff.insert(DiffModifies::ACCESSIBILITY);
        }

        if self.layout != image.layout {
            diff.insert(DiffModifies::LAYOUT);
        }

        if self.gif_data != image.gif_data {
            diff.insert(DiffModifies::LAYOUT);
            diff.insert(DiffModifies::STYLE);
        }

        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout)
    }

    fn effect(&'_ self) -> Option<Cow<'_, EffectData>> {
        None
    }

    fn style(&'_ self) -> Cow<'_, StyleState> {
        Cow::Owned(StyleState::default())
    }

    fn text_style(&'_ self) -> Cow<'_, TextStyleData> {
        Cow::Owned(TextStyleData::default())
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Borrowed(&self.accessibility)
    }

    fn should_measure_inner_children(&self) -> bool {
        false
    }

    fn should_hook_measurement(&self) -> bool {
        true
    }

    fn measure(&self, context: LayoutContext) -> Option<(Size2D, Rc<dyn Any>)> {
        let image = self.gif_data.holder.borrow();

        let image_width = image.width() as f32;
        let image_height = image.height() as f32;

        let width_ratio = context.area_size.width / image.width() as f32;
        let height_ratio = context.area_size.height / image.height() as f32;

        let size = match self.image_data.aspect_ratio {
            AspectRatio::Max => {
                let ratio = width_ratio.max(height_ratio);

                Size2D::new(image_width * ratio, image_height * ratio)
            }
            AspectRatio::Min => {
                let ratio = width_ratio.min(height_ratio);

                Size2D::new(image_width * ratio, image_height * ratio)
            }
            AspectRatio::Fit => Size2D::new(image_width, image_height),
            AspectRatio::None => *context.area_size,
        };

        Some((size, Rc::new(())))
    }

    fn clip(&self, context: ClipContext) {
        let area = context.visible_area;
        context.canvas.clip_rect(
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            ClipOp::Intersect,
            true,
        );
    }

    fn render(&self, context: RenderContext) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);

        let sampling = match self.image_data.sampling_mode {
            SamplingMode::Nearest => SamplingOptions::new(FilterMode::Nearest, MipmapMode::None),
            SamplingMode::Bilinear => SamplingOptions::new(FilterMode::Linear, MipmapMode::None),
            SamplingMode::Trilinear => SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear),
            SamplingMode::Mitchell => SamplingOptions::from(CubicResampler::mitchell()),
            SamplingMode::CatmullRom => SamplingOptions::from(CubicResampler::catmull_rom()),
        };

        let rect = SkRect::new(
            context.layout_node.area.min_x(),
            context.layout_node.area.min_y(),
            context.layout_node.area.max_x(),
            context.layout_node.area.max_y(),
        );

        let frame = self.gif_data.holder.borrow();

        if self.gif_data.dispose == DisposalMethod::Background {
            let rect = Rect::from_xywh(
                self.gif_data.left,
                self.gif_data.top,
                self.gif_data.width,
                self.gif_data.height,
            );
            context.canvas.save();
            context.canvas.clip_rect(rect, None, false);
            context.canvas.clear(Color::TRANSPARENT);
            context.canvas.restore();
        }

        self.gif_data.surface.borrow_mut().canvas().draw_image(
            &*frame,
            (self.gif_data.left, self.gif_data.top),
            None,
        );

        context.canvas.draw_image_rect_with_sampling_options(
            self.gif_data.surface.borrow_mut().image_snapshot(),
            None,
            rect,
            sampling,
            &paint,
        );
    }
}

#[derive(Clone)]
struct GifData {
    holder: Rc<RefCell<SkImage>>,
    surface: Rc<RefCell<freya_engine::prelude::Surface>>,
    dispose: DisposalMethod,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
}

impl PartialEq for GifData {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.holder, &other.holder)
    }
}
