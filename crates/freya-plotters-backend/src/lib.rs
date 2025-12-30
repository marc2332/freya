use std::error::Error;

use freya_engine::prelude::*;
use plotters_backend::{
    BackendCoord,
    BackendStyle,
    BackendTextStyle,
    DrawingBackend,
    DrawingErrorKind,
    rasterizer,
    text_anchor::{
        HPos,
        VPos,
    },
};

#[derive(Debug)]
pub struct PlotSkiaBackendError;

impl std::fmt::Display for PlotSkiaBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Skia backend error.")
    }
}

impl Error for PlotSkiaBackendError {}

pub struct PlotSkiaBackend<'a> {
    size: (i32, i32),
    canvas: &'a Canvas,
    font_collection: &'a mut FontCollection,
}

impl<'a> PlotSkiaBackend<'a> {
    pub fn new(
        canvas: &'a Canvas,
        font_collection: &'a mut FontCollection,
        size: (i32, i32),
    ) -> Self {
        Self {
            canvas,
            font_collection,
            size,
        }
    }
}

impl DrawingBackend for PlotSkiaBackend<'_> {
    type ErrorType = PlotSkiaBackendError;

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut paint = Paint::default();
        let color = style.color();
        paint.set_color(Color::from_argb(
            (255. * color.alpha) as u8,
            color.rgb.0,
            color.rgb.1,
            color.rgb.2,
        ));
        paint.set_stroke_width(style.stroke_width() as f32);
        self.canvas.draw_line(from, to, &paint);
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut paint = Paint::default();
        let color = style.color();
        paint.set_color(Color::from_argb(
            (255. * color.alpha) as u8,
            color.rgb.0,
            color.rgb.1,
            color.rgb.2,
        ));
        paint.set_style(if fill {
            PaintStyle::Fill
        } else {
            PaintStyle::Stroke
        });
        paint.set_stroke_width(style.stroke_width() as f32);
        let rect = Rect::new(
            upper_left.0 as f32,
            upper_left.1 as f32,
            bottom_right.0 as f32,
            bottom_right.1 as f32,
        );
        self.canvas.draw_rect(rect, &paint);
        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        // Code based on the SVG backend implementation
        if style.stroke_width() == 1 {
            let mut begin: Option<BackendCoord> = None;
            for end in path.into_iter() {
                if let Some(begin) = begin {
                    let result = self.draw_line(begin, end, style);
                    #[allow(clippy::question_mark)]
                    if result.is_err() {
                        return result;
                    }
                }
                begin = Some(end);
            }
        } else {
            let p: Vec<_> = path.into_iter().collect();
            let v = rasterizer::polygonize(&p[..], style.stroke_width());
            return self.fill_polygon(v, &style.color());
        }
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let radius = radius as f32;

        let mut paint = Paint::default();
        let color = style.color();
        paint.set_anti_alias(true);
        paint.set_style(if fill {
            PaintStyle::Fill
        } else {
            PaintStyle::Stroke
        });
        paint.set_color(Color::from_argb(
            (255.0 * color.alpha) as u8,
            color.rgb.0,
            color.rgb.1,
            color.rgb.2,
        ));

        if !fill {
            paint.set_stroke_width(1.0);
        }

        self.canvas.draw_circle(center, radius, &paint);

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let vert_buf: Vec<_> = vert.into_iter().collect();

        let mut paint = Paint::default();
        let color = style.color();
        paint.set_color(Color::from_argb(
            (255. * color.alpha) as u8,
            color.rgb.0,
            color.rgb.1,
            color.rgb.2,
        ));
        let mut path = PathBuilder::new();
        let first = vert_buf[0];
        path.move_to(first);

        for pos in &vert_buf[1..] {
            path.line_to(*pos);
        }
        let path = path.detach();
        self.canvas.draw_path(&path, &paint);

        Ok(())
    }

    fn draw_text<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut builder =
            ParagraphBuilder::new(&ParagraphStyle::default(), self.font_collection.clone());
        let mut text_style = TextStyle::new();
        let color = style.color();
        text_style.set_color(Color::from_argb(
            (255. * color.alpha) as u8,
            color.rgb.0,
            color.rgb.1,
            color.rgb.2,
        ));
        text_style.set_font_families(&[style.family().as_str()]);
        text_style.set_font_size(style.size() as f32);
        builder.push_style(&text_style);
        builder.add_text(text);
        let mut paragraph = builder.build();
        paragraph.layout(f32::MAX);

        let mut pos = (pos.0 as f32, pos.1 as f32);
        match style.anchor().h_pos {
            HPos::Left => {}
            HPos::Center => {
                pos.0 -= paragraph.max_intrinsic_width() / 2.0;
            }
            HPos::Right => {
                pos.0 -= paragraph.max_intrinsic_width();
            }
        }
        match style.anchor().v_pos {
            VPos::Top => {}
            VPos::Center => {
                pos.1 -= paragraph.height() / 2.0;
            }
            VPos::Bottom => {
                pos.1 -= paragraph.height();
            }
        }

        paragraph.paint(self.canvas, pos);
        Ok(())
    }

    fn estimate_text_size<TStyle: BackendTextStyle>(
        &self,
        text: &str,
        style: &TStyle,
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        let mut builder =
            ParagraphBuilder::new(&ParagraphStyle::default(), self.font_collection.clone());
        let mut text_style = TextStyle::new();
        let color = style.color();
        text_style.set_color(Color::from_argb(
            (255. * color.alpha) as u8,
            color.rgb.0,
            color.rgb.1,
            color.rgb.2,
        ));
        text_style.set_font_families(&[style.family().as_str()]);
        text_style.set_font_size(style.size() as f32);
        builder.push_style(&text_style);
        builder.add_text(text);
        let mut paragraph = builder.build();
        paragraph.layout(f32::MAX);
        Ok((
            paragraph.max_intrinsic_width() as u32,
            paragraph.height() as u32,
        ))
    }

    fn draw_pixel(
        &mut self,
        _point: plotters_backend::BackendCoord,
        _color: plotters_backend::BackendColor,
    ) -> Result<(), plotters_backend::DrawingErrorKind<Self::ErrorType>> {
        todo!()
    }

    fn get_size(&self) -> (u32, u32) {
        (self.size.0 as u32, self.size.1 as u32)
    }

    fn ensure_prepared(
        &mut self,
    ) -> Result<(), plotters_backend::DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), plotters_backend::DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }
}
