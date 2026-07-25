//! [image()] makes it possible to render a Skia image into the canvas.

use std::{
    any::Any,
    borrow::Cow,
    collections::HashMap,
    rc::Rc,
};

use bytes::Bytes;
use freya_engine::prelude::{
    AlphaType,
    ClipOp,
    ColorType,
    CubicResampler,
    Data,
    FilterMode,
    ISize,
    ImageInfo,
    MipmapMode,
    Paint,
    SamplingOptions,
    SkImage,
    SkRect,
    raster_from_data,
};
use rustc_hash::FxHashMap;
use torin::prelude::Size2D;

use crate::{
    data::{
        AccessibilityData,
        EffectData,
        LayoutData,
        StyleState,
        TextStyleData,
    },
    diff_key::DiffKey,
    element::{
        ClipContext,
        Element,
        ElementExt,
        EventHandlerType,
        LayoutContext,
        RenderContext,
    },
    events::name::EventName,
    layers::Layer,
    prelude::{
        AccessibilityExt,
        ChildrenExt,
        ContainerExt,
        ContainerWithContentExt,
        EffectExt,
        EventHandlersExt,
        ImageExt,
        KeyExt,
        LayerExt,
        LayoutExt,
        MaybeExt,
    },
    style::corner_radius::CornerRadius,
    tree::DiffModifies,
};

/// [image] makes it possible to render a Skia image into the canvas.
/// You most likely want to use a higher level than this, like the component `ImageViewer`.
///
/// See the available methods in [Image].
pub fn image(image_handle: ImageHandle) -> Image {
    let mut accessibility = AccessibilityData::default();
    accessibility.builder.set_role(accesskit::Role::Image);
    Image {
        key: DiffKey::None,
        element: ImageElement {
            image_handle,
            accessibility,
            layout: LayoutData::default(),
            event_handlers: HashMap::default(),
            image_data: ImageData::default(),
            relative_layer: Layer::default(),
            effect: None,
            corner_radius: None,
        },
        elements: Vec::new(),
    }
}

/// How an image is positioned within its bounds once it has been scaled.
#[derive(Default, Clone, Debug, PartialEq)]
pub enum ImageCover {
    /// Anchor the image to the top-left of the bounds. This is the default.
    #[default]
    Fill,
    /// Center the image within the bounds.
    Center,
}

/// How an image is scaled to fit its bounds while preserving its aspect ratio.
#[derive(Default, Clone, Debug, PartialEq)]
pub enum AspectRatio {
    /// Scale so the whole image fits inside the bounds. This is the default.
    #[default]
    Min,
    /// Scale so the image covers the whole bounds, cropping the overflow.
    Max,
    /// Keep the image at its natural size.
    Fit,
    /// Stretch the image to the bounds, ignoring its aspect ratio.
    None,
}

/// The filtering algorithm used when an image is scaled.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum SamplingMode {
    /// Nearest-neighbor, fastest and sharpest, best for pixel art.
    Nearest,
    /// Bilinear filtering.
    Bilinear,
    /// Trilinear filtering with mipmaps. This is the default.
    #[default]
    Trilinear,
    /// Mitchell-Netravali cubic resampling, a smooth high-quality filter.
    Mitchell,
    /// Catmull-Rom cubic resampling, a sharper high-quality filter.
    CatmullRom,
}

impl SamplingMode {
    /// The Skia [`SamplingOptions`] backing this filtering algorithm.
    pub fn sampling_options(&self) -> SamplingOptions {
        match self {
            Self::Nearest => SamplingOptions::new(FilterMode::Nearest, MipmapMode::None),
            Self::Bilinear => SamplingOptions::new(FilterMode::Linear, MipmapMode::None),
            Self::Trilinear => SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear),
            Self::Mitchell => SamplingOptions::from(CubicResampler::mitchell()),
            Self::CatmullRom => SamplingOptions::from(CubicResampler::catmull_rom()),
        }
    }
}

/// A decoded image shared by reference, ready to be rendered by an [`image()`].
#[derive(Clone)]
pub struct ImageHandle {
    pub image: SkImage,
    /// Backing data of the [`SkImage`], kept alive for as long as the image is used.
    pub bytes: Bytes,
}

impl ImageHandle {
    pub fn new(image: SkImage, bytes: Bytes) -> Self {
        Self { image, bytes }
    }

    /// Build a handle from a raw `RGBA8888` pixel buffer, validating its length.
    pub fn from_rgba(width: u32, height: u32, bytes: Bytes, alpha_type: AlphaType) -> Option<Self> {
        let row_bytes = (width as usize).checked_mul(4)?;
        if bytes.len() < row_bytes.checked_mul(height as usize)? {
            return None;
        }
        let info = ImageInfo::new(
            ISize::new(width as i32, height as i32),
            ColorType::RGBA8888,
            alpha_type,
            None,
        );
        // Safety: `bytes` outlives the SkImage because the returned handle owns it.
        let data = unsafe { Data::new_bytes(&bytes) };
        let image = raster_from_data(&info, data, row_bytes)?;
        Some(Self::new(image, bytes))
    }
}

impl PartialEq for ImageHandle {
    fn eq(&self, other: &Self) -> bool {
        self.image.unique_id() == other.image.unique_id()
    }
}

/// How an [`image()`] is scaled and sampled, grouping [`SamplingMode`], [`AspectRatio`] and [`ImageCover`].
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageData {
    pub sampling_mode: SamplingMode,
    pub aspect_ratio: AspectRatio,
    pub image_cover: ImageCover,
}

