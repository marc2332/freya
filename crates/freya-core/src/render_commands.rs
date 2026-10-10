use std::rc::Rc;

use freya_engine::prelude::{
    Canvas,
    ClipOp,
    FontCollection,
    ImageFilter,
    Paint,
    PaintStyle,
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
};
use torin::prelude::{
    Area,
    Point2D,
    Size2D,
};

use crate::{
    data::TextStyleState,
    elements::paragraph::ParagraphPaintExt,
    style::{
        render_callback::{
            RenderCallback,
            RenderContext as FillRenderContext,
        },
        text_align::TextAlign,
        text_shadow::TextShadow,
    },
};

/// Context given to [RenderCommand::Custom] draws when they are replayed.
pub struct CustomRenderContext<'a> {
    pub canvas: &'a Canvas,
    pub font_collection: &'a mut FontCollection,
}

/// A single recorded canvas operation.
pub enum RenderCommand {
    Rect {
        rect: SkRect,
        paint: Paint,
    },
    Path {
        path: SkPath,
        paint: Paint,
    },
    RRect {
        rrect: SkRRect,
        paint: Paint,
    },
    DRRect {
        outer: SkRRect,
        inner: SkRRect,
        paint: Paint,
    },
    Paragraph {
        paragraph: Rc<SkParagraph>,
        origin: Point2D,
    },
    Image {
        image: SkImage,
        rect: SkRect,
        sampling: SamplingOptions,
        paint: Paint,
    },
    ClipRect {
        rect: SkRect,
        op: ClipOp,
        anti_alias: bool,
    },
    ClipRRect {
        rrect: SkRRect,
        op: ClipOp,
        anti_alias: bool,
    },
    ClipPath {
        path: SkPath,
        op: ClipOp,
        anti_alias: bool,
    },
    Translate {
        x: f32,
        y: f32,
    },
    Scale {
        x: f32,
        y: f32,
    },
    Concat {
        matrix: SkMatrix,
    },
    Save,
    Restore,
    SaveLayer {
        bounds: SkRect,
        alpha: Option<f32>,
    },
    BackdropBlur {
        bounds: SkRect,
        filter: ImageFilter,
    },
    Custom {
        draw: Rc<dyn Fn(&mut CustomRenderContext)>,
    },
}

/// The recorded commands of a node plus the device-space bounds they paint.
pub struct NodeRecording {
    pub commands: Vec<RenderCommand>,
    /// Painted bounds rounded out to the pixel grid, `None` when they could not
    /// be bounded reliably and must be treated as covering everything.
    pub bounds: Option<Area>,
    /// Whether any command samples the pixels underneath it, like backdrop blurs.
    pub samples_backdrop: bool,
}

impl NodeRecording {
    /// Replay the recorded commands onto a canvas, isolated by an outer save.
    pub fn replay(&self, canvas: &Canvas, font_collection: &mut FontCollection) {
        let checkpoint = canvas.save();
        for command in &self.commands {
            match command {
                RenderCommand::Rect { rect, paint } => {
                    canvas.draw_rect(rect, paint);
                }
                RenderCommand::Path { path, paint } => {
                    canvas.draw_path(path, paint);
                }
                RenderCommand::RRect { rrect, paint } => {
                    canvas.draw_rrect(rrect, paint);
                }
                RenderCommand::DRRect {
                    outer,
                    inner,
                    paint,
                } => {
                    canvas.draw_drrect(outer, inner, paint);
                }
                RenderCommand::Paragraph { paragraph, origin } => {
                    let checkpoint = canvas.save();
                    canvas.translate(origin.to_tuple());
                    paragraph.paint(canvas, (0., 0.));
                    canvas.restore_to_count(checkpoint);
                }
                RenderCommand::Image {
                    image,
                    rect,
                    sampling,
                    paint,
                } => {
                    canvas.draw_image_rect_with_sampling_options(
                        image, None, *rect, *sampling, paint,
                    );
                }
                RenderCommand::ClipRect {
                    rect,
                    op,
                    anti_alias,
                } => {
                    canvas.clip_rect(*rect, *op, *anti_alias);
                }
                RenderCommand::ClipRRect {
                    rrect,
                    op,
                    anti_alias,
                } => {
                    canvas.clip_rrect(*rrect, *op, *anti_alias);
                }
                RenderCommand::ClipPath {
                    path,
                    op,
                    anti_alias,
                } => {
                    canvas.clip_path(path, *op, *anti_alias);
                }
                RenderCommand::Translate { x, y } => {
                    canvas.translate((*x, *y));
                }
                RenderCommand::Scale { x, y } => {
                    canvas.scale((*x, *y));
                }
                RenderCommand::Concat { matrix } => {
                    canvas.concat(matrix);
                }
                RenderCommand::Save => {
                    canvas.save();
                }
                RenderCommand::Restore => {
                    canvas.restore();
                }
                RenderCommand::SaveLayer { bounds, alpha } => match alpha {
                    Some(alpha) => {
                        canvas.save_layer_alpha_f(*bounds, *alpha);
                    }
                    None => {
                        canvas.save_layer(&SaveLayerRec::default().bounds(bounds));
                    }
                },
                RenderCommand::BackdropBlur { bounds, filter } => {
                    canvas.save_layer(&SaveLayerRec::default().bounds(bounds).backdrop(filter));
                }
                RenderCommand::Custom { draw } => {
                    let mut context = CustomRenderContext {
                        canvas,
                        font_collection,
                    };
                    draw(&mut context);
                }
            }
        }
        canvas.restore_to_count(checkpoint);
    }
}

