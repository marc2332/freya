use std::{any::Any, borrow::Cow, rc::Rc};

use freya_core::{integration::*, prelude::*};
use freya_engine::prelude::{
    ClipOp, Color, FilterMode, MipmapMode, Paint, PaintStyle, PathBuilder, SamplingOptions,
    SkImage, SkRect,
};

#[derive(Clone)]
pub(crate) struct VideoFrameElement {
    pub layout: LayoutData,
    pub current_frame: Option<Rc<SkImage>>,
    pub show_overlay: bool,
    pub is_paused: bool,
}

impl PartialEq for VideoFrameElement {
    fn eq(&self, other: &Self) -> bool {
        self.layout == other.layout
            && self.show_overlay == other.show_overlay
            && self.is_paused == other.is_paused
            && match (&self.current_frame, &other.current_frame) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

impl ElementExt for VideoFrameElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(other) = (other.as_ref() as &dyn Any).downcast_ref::<VideoFrameElement>() else {
            return false;
        };
        self != other
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(other) = (other.as_ref() as &dyn Any).downcast_ref::<VideoFrameElement>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.layout != other.layout {
            diff.insert(DiffModifies::LAYOUT);
        }

        let frame_changed = match (&self.current_frame, &other.current_frame) {
            (Some(a), Some(b)) => !Rc::ptr_eq(a, b),
            (None, None) => false,
            _ => true,
        };
        if frame_changed
            || self.show_overlay != other.show_overlay
            || self.is_paused != other.is_paused
        {
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
        Cow::Owned(AccessibilityData::default())
    }

    fn should_measure_inner_children(&self) -> bool {
        false
    }

    fn should_hook_measurement(&self) -> bool {
        false
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
        let area = context.layout_node.visible_area();
        let rect = SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());

        // Draw video frame
        if let Some(frame) = &self.current_frame {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            context.canvas.draw_image_rect_with_sampling_options(
                frame.as_ref(),
                None,
                rect,
                SamplingOptions::new(FilterMode::Linear, MipmapMode::None),
                &paint,
            );
        }

        // Dark tint + transport icons on hover
        if self.show_overlay {
            let mut tint = Paint::default();
            tint.set_style(PaintStyle::Fill);
            tint.set_color(Color::from_argb(140, 0, 0, 0));
            tint.set_anti_alias(true);
            context.canvas.draw_rect(rect, &tint);

            let cx = (area.min_x() + area.max_x()) / 2.0;
            let cy = (area.min_y() + area.max_y()) / 2.0;
            let icon_size = 36.0_f32;

            let mut icon_paint = Paint::default();
            icon_paint.set_color(Color::WHITE);
            icon_paint.set_anti_alias(true);
            icon_paint.set_style(PaintStyle::Fill);

            if self.is_paused {
                let mut builder = PathBuilder::new();
                builder.move_to((cx - icon_size * 0.4, cy - icon_size * 0.6));
                builder.line_to((cx + icon_size * 0.7, cy));
                builder.line_to((cx - icon_size * 0.4, cy + icon_size * 0.6));
                builder.close();
                context.canvas.draw_path(&builder.snapshot(), &icon_paint);
            } else {
                let bar_w = icon_size * 0.28;
                let bar_h = icon_size * 1.1;
                let gap = icon_size * 0.22;
                context.canvas.draw_rect(
                    SkRect::from_xywh(cx - gap - bar_w, cy - bar_h / 2.0, bar_w, bar_h),
                    &icon_paint,
                );
                context.canvas.draw_rect(
                    SkRect::from_xywh(cx + gap, cy - bar_h / 2.0, bar_w, bar_h),
                    &icon_paint,
                );
            }
        }
    }
}

/// Builder for the internal video frame element.
pub(crate) struct VideoFrame {
    pub element: VideoFrameElement,
    pub key: DiffKey,
}

pub(crate) fn video_frame(
    current_frame: Option<Rc<SkImage>>,
    show_overlay: bool,
    is_paused: bool,
) -> VideoFrame {
    VideoFrame {
        key: DiffKey::None,
        element: VideoFrameElement {
            current_frame,
            show_overlay,
            is_paused,
            layout: LayoutData::default(),
        },
    }
}

impl KeyExt for VideoFrame {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for VideoFrame {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerSizeExt for VideoFrame {}

impl MaybeExt for VideoFrame {}

impl From<VideoFrame> for Element {
    fn from(value: VideoFrame) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: vec![],
        }
    }
}