#[derive(PartialEq, Clone)]
pub struct ImageElement {
    pub accessibility: AccessibilityData,
    pub layout: LayoutData,
    pub event_handlers: FxHashMap<EventName, EventHandlerType>,
    pub image_handle: ImageHandle,
    pub image_data: ImageData,
    pub relative_layer: Layer,
    pub effect: Option<EffectData>,
    pub corner_radius: Option<CornerRadius>,
}

impl ElementExt for ImageElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(image) = (other.as_ref() as &dyn Any).downcast_ref::<ImageElement>() else {
            return false;
        };
        self != image
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(image) = (other.as_ref() as &dyn Any).downcast_ref::<ImageElement>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.accessibility != image.accessibility {
            diff.insert(DiffModifies::ACCESSIBILITY);
        }

        if self.relative_layer != image.relative_layer {
            diff.insert(DiffModifies::LAYER);
        }

        if self.layout != image.layout {
            diff.insert(DiffModifies::LAYOUT);
        }

        if self.image_handle != image.image_handle {
            diff.insert(DiffModifies::STYLE);

            if self.image_handle.image.dimensions() != image.image_handle.image.dimensions() {
                diff.insert(DiffModifies::LAYOUT);
            }
        }

        if self.effect != image.effect {
            diff.insert(DiffModifies::EFFECT);
        }

        if self.corner_radius != image.corner_radius {
            diff.insert(DiffModifies::STYLE);
        }

        if self.event_handlers != image.event_handlers {
            diff.insert(DiffModifies::EVENT_HANDLERS);
        }

        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout)
    }

    fn effect(&'_ self) -> Option<Cow<'_, EffectData>> {
        self.effect.as_ref().map(Cow::Borrowed)
    }

    fn style(&'_ self) -> Cow<'_, StyleState> {
        Cow::Owned(StyleState {
            corner_radius: self.corner_radius.unwrap_or_default(),
            ..StyleState::default()
        })
    }

    fn text_style(&'_ self) -> Cow<'_, TextStyleData> {
        Cow::Owned(TextStyleData::default())
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Borrowed(&self.accessibility)
    }

    fn layer(&self) -> Layer {
        self.relative_layer
    }

    fn events_handlers(&'_ self) -> Option<Cow<'_, FxHashMap<EventName, EventHandlerType>>> {
        Some(Cow::Borrowed(&self.event_handlers))
    }

    fn should_measure_inner_children(&self) -> bool {
        true
    }

    fn should_hook_measurement(&self) -> bool {
        true
    }

    fn measure(&self, context: LayoutContext) -> Option<(Size2D, Rc<dyn Any>)> {
        let image = &self.image_handle.image;

        let image_width = image.width() as f32;
        let image_height = image.height() as f32;

        let area_size = (*context.area_size - context.torin_node.margin.into()).max(Size2D::zero());

        let width_ratio = area_size.width / image_width;
        let height_ratio = area_size.height / image_height;

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
            AspectRatio::None => area_size,
        };

        Some((size, Rc::new(size)))
    }

    fn clip(&self, context: ClipContext) {
        let rrect = self.render_rect(context.visible_area, context.scale_factor as f32);
        context.canvas.clip_rrect(rrect, ClipOp::Intersect, true);
    }

    fn render(&self, context: RenderContext) {
        let size = context
            .layout_node
            .data
            .as_ref()
            .unwrap()
            .downcast_ref::<Size2D>()
            .unwrap();

        let area = context.layout_node.visible_area();

        let mut rect = SkRect::new(
            area.min_x(),
            area.min_y(),
            area.min_x() + size.width,
            area.min_y() + size.height,
        );
        if self.image_data.image_cover == ImageCover::Center {
            let width_offset = (size.width - area.width()) / 2.;
            let height_offset = (size.height - area.height()) / 2.;

            rect.left -= width_offset;
            rect.right -= width_offset;
            rect.top -= height_offset;
            rect.bottom -= height_offset;
        }

        context.canvas.save();
        let clip_rrect = self.render_rect(&area, context.scale_factor as f32);
        context
            .canvas
            .clip_rrect(clip_rrect, ClipOp::Intersect, true);

        let sampling = self.image_data.sampling_mode.sampling_options();

        let mut paint = Paint::default();
        paint.set_anti_alias(true);

        context.canvas.draw_image_rect_with_sampling_options(
            &self.image_handle.image,
            None,
            rect,
            sampling,
            &paint,
        );

        context.canvas.restore();
    }
}

impl From<Image> for Element {
    fn from(value: Image) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: value.elements,
        }
    }
}

impl KeyExt for Image {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl EventHandlersExt for Image {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.element.event_handlers
    }
}

impl AccessibilityExt for Image {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.element.accessibility
    }
}
impl MaybeExt for Image {}

impl LayoutExt for Image {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerExt for Image {}
impl ContainerWithContentExt for Image {}

impl ImageExt for Image {
    fn get_image_data(&mut self) -> &mut ImageData {
        &mut self.element.image_data
    }
}

impl ChildrenExt for Image {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl LayerExt for Image {
    fn get_layer(&mut self) -> &mut Layer {
        &mut self.element.relative_layer
    }
}

impl EffectExt for Image {
    fn get_effect(&mut self) -> &mut EffectData {
        self.element.effect.get_or_insert_with(EffectData::default)
    }
}

pub struct Image {
    key: DiffKey,
    element: ImageElement,
    elements: Vec<Element>,
}

impl Image {
    pub fn try_downcast(element: &dyn ElementExt) -> Option<ImageElement> {
        (element as &dyn Any)
            .downcast_ref::<ImageElement>()
            .cloned()
    }

    /// Round the image's corners, clipping it to the rounded shape. See [`CornerRadius`].
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.element.corner_radius = Some(corner_radius.into());
        self
    }
}