#[derive(Clone)]
struct RecordState {
    matrix: SkMatrix,
    clip: Option<SkRect>,
}

impl Default for RecordState {
    fn default() -> Self {
        Self {
            matrix: SkMatrix::new_identity(),
            clip: None,
        }
    }
}

/// Records canvas-like operations and accumulates conservative device bounds,
/// marking the recording unbounded when an operation cannot be bounded.
pub struct RenderRecorder {
    commands: Vec<RenderCommand>,
    states: Vec<RecordState>,
    bounds: Option<SkRect>,
    unbounded: bool,
    samples_backdrop: bool,
}

impl Default for RenderRecorder {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            states: vec![RecordState::default()],
            bounds: None,
            unbounded: false,
            samples_backdrop: false,
        }
    }
}

impl RenderRecorder {
    fn state(&self) -> &RecordState {
        self.states.last().unwrap()
    }

    fn state_mut(&mut self) -> &mut RecordState {
        self.states.last_mut().unwrap()
    }

    /// Union painted device bounds, conservatively ignoring the current clip.
    fn push_device_bounds(&mut self, device_bounds: SkRect) {
        match &mut self.bounds {
            Some(bounds) => bounds.join(device_bounds),
            None => self.bounds = Some(device_bounds),
        }
    }

    /// Union the painted bounds of a draw whose extent is unknown, falling back
    /// to the current clip or marking the whole recording as unbounded.
    fn push_unbounded(&mut self) {
        match self.state().clip {
            Some(clip) => self.push_device_bounds(clip),
            None => {
                self.unbounded = true;
            }
        }
    }

    /// Union the painted bounds of a draw, expanded by what the paint may add.
    fn push_draw_bounds(&mut self, mut local_bounds: SkRect, paint: &Paint, mask_sigma: f32) {
        match paint.style() {
            PaintStyle::Stroke | PaintStyle::StrokeAndFill => {
                let stroke_width = paint.stroke_width().max(1.0);
                local_bounds = local_bounds.with_outset((stroke_width, stroke_width));
            }
            PaintStyle::Fill => {}
        }

        if mask_sigma > 0.0 {
            let outset = mask_sigma * 3.0;
            local_bounds = local_bounds.with_outset((outset, outset));
        } else if paint.mask_filter().is_some() {
            self.push_unbounded();
            return;
        }

        if let Some(image_filter) = paint.image_filter() {
            if image_filter.can_compute_fast_bounds() {
                local_bounds = image_filter.compute_fast_bounds(local_bounds);
            } else {
                self.push_unbounded();
                return;
            }
        }

        let mut device_bounds = self.state().matrix.map_rect(local_bounds).0;
        if paint.is_anti_alias() {
            device_bounds = device_bounds.with_outset((1.0, 1.0));
        }
        self.push_device_bounds(device_bounds);
    }

    pub fn save(&mut self) {
        let state = self.state().clone();
        self.states.push(state);
        self.commands.push(RenderCommand::Save);
    }

