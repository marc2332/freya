use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use bytes::Bytes;
use freya_engine::prelude::{
    ClipOp,
    CubicResampler,
    FilterMode,
    MipmapMode,
    Paint,
    SamplingOptions,
    SkImage,
    SkRect,
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
    prelude::{
        AccessibilityExt,
        ChildrenExt,
        ContainerExt,
        ContainerWithContentExt,
        EventHandlersExt,
        ImageExt,
        KeyExt,
        LayerExt,
        LayoutExt,
        MaybeExt,
    },
    tree::DiffModifies,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub enum AspectRatio {
    #[default]
    Min,
    Max,
    Fit,
    None,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum SamplingMode {
    #[default]
    Nearest,
    Bilinear,
    Trilinear,
    Mitchell,
    CatmullRom,
}

#[derive(Clone)]
pub struct ImageHolder {
    pub image: Rc<RefCell<SkImage>>,
    pub bytes: Bytes,
}

impl PartialEq for ImageHolder {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.image, &other.image)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageData {
    pub sampling_mode: SamplingMode,
    pub aspect_ratio: AspectRatio,
}

#[derive(PartialEq, Clone)]
pub struct ImageElement {
    pub accessibility: AccessibilityData,
    pub layout: LayoutData,
    pub event_handlers: FxHashMap<EventName, EventHandlerType>,
    pub image_holder: ImageHolder,
    pub image_data: ImageData,
    pub relative_layer: i16,
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

        if self.image_holder != image.image_holder {
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

    fn relative_layer(&self) -> i16 {
        self.relative_layer
    }

    fn should_measure_inner_children(&self) -> bool {
        true
    }

    fn should_hook_measurement(&self) -> bool {
        true
    }

    fn measure(&self, context: LayoutContext) -> Option<(Size2D, Rc<dyn Any>)> {
        let image = self.image_holder.image.borrow();

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

        let image = self.image_holder.image.borrow();

        context
            .canvas
            .draw_image_rect_with_sampling_options(&*image, None, rect, sampling, &paint);
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
    fn get_layer(&mut self) -> &mut i16 {
        &mut self.element.relative_layer
    }
}

pub struct Image {
    key: DiffKey,
    element: ImageElement,
    elements: Vec<Element>,
}

/// [image] makes it possible to render a Skia image into the canvas.
/// You most likely want to use a higher level than this, like the component `ImageViewer`.
///
/// See the available methods in [Image].
pub fn image(image_holder: ImageHolder) -> Image {
    Image {
        key: DiffKey::None,
        element: ImageElement {
            image_holder,
            accessibility: AccessibilityData::default(),
            layout: LayoutData::default(),
            event_handlers: HashMap::default(),
            image_data: ImageData::default(),
            relative_layer: 0,
        },
        elements: Vec::new(),
    }
}

impl Image {
    pub fn try_downcast(element: &dyn ElementExt) -> Option<ImageElement> {
        (element as &dyn Any)
            .downcast_ref::<ImageElement>()
            .cloned()
    }
}
