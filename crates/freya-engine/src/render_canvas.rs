use std::{
    borrow::Borrow,
    rc::Rc,
};

use euclid::Point2D;

use crate::prelude::{
    Canvas,
    ClipOp,
    CustomRenderContext,
    FontCollection,
    ImageFilter,
    Paint,
    SamplingOptions,
    SaveLayerRec,
    SkBlurStyle,
    SkImage,
    SkMaskFilter,
    SkMatrix,
    SkParagraph,
    SkPath,
    SkRRect,
    SkRect,
    SkTextAlign,
    SkTextShadow,
};

#[cfg(feature = "incremental-painting")]
use crate::prelude::RenderRecorder;

/// Draw directly onto a canvas or record operations for later replay.
pub enum RenderCanvas<'a> {
    Direct(&'a Canvas),
    #[cfg(feature = "incremental-painting")]
    Recording(RenderRecorder),
}

impl RenderCanvas<'_> {
    pub fn save(&mut self) {
        match self {
            Self::Direct(canvas) => {
                canvas.save();
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.save(),
        }
    }

    pub fn restore(&mut self) {
        match self {
            Self::Direct(canvas) => {
                canvas.restore();
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.restore(),
        }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        match self {
            Self::Direct(canvas) => {
                canvas.translate((x, y));
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.translate(x, y),
        }
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        match self {
            Self::Direct(canvas) => {
                canvas.scale((x, y));
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.scale(x, y),
        }
    }

    pub fn concat(&mut self, matrix: SkMatrix) {
        match self {
            Self::Direct(canvas) => {
                canvas.concat(&matrix);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.concat(matrix),
        }
    }

    pub fn clip_rect(&mut self, rect: SkRect, op: ClipOp, anti_alias: bool) {
        match self {
            Self::Direct(canvas) => {
                canvas.clip_rect(rect, op, anti_alias);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.clip_rect(rect, op, anti_alias),
        }
    }

    pub fn clip_rrect(&mut self, rrect: SkRRect, op: ClipOp, anti_alias: bool) {
        match self {
            Self::Direct(canvas) => {
                canvas.clip_rrect(rrect, op, anti_alias);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.clip_rrect(rrect, op, anti_alias),
        }
    }

    pub fn clip_path(&mut self, path: SkPath, op: ClipOp, anti_alias: bool) {
        match self {
            Self::Direct(canvas) => {
                canvas.clip_path(&path, op, anti_alias);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.clip_path(path, op, anti_alias),
        }
    }

    pub fn draw_rect(&mut self, rect: SkRect, paint: Paint) {
        match self {
            Self::Direct(canvas) => {
                canvas.draw_rect(rect, &paint);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.draw_rect(rect, paint),
        }
    }

    pub fn draw_path(&mut self, path: SkPath, paint: Paint) {
        match self {
            Self::Direct(canvas) => {
                canvas.draw_path(&path, &paint);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.draw_path(path, paint),
        }
    }

    pub fn draw_path_blurred(&mut self, path: SkPath, paint: &Paint, blur_sigma: f32) {
        match self {
            Self::Direct(canvas) => {
                let mut paint = paint.clone();
                paint.set_mask_filter(SkMaskFilter::blur(SkBlurStyle::Normal, blur_sigma, false));
                canvas.draw_path(&path, &paint);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.draw_path_blurred(path, paint, blur_sigma),
        }
    }

    pub fn draw_rrect(&mut self, rrect: SkRRect, paint: Paint) {
        match self {
            Self::Direct(canvas) => {
                canvas.draw_rrect(rrect, &paint);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.draw_rrect(rrect, paint),
        }
    }

    pub fn draw_drrect(&mut self, outer: SkRRect, inner: SkRRect, paint: Paint) {
        match self {
            Self::Direct(canvas) => {
                canvas.draw_drrect(outer, inner, &paint);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.draw_drrect(outer, inner, paint),
        }
    }

    pub fn draw_paragraph(
        &mut self,
        paragraph: impl Borrow<SkParagraph> + Into<Rc<SkParagraph>>,
        origin: Point2D<f32, ()>,
        _text_shadows: impl IntoIterator<Item = SkTextShadow>,
        _text_align: SkTextAlign,
    ) {
        match self {
            Self::Direct(canvas) => {
                let checkpoint = canvas.save();
                canvas.translate(origin.to_tuple());
                paragraph.borrow().paint(canvas, (0., 0.));
                canvas.restore_to_count(checkpoint);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => {
                recorder.draw_paragraph(paragraph, origin, _text_shadows, _text_align);
            }
        }
    }

    pub fn draw_image_rect(
        &mut self,
        image: SkImage,
        rect: SkRect,
        sampling: SamplingOptions,
        paint: Paint,
    ) {
        match self {
            Self::Direct(canvas) => {
                canvas.draw_image_rect_with_sampling_options(&image, None, rect, sampling, &paint);
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.draw_image_rect(image, rect, sampling, paint),
        }
    }

    pub fn save_layer(&mut self, bounds: SkRect, alpha: Option<f32>) {
        match self {
            Self::Direct(canvas) => match alpha {
                Some(alpha) => {
                    canvas.save_layer_alpha_f(bounds, alpha);
                }
                None => {
                    canvas.save_layer(&SaveLayerRec::default().bounds(&bounds));
                }
            },
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.save_layer(bounds, alpha),
        }
    }

    pub fn backdrop_blur(&mut self, bounds: SkRect, filter: ImageFilter, _blur_sigma: (f32, f32)) {
        match self {
            Self::Direct(canvas) => {
                canvas.save_layer(&SaveLayerRec::default().bounds(&bounds).backdrop(&filter));
            }
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.backdrop_blur(bounds, filter, _blur_sigma),
        }
    }

    pub fn custom(
        &mut self,
        _bounds: SkRect,
        font_collection: &mut FontCollection,
        draw: impl Fn(&mut CustomRenderContext) + 'static,
    ) {
        match self {
            Self::Direct(canvas) => draw(&mut CustomRenderContext {
                canvas,
                font_collection,
            }),
            #[cfg(feature = "incremental-painting")]
            Self::Recording(recorder) => recorder.custom(_bounds, draw),
        }
    }
}