    pub fn restore(&mut self) {
        if self.states.len() > 1 {
            self.states.pop();
        }
        self.commands.push(RenderCommand::Restore);
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        let matrix = SkMatrix::translate((x, y));
        let state = self.state_mut();
        state.matrix = SkMatrix::concat(&state.matrix, &matrix);
        self.commands.push(RenderCommand::Translate { x, y });
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        let matrix = SkMatrix::scale((x, y));
        let state = self.state_mut();
        state.matrix = SkMatrix::concat(&state.matrix, &matrix);
        self.commands.push(RenderCommand::Scale { x, y });
    }

    pub fn concat(&mut self, matrix: SkMatrix) {
        let state = self.state_mut();
        state.matrix = SkMatrix::concat(&state.matrix, &matrix);
        self.commands.push(RenderCommand::Concat { matrix });
    }

    fn apply_clip(&mut self, local_bounds: SkRect, op: ClipOp) {
        if op == ClipOp::Intersect {
            let state = self.state();
            let device_bounds = state.matrix.map_rect(local_bounds).0;
            let clip = match state.clip {
                Some(mut clip) => {
                    if !clip.intersect(device_bounds) {
                        clip = SkRect::new_empty();
                    }
                    clip
                }
                None => device_bounds,
            };
            self.state_mut().clip = Some(clip);
        }
    }

    pub fn clip_rect(&mut self, rect: SkRect, op: ClipOp, anti_alias: bool) {
        self.apply_clip(rect, op);
        self.commands.push(RenderCommand::ClipRect {
            rect,
            op,
            anti_alias,
        });
    }

    pub fn clip_rrect(&mut self, rrect: SkRRect, op: ClipOp, anti_alias: bool) {
        self.apply_clip(*rrect.rect(), op);
        self.commands.push(RenderCommand::ClipRRect {
            rrect,
            op,
            anti_alias,
        });
    }

    pub fn clip_path(&mut self, path: SkPath, op: ClipOp, anti_alias: bool) {
        self.apply_clip(path.bounds().to_owned(), op);
        self.commands.push(RenderCommand::ClipPath {
            path,
            op,
            anti_alias,
        });
    }

    pub fn draw_rect(&mut self, rect: SkRect, paint: Paint) {
        self.push_draw_bounds(rect, &paint, 0.0);
        self.commands.push(RenderCommand::Rect { rect, paint });
    }

    pub fn draw_path(&mut self, path: SkPath, paint: Paint) {
        self.push_draw_bounds(path.bounds().to_owned(), &paint, 0.0);
        self.commands.push(RenderCommand::Path { path, paint });
    }

    /// Draw a path blurred by a gaussian mask filter of the given sigma.
    /// The filter is created here so the painted bounds account for it.
    pub fn draw_path_blurred(&mut self, path: SkPath, paint: &Paint, blur_sigma: f32) {
        let mut paint = paint.clone();
        paint.set_mask_filter(SkMaskFilter::blur(SkBlurStyle::Normal, blur_sigma, false));
        self.push_draw_bounds(path.bounds().to_owned(), &paint, blur_sigma);
        self.commands.push(RenderCommand::Path { path, paint });
    }

    pub fn draw_rrect(&mut self, rrect: SkRRect, paint: Paint) {
        self.push_draw_bounds(*rrect.rect(), &paint, 0.0);
        self.commands.push(RenderCommand::RRect { rrect, paint });
    }

    pub fn draw_drrect(&mut self, outer: SkRRect, inner: SkRRect, paint: Paint) {
        self.push_draw_bounds(*outer.rect(), &paint, 0.0);
        self.commands.push(RenderCommand::DRRect {
            outer,
            inner,
            paint,
        });
    }

    /// Painted width of a paragraph. Aligned glyphs can sit anywhere inside
    /// the layout width, default-aligned ones never pass the longest line.
    fn paragraph_width(paragraph: &SkParagraph, text_align: TextAlign) -> f32 {
        if text_align == TextAlign::default() {
            paragraph.longest_line()
        } else {
            paragraph.max_width().max(paragraph.longest_line())
        }
    }

