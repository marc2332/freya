use std::{
    any::Any,
    borrow::Cow,
    collections::HashMap,
    rc::Rc,
};

use freya_components::loader::CircularLoader;
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
    ClipOp,
    CubicResampler,
    FilterMode,
    MipmapMode,
    Paint,
    SamplingOptions,
    SkRect,
};
use torin::prelude::Size2D;

use crate::{
    PlaybackState,
    VideoFrame,
    VideoPlayer,
};

/// Renders the current frame of a [`VideoPlayer`].
#[derive(PartialEq)]
pub struct VideoViewer {
    player: VideoPlayer,

    layout: LayoutData,
    image_data: ImageData,
    accessibility: AccessibilityData,

    key: DiffKey,
}

impl VideoViewer {
    pub fn new(player: VideoPlayer) -> Self {
        Self {
            player,
            layout: LayoutData::default(),
            image_data: ImageData::default(),
            accessibility: AccessibilityData::default(),
            key: DiffKey::None,
        }
    }
}

impl KeyExt for VideoViewer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for VideoViewer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for VideoViewer {}

impl ImageExt for VideoViewer {
    fn get_image_data(&mut self) -> &mut ImageData {
        &mut self.image_data
    }
}

impl AccessibilityExt for VideoViewer {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl Component for VideoViewer {
    fn render(&self) -> impl IntoElement {
        match (self.player.frame(), self.player.state()) {
            (Some(frame), _) => video(frame)
                .accessibility(self.accessibility.clone())
                .a11y_role(AccessibilityRole::Video)
                .a11y_focusable(true)
                .layout(self.layout.clone())
                .image_data(self.image_data.clone())
                .into_element(),
            (None, PlaybackState::Errored) => "Failed to decode video".into_element(),
            (None, _) => rect()
                .layout(self.layout.clone())
                .center()
                .child(CircularLoader::new())
                .into(),
        }
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Low-level element that paints a single [`VideoFrame`]. Prefer [`VideoViewer`].
pub fn video(frame: VideoFrame) -> Video {
    Video {
        key: DiffKey::None,
        element: VideoElement {
            frame,
            accessibility: AccessibilityData::default(),
            layout: LayoutData::default(),
            event_handlers: HashMap::default(),
            image_data: ImageData::default(),
        },
    }
}

pub struct Video {
    key: DiffKey,
    element: VideoElement,
}

impl Video {
    pub fn try_downcast(element: &dyn ElementExt) -> Option<VideoElement> {
        (element as &dyn Any)
            .downcast_ref::<VideoElement>()
            .cloned()
    }
}

impl From<Video> for Element {
    fn from(value: Video) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: vec![],
        }
    }
}

impl KeyExt for Video {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for Video {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerExt for Video {}

impl ImageExt for Video {
    fn get_image_data(&mut self) -> &mut ImageData {
        &mut self.element.image_data
    }
}

impl AccessibilityExt for Video {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.element.accessibility
    }
}

impl EventHandlersExt for Video {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.element.event_handlers
    }
}

impl MaybeExt for Video {}

#[derive(Clone)]
pub struct VideoElement {
    accessibility: AccessibilityData,
    layout: LayoutData,
    event_handlers: FxHashMap<EventName, EventHandlerType>,
    frame: VideoFrame,
    image_data: ImageData,
}

impl PartialEq for VideoElement {
    fn eq(&self, other: &Self) -> bool {
        self.accessibility == other.accessibility
            && self.layout == other.layout
            && self.image_data == other.image_data
            && self.frame == other.frame
    }
}

impl ElementExt for VideoElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(video) = (other.as_ref() as &dyn Any).downcast_ref::<VideoElement>() else {
            return false;
        };
        self != video
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(video) = (other.as_ref() as &dyn Any).downcast_ref::<VideoElement>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.accessibility != video.accessibility {
            diff.insert(DiffModifies::ACCESSIBILITY);
        }

        if self.layout != video.layout {
            diff.insert(DiffModifies::LAYOUT);
        }

        if self.frame != video.frame {
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
        let image = self.frame.image();
        let image_width = image.width() as f32;
        let image_height = image.height() as f32;

        let width_ratio = context.area_size.width / image_width;
        let height_ratio = context.area_size.height / image_height;
        let scaled = |ratio: f32| Size2D::new(image_width * ratio, image_height * ratio);

        let size = match self.image_data.aspect_ratio {
            AspectRatio::Max => scaled(width_ratio.max(height_ratio)),
            AspectRatio::Min => scaled(width_ratio.min(height_ratio)),
            AspectRatio::Fit => Size2D::new(image_width, image_height),
            AspectRatio::None => *context.area_size,
        };

        Some((size, Rc::new(size)))
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
            let offset_x = (size.width - area.width()) / 2.;
            let offset_y = (size.height - area.height()) / 2.;
            rect.left -= offset_x;
            rect.right -= offset_x;
            rect.top -= offset_y;
            rect.bottom -= offset_y;
        }

        let mut paint = Paint::default();
        paint.set_anti_alias(true);

        context.canvas.save();
        context.canvas.clip_rect(
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            ClipOp::Intersect,
            true,
        );
        context.canvas.draw_image_rect_with_sampling_options(
            self.frame.image(),
            None,
            rect,
            sampling_options(&self.image_data.sampling_mode),
            &paint,
        );
        context.canvas.restore();
    }
}

fn sampling_options(mode: &SamplingMode) -> SamplingOptions {
    match mode {
        SamplingMode::Nearest => SamplingOptions::new(FilterMode::Nearest, MipmapMode::None),
        SamplingMode::Bilinear => SamplingOptions::new(FilterMode::Linear, MipmapMode::None),
        SamplingMode::Trilinear => SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear),
        SamplingMode::Mitchell => SamplingOptions::from(CubicResampler::mitchell()),
        SamplingMode::CatmullRom => SamplingOptions::from(CubicResampler::catmull_rom()),
    }
}