    /// Draw a laid-out paragraph, its text shadows accounted into the bounds.
    pub fn draw_paragraph(
        &mut self,
        paragraph: impl Into<Rc<SkParagraph>>,
        origin: Point2D,
        text_shadows: &[TextShadow],
        text_align: TextAlign,
    ) {
        let paragraph = paragraph.into();
        let width = Self::paragraph_width(&paragraph, text_align);
        // Glyphs may slightly overflow the layout metrics
        let mut local_bounds = SkRect::from_xywh(origin.x, origin.y, width, paragraph.height())
            .with_outset((2.0, 2.0));

        for text_shadow in text_shadows {
            let outset_x = text_shadow.offset.0.abs() + text_shadow.blur_sigma as f32 * 3.0;
            let outset_y = text_shadow.offset.1.abs() + text_shadow.blur_sigma as f32 * 3.0;
            local_bounds = local_bounds.with_outset((outset_x, outset_y));
        }

        self.push_draw_bounds(local_bounds, &Paint::default(), 0.0);
        self.commands
            .push(RenderCommand::Paragraph { paragraph, origin });
    }

    /// Draw a styled paragraph with its text shadows included in the bounds.
    pub fn draw_styled_paragraph(
        &mut self,
        paragraph: &Rc<SkParagraph>,
        origin: Point2D,
        text_style: &TextStyleState,
        scale_factor: f64,
    ) {
        self.draw_paragraph(
            paragraph.clone(),
            origin,
            &text_style.text_shadows,
            text_style.text_align,
        );
        if let Some(callback) = text_style.color.render_callback() {
            self.draw_fill(
                callback.clone(),
                origin,
                paragraph.fill_area().size,
                text_style,
                scale_factor,
            );
        }
    }

    pub fn draw_fill(
        &mut self,
        callback: RenderCallback,
        origin: Point2D,
        size: Size2D,
        text_style: &TextStyleState,
        scale_factor: f64,
    ) {
        let scale_factor = scale_factor as f32;
        let text_style = text_style.clone();
        self.save();
        self.translate(origin.x, origin.y);
        self.scale(scale_factor, scale_factor);
        self.push_unbounded();
        self.custom(
            SkRect::from_xywh(
                0.,
                0.,
                size.width / scale_factor,
                size.height / scale_factor,
            ),
            move |custom| {
                callback.call(&mut FillRenderContext {
                    canvas: custom.canvas,
                    font_collection: custom.font_collection,
                    origin: origin / scale_factor,
                    size: size / scale_factor,
                    text_style_state: &text_style,
                });
            },
        );
        self.restore();
    }

    pub fn draw_image_rect(
        &mut self,
        image: SkImage,
        rect: SkRect,
        sampling: SamplingOptions,
        paint: Paint,
    ) {
        self.push_draw_bounds(rect, &paint, 0.0);
        self.commands.push(RenderCommand::Image {
            image,
            rect,
            sampling,
            paint,
        });
    }

    pub fn save_layer(&mut self, bounds: SkRect, alpha: Option<f32>) {
        let state = self.state().clone();
        self.states.push(state);
        self.commands
            .push(RenderCommand::SaveLayer { bounds, alpha });
    }

    /// Record a layer that samples and blurs the pixels underneath `bounds`.
    pub fn backdrop_blur(&mut self, bounds: SkRect, filter: ImageFilter, blur_sigma: (f32, f32)) {
        self.samples_backdrop = true;
        let outset = (blur_sigma.0 * 3.0, blur_sigma.1 * 3.0);
        let local_bounds = bounds.with_outset(outset);
        let device_bounds = self.state().matrix.map_rect(local_bounds).0;
        self.push_device_bounds(device_bounds);

        let state = self.state().clone();
        self.states.push(state);
        self.commands
            .push(RenderCommand::BackdropBlur { bounds, filter });
    }

    /// Record an arbitrary draw painting at most `bounds`.
    pub fn custom(&mut self, bounds: SkRect, draw: impl Fn(&mut CustomRenderContext) + 'static) {
        let device_bounds = self.state().matrix.map_rect(bounds).0;
        self.push_device_bounds(device_bounds);
        self.commands.push(RenderCommand::Custom {
            draw: Rc::new(draw),
        });
    }

    /// Finish the recording, rounding the painted bounds out to the pixel grid.
    pub fn finish(self) -> NodeRecording {
        let bounds = if self.unbounded {
            None
        } else {
            Some(self.bounds.map_or_else(Area::zero, |bounds| {
                Area::new(
                    Point2D::new(bounds.left, bounds.top),
                    Size2D::new(bounds.width(), bounds.height()),
                )
                .round_out()
            }))
        };

        NodeRecording {
            commands: self.commands,
            bounds,
            samples_backdrop: self.samples_backdrop,
        }
    }
}
