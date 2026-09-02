use std::sync::Arc;

use anyrender::{
    PaintScene,
    RenderContext,
    filters::{
        Filter,
        FilterEffect,
        FilterId,
        FilterInput,
        component_transfer::TransferFunction,
    },
};
use freya_engine::prelude::{
    BlurStyle,
    Canvas,
    Color,
    ColorSpace,
    Data,
    Font,
    FontArguments,
    FontEdging as Edging,
    FontHinting,
    FontMgr,
    GlyphId,
    GlyphPositions,
    ImageFilter,
    MaskFilter,
    Paint,
    PaintCap,
    PaintJoin,
    PaintStyle,
    PathEffect,
    Point,
    RRect,
    Rect,
    SaveLayerRec,
    Shader,
    Typeface,
    VariationCoordinate as Coordinate,
    VariationPosition,
};
use peniko::color::AlphaColor;

use crate::anyrender::{
    cache::{
        FontCacheKey,
        FontCacheKeyBorrowed,
        GenerationalCache,
        NormalizedTypefaceCacheKey,
        NormalizedTypefaceCacheKeyBorrowed,
    },
    scene::sk_peniko::color4f_from_alpha_color,
};

pub struct SkiaSceneCache {
    paint: Paint,
    dash_intervals: Vec<f32>,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    extracted_font_data: GenerationalCache<(u64, u32), peniko::FontData>,
    typeface: GenerationalCache<(u64, u32), Typeface>,
    normalized_typeface: GenerationalCache<NormalizedTypefaceCacheKey, Typeface>,
    image_shader: GenerationalCache<u64, Shader>,
    font: GenerationalCache<FontCacheKey, Font>,
    font_mgr: FontMgr,
    glyph_id_buf: Vec<GlyphId>,
    glyph_pos_buf: Vec<Point>,
}

impl SkiaSceneCache {
    /// Create a new `SkiaSceneCache`
    pub fn new() -> Self {
        Default::default()
    }

    /// Increment the generation on the caches. Should be called once per frame.
    pub fn next_gen(&mut self) {
        self.typeface.next_gen();
        self.normalized_typeface.next_gen();
        self.image_shader.next_gen();
        self.font.next_gen();
    }
}

impl Default for SkiaSceneCache {
    fn default() -> Self {
        Self {
            paint: Paint::default(),
            dash_intervals: Vec::new(),
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            extracted_font_data: GenerationalCache::new(10),
            typeface: GenerationalCache::new(1),
            normalized_typeface: GenerationalCache::new(1),
            image_shader: GenerationalCache::new(1),
            font: GenerationalCache::new(10),
            font_mgr: FontMgr::new(),
            glyph_id_buf: Default::default(),
            glyph_pos_buf: Default::default(),
        }
    }
}

pub struct SkiaScenePainter<'a> {
    pub(crate) inner: &'a Canvas,
    pub(crate) cache: &'a mut SkiaSceneCache,
}

impl SkiaScenePainter<'_> {
    pub fn new<'a>(canvas: &'a Canvas, cache: &'a mut SkiaSceneCache) -> SkiaScenePainter<'a> {
        SkiaScenePainter {
            inner: canvas,
            cache,
        }
    }

    fn reset_paint(&mut self) {
        self.cache.paint.reset();
        self.cache.paint.set_anti_alias(true);
    }

    fn set_paint_alpha(&mut self, alpha: f32) {
        self.cache.paint.set_alpha_f(alpha);
    }

    fn set_paint_filter(&mut self, filter: ImageFilter) {
        self.cache.paint.set_image_filter(filter);
    }

    fn set_paint_blend_mode(&mut self, blend_mode: impl Into<peniko::BlendMode>) {
        self.cache
            .paint
            .set_blend_mode(sk_peniko::blend_mode_from(blend_mode.into()));
    }

    fn set_matrix(&self, transform: kurbo::Affine) {
        self.inner.set_matrix(&sk_kurbo::m44_from_affine(transform));
    }

    fn concat_matrix(&self, transform: kurbo::Affine) {
        self.inner.concat(&sk_kurbo::matrix_from_affine(transform));
    }

    fn clip(&self, shape: &impl kurbo::Shape) {
        if let Some(rect) = shape.as_rect() {
            self.inner.clip_rect(sk_kurbo::rect_from(rect), None, true);
        } else if let Some(rrect) = shape.as_rounded_rect() {
            self.inner
                .clip_rrect(sk_kurbo::rrect_from(rrect), None, true);
        } else {
            self.inner
                .clip_path(&sk_kurbo::path_from_shape(shape), None, true);
        }
    }

    fn set_paint_brush<'a>(
        &mut self,
        brush: impl Into<anyrender::PaintRef<'a>>,
        brush_transform: Option<kurbo::Affine>,
    ) {
        let brush: anyrender::PaintRef<'a> = brush.into();
        match brush {
            anyrender::Paint::Solid(alpha_color) => {
                self.cache.paint.set_color4f(
                    sk_peniko::color4f_from_alpha_color(alpha_color),
                    &ColorSpace::new_srgb(),
                );
            }
            anyrender::Paint::Gradient(gradient) => {
                self.cache
                    .paint
                    .set_shader(sk_peniko::shader_from_gradient(gradient, brush_transform));
            }
            anyrender::Paint::Image(image_brush) => {
                if let Some(shader) = self.cache.image_shader.hit(&image_brush.image.data.id()) {
                    self.cache.paint.set_shader(shader.clone());
                    return;
                }

                let image_shader = sk_peniko::shader_from_image_brush(image_brush, brush_transform);

                if let Some(shader) = &image_shader {
                    self.cache
                        .image_shader
                        .insert(image_brush.image.data.id(), shader.clone());
                }

                self.cache.paint.set_shader(image_shader);
            }

            // Render unsupported custom paints as transparent
            anyrender::Paint::Resource(_) | anyrender::Paint::Custom(_) => {
                self.cache.paint.set_color4f(
                    sk_peniko::color4f_from_alpha_color(AlphaColor::TRANSPARENT),
                    &ColorSpace::new_srgb(),
                );
            }
        }
    }

    fn set_paint_style<'a>(&mut self, style: impl Into<peniko::StyleRef<'a>>) {
        match style.into() {
            peniko::StyleRef::Fill(_) => {
                self.cache.paint.set_style(PaintStyle::Fill);
            }
            peniko::StyleRef::Stroke(stroke) => {
                self.cache.paint.set_style(PaintStyle::Stroke);
                self.cache.paint.set_stroke(true);
                self.cache.paint.set_stroke_width(stroke.width as f32);
                self.cache.paint.set_stroke_miter(stroke.miter_limit as f32);
                self.cache.paint.set_stroke_join(match stroke.join {
                    kurbo::Join::Bevel => PaintJoin::Bevel,
                    kurbo::Join::Miter => PaintJoin::Miter,
                    kurbo::Join::Round => PaintJoin::Round,
                });
                self.cache.paint.set_stroke_cap(match stroke.start_cap {
                    kurbo::Cap::Butt => PaintCap::Butt,
                    kurbo::Cap::Square => PaintCap::Square,
                    kurbo::Cap::Round => PaintCap::Round,
                });
                if stroke.dash_pattern.is_empty() {
                    self.cache.paint.set_path_effect(None);
                } else {
                    self.cache.dash_intervals.clear();
                    self.cache
                        .dash_intervals
                        .extend(stroke.dash_pattern.iter().map(|dash| *dash as f32));
                    self.cache.paint.set_path_effect(PathEffect::dash(
                        self.cache.dash_intervals.as_slice(),
                        stroke.dash_offset as f32,
                    ));
                }
            }
        }
    }

    fn draw_shape(&mut self, shape: &impl kurbo::Shape) {
        self.draw_shape_with_fill(shape, None);
    }

    fn draw_shape_with_fill(
        &mut self,
        shape: &impl kurbo::Shape,
        fill: impl Into<Option<peniko::Fill>>,
    ) {
        if let Some(rect) = shape.as_rect() {
            self.inner
                .draw_rect(sk_kurbo::rect_from(rect), &self.cache.paint);
        } else if let Some(rrect) = shape.as_rounded_rect() {
            self.inner
                .draw_rrect(sk_kurbo::rrect_from(rrect), &self.cache.paint);
        } else if let Some(line) = shape.as_line() {
            self.inner.draw_line(
                (line.p0.x as f32, line.p0.y as f32),
                (line.p1.x as f32, line.p1.y as f32),
                &self.cache.paint,
            );
        } else if let Some(circle) = shape.as_circle() {
            self.inner.draw_circle(
                (circle.center.x as f32, circle.center.y as f32),
                circle.radius as f32,
                &self.cache.paint,
            );
        } else {
            let mut path = sk_kurbo::path_from_shape(shape);
            if let Some(fill) = fill.into() {
                path.set_fill_type(sk_peniko::path_fill_type_from_fill(fill));
            }
            self.inner.draw_path(&path, &self.cache.paint);
        }
    }

    fn get_or_cache_font(
        &mut self,
        font: &peniko::FontData,
        normalized_coords: &[anyrender::NormalizedCoord],
        font_size: f32,
        hint: bool,
    ) -> Option<Font> {
        let cache_key_borrowed = FontCacheKeyBorrowed {
            typeface_id: font.data.id(),
            typeface_index: font.index,
            normalized_coords,
            font_size: font_size.to_bits(),
            hint,
        };

        if let Some(cached) = self.cache.font.hit(&cache_key_borrowed) {
            return Some(cached.clone());
        }

        let typeface = self.get_or_cache_normalized_typeface(font, normalized_coords)?;

        let cache_key = FontCacheKey {
            typeface_id: font.data.id(),
            typeface_index: font.index,
            normalized_coords: normalized_coords.to_vec(),
            font_size: font_size.to_bits(),
            hint,
        };

        let mut font = Font::from_typeface(typeface, font_size);
        font.set_hinting(if hint {
            FontHinting::Normal
        } else {
            FontHinting::None
        });
        font.set_edging(Edging::SubpixelAntiAlias);

        self.cache.font.insert(cache_key, font.clone());

        Some(font)
    }

    fn get_or_cache_normalized_typeface(
        &mut self,
        font: &peniko::FontData,
        normalized_coords: &[anyrender::NormalizedCoord],
    ) -> Option<Typeface> {
        fn f2dot14_to_f32(raw_value: i16) -> f32 {
            let int = (raw_value >> 14) as f32;
            let fract = (raw_value & !(!0 << 14)) as f32 / (1 << 14) as f32;
            int + fract
        }

        if normalized_coords.is_empty() {
            return self.get_or_cache_typeface(font);
        }

        let cache_key_borrowed = NormalizedTypefaceCacheKeyBorrowed {
            typeface_id: font.data.id(),
            typeface_index: font.index,
            normalized_coords,
        };

        if let Some(cached) = self.cache.normalized_typeface.hit(&cache_key_borrowed) {
            return Some(cached.clone());
        }

        let typeface = self.get_or_cache_typeface(font)?;

        let axes = typeface.variation_design_parameters().unwrap_or_default();

        if axes.is_empty() {
            return Some(typeface);
        }

        let coordinates: Vec<Coordinate> = axes
            .iter()
            .zip(normalized_coords.iter().map(|c| f2dot14_to_f32(*c)))
            .filter(|(_, value)| *value != 0.0)
            .map(|(axis, factor)| {
                let value = if factor < 0.0 {
                    lerp_f32(axis.min, axis.def, -factor)
                } else {
                    lerp_f32(axis.def, axis.max, factor)
                };

                Coordinate {
                    axis: axis.tag,
                    value,
                }
            })
            .collect();
        let variation_position = VariationPosition {
            coordinates: &coordinates,
        };

        let normalized_typeface = typeface
            .clone_with_arguments(
                &FontArguments::new().set_variation_design_position(variation_position),
            )
            .unwrap();

        self.cache.normalized_typeface.insert(
            NormalizedTypefaceCacheKey {
                typeface_id: font.data.id(),
                typeface_index: font.index,
                normalized_coords: normalized_coords.to_vec(),
            },
            normalized_typeface.clone(),
        );

        Some(normalized_typeface)
    }

    fn get_or_cache_typeface<'a>(
        &'a mut self,
        #[allow(unused_mut)] mut font: &'a peniko::FontData,
    ) -> Option<Typeface> {
        let cache_key = (font.data.id(), font.index);

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        #[allow(clippy::map_entry, reason = "Cannot early-return with entry API")]
        {
            use std::sync::Arc;

            use peniko::Blob;

            if let Some(collection) = oaty::Collection::new(font.data.data()) {
                if !self.cache.extracted_font_data.contains_key(&cache_key) {
                    let Some(data) = collection
                        .get_font(font.index)
                        .and_then(|font| font.copy_data())
                    else {
                        tracing::warn!("Failed to extract font {} {}", cache_key.0, cache_key.1);
                        return None;
                    };

                    let blob = Blob::new(Arc::new(data));
                    let font_data = peniko::FontData::new(blob, 0);
                    self.cache.extracted_font_data.insert(cache_key, font_data);
                }
                font = self.cache.extracted_font_data.hit(&cache_key).unwrap()
            }
        }

        if let Some(cached) = self.cache.typeface.hit(&cache_key) {
            return Some(cached.clone());
        }

        let Some(typeface) = self
            .cache
            .font_mgr
            .new_from_data(Data::new_copy(font.data.data()), font.index)
        else {
            tracing::warn!("Failed to load font {} {}", cache_key.0, cache_key.1);
            return None;
        };

        self.cache.typeface.insert(cache_key, typeface.clone());

        Some(typeface)
    }
}

impl RenderContext for SkiaScenePainter<'_> {}
impl PaintScene for SkiaScenePainter<'_> {
    fn reset(&mut self) {
        self.inner.clear(Color::WHITE);
    }

    fn push_layer(
        &mut self,
        blend: impl Into<peniko::BlendMode>,
        alpha: f32,
        transform: kurbo::Affine,
        clip: &impl kurbo::Shape,
        filter: Option<Arc<Filter>>,
        backdrop_filter: Option<Arc<Filter>>,
    ) {
        let blend: peniko::BlendMode = blend.into();

        self.reset_paint();
        self.set_paint_alpha(alpha);
        self.set_paint_blend_mode(blend);

        if let Some(filter) = filter.as_ref().and_then(|f| convert_filter(f)) {
            self.set_paint_filter(filter);
        }

        self.inner.save();

        self.set_matrix(transform);
        self.clip(clip);

        let backdrop_filter = backdrop_filter.as_ref().and_then(|f| convert_filter(f));
        let mut save_layer_rec = SaveLayerRec::default().paint(&self.cache.paint);
        if let Some(backdrop_filter) = backdrop_filter.as_ref() {
            save_layer_rec = save_layer_rec.backdrop(backdrop_filter);
        }

        self.inner.save_layer(&save_layer_rec);
    }

    fn push_clip_layer(&mut self, transform: kurbo::Affine, clip: &impl kurbo::Shape) {
        self.inner.save(); // we need to do two saves because of pop_layer

        self.set_matrix(transform);
        self.clip(clip);
        self.inner.save();
    }

    fn pop_layer(&mut self) {
        self.inner.restore();
        self.inner.restore();
    }

    fn stroke<'a>(
        &mut self,
        style: &kurbo::Stroke,
        transform: kurbo::Affine,
        brush: impl Into<anyrender::PaintRef<'a>>,
        brush_transform: Option<kurbo::Affine>,
        shape: &impl kurbo::Shape,
    ) {
        self.set_matrix(transform);

        self.reset_paint();
        self.set_paint_brush(brush, brush_transform);
        self.set_paint_style(style);
        self.draw_shape(shape);
    }

    fn fill<'a>(
        &mut self,
        style: peniko::Fill,
        transform: kurbo::Affine,
        brush: impl Into<anyrender::PaintRef<'a>>,
        brush_transform: Option<kurbo::Affine>,
        shape: &impl kurbo::Shape,
    ) {
        self.set_matrix(transform);

        self.reset_paint();
        self.set_paint_brush(brush, brush_transform);
        self.set_paint_style(style);
        self.draw_shape_with_fill(shape, style);
    }

    fn draw_glyphs<'a, 's: 'a>(
        &'s mut self,
        #[allow(unused_mut)] mut font: &'a peniko::FontData,
        font_size: f32,
        hint: bool,
        normalized_coords: &'a [anyrender::NormalizedCoord],
        _embolden: kurbo::Vec2,
        style: impl Into<peniko::StyleRef<'a>>,
        brush: impl Into<anyrender::PaintRef<'a>>,
        brush_alpha: f32,
        transform: kurbo::Affine,
        glyph_transform: Option<kurbo::Affine>,
        glyphs: impl Iterator<Item = anyrender::Glyph>,
    ) {
        self.set_matrix(transform);

        if let Some(glyph_transform) = glyph_transform {
            self.concat_matrix(glyph_transform);
        }

        self.reset_paint();
        self.set_paint_brush(brush, None);
        self.set_paint_style(style);
        self.set_paint_alpha(brush_alpha);

        let Some(font) = self.get_or_cache_font(font, normalized_coords, font_size, hint) else {
            return;
        };

        let (min_size, _) = glyphs.size_hint();
        self.cache.glyph_id_buf.reserve(min_size);
        self.cache.glyph_pos_buf.reserve(min_size);

        for glyph in glyphs {
            self.cache.glyph_id_buf.push(GlyphId::from(glyph.id as u16));
            self.cache.glyph_pos_buf.push(Point::new(glyph.x, glyph.y));
        }

        self.inner.draw_glyphs_at(
            &self.cache.glyph_id_buf[..],
            GlyphPositions::Points(&self.cache.glyph_pos_buf[..]),
            Point::new(0.0, 0.0),
            &font,
            &self.cache.paint,
        );

        self.cache.glyph_id_buf.clear();
        self.cache.glyph_pos_buf.clear();
    }

    fn draw_box_shadow(
        &mut self,
        transform: kurbo::Affine,
        rect: kurbo::Rect,
        brush: peniko::Color,
        radius: f64,
        std_dev: f64,
    ) {
        self.set_matrix(transform);

        self.reset_paint();
        self.set_paint_brush(brush, None);
        self.cache.paint.set_style(PaintStyle::Fill);

        if std_dev > 0.0 {
            self.cache.paint.set_mask_filter(
                MaskFilter::blur(BlurStyle::Normal, std_dev as f32, false).unwrap(),
            );
        }

        let rrect = RRect::new_nine_patch(
            Rect::new(
                rect.x0 as f32,
                rect.y0 as f32,
                rect.x1 as f32,
                rect.y1 as f32,
            ),
            radius as f32,
            radius as f32,
            radius as f32,
            radius as f32,
        );

        self.inner.draw_rrect(rrect, &self.cache.paint);
    }
}

fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn convert_filter(filter: &Filter) -> Option<ImageFilter> {
    convert_filter_effect(filter, filter.output())
}

fn convert_filter_effect(filter: &Filter, id: FilterId) -> Option<ImageFilter> {
    let node = &filter.nodes()[id.0 as usize];
    let input = node.inputs.primary.as_ref().and_then(|input| match input {
        FilterInput::Source(_) => None,
        FilterInput::Result(input_id) => convert_filter_effect(filter, *input_id),
    });

    // Clone to avoid borrow checking issues. Cloning is cheap as filters are refcounted.
    let input_clone = input.clone();

    use freya_engine::prelude::{
        color_filters,
        image_filters,
        shaders,
    };

    let filter: Option<ImageFilter> = match &node.effect {
        FilterEffect::Flood(color) => {
            let rgba8 = color.to_rgba8();
            let sk_color = Color::from_argb(rgba8.a, rgba8.r, rgba8.g, rgba8.b);
            let shader = shaders::color(sk_color);
            image_filters::shader(shader, None)
        }
        FilterEffect::GaussianBlur(blur) => {
            image_filters::blur((blur.std_deviation, blur.std_deviation), None, input, None)
        }
        FilterEffect::DropShadow(drop_shadow) => image_filters::drop_shadow(
            (drop_shadow.dx, drop_shadow.dy),
            (drop_shadow.std_deviation, drop_shadow.std_deviation),
            color4f_from_alpha_color(drop_shadow.color),
            None,
            input,
            None,
        ),
        FilterEffect::ComponentTransfer(ct) => {
            fn map_transfer_function(func: &TransferFunction) -> Option<[u8; 256]> {
                match func {
                    TransferFunction::Identity => None,
                    TransferFunction::Table(table) => {
                        if table.is_empty() {
                            None
                        } else {
                            let table = table.as_slice();
                            let n = table.len();

                            let mut values = [0; 256];
                            for (i, out) in values.iter_mut().enumerate() {
                                let c = i as f64 / 255.0;
                                let k = (c * (n - 1) as f64) as usize;
                                let v1 = table[k] as f64;
                                let v2 = table[(k + 1).min(n - 1)] as f64;
                                let val =
                                    255.0 * (v1 + (c * (n - 1) as f64 - k as f64) * (v2 - v1));
                                *out = val.clamp(0.0, 255.0) as u8;
                            }
                            Some(values)
                        }
                    }
                    TransferFunction::Discrete(items) => {
                        if items.is_empty() {
                            None
                        } else {
                            let items = items.as_slice();
                            let n = items.len();

                            let mut values = [0; 256];
                            for (i, out) in values.iter_mut().enumerate() {
                                let k = (((i * n) as f64 / 255.0) as usize).min(n - 1);
                                let val = 255.0 * items[k] as f64;
                                *out = val.clamp(0.0, 255.0) as u8;
                            }
                            Some(values)
                        }
                    }
                    TransferFunction::Linear(tf) => {
                        let mut values = [0; 256];
                        for (i, out) in values.iter_mut().enumerate() {
                            let val: f64 =
                                (tf.slope as f64 * i as f64) + (255.0 * tf.intercept as f64);
                            *out = val.clamp(0.0, 255.0) as u8;
                        }
                        Some(values)
                    }
                    TransferFunction::Gamma(tf) => {
                        let mut values = [0; 256];
                        for (i, out) in values.iter_mut().enumerate() {
                            let val: f64 = 255.0
                                * (tf.amplitude as f64
                                    * (i as f64 / 255.0).powf(tf.exponent as f64))
                                + tf.offset as f64;
                            *out = val.clamp(0.0, 255.0) as u8;
                        }
                        Some(values)
                    }
                }
            }

            let color_filter = color_filters::table_argb(
                map_transfer_function(&ct.alpha_function).as_ref(),
                map_transfer_function(&ct.red_function).as_ref(),
                map_transfer_function(&ct.green_function).as_ref(),
                map_transfer_function(&ct.blue_function).as_ref(),
            );

            color_filter.and_then(|cf| image_filters::color_filter(cf, input, None))
        }
        FilterEffect::ColorMatrix(color_matrix) => {
            let color_filter =
                color_filters::matrix_row_major(&color_matrix.0, color_filters::Clamp::Yes);
            image_filters::color_filter(color_filter, input, None)
        }
        FilterEffect::Offset(offset) => {
            image_filters::offset((offset.x as f32, offset.y as f32), input, None)
        }

        // TODO: implement remaining filter effect types
        FilterEffect::Composite(_composite_operator) => None,
        FilterEffect::Blend(_mix) => None,
        FilterEffect::Morphology(_morphology_filter) => None,
        FilterEffect::ConvolveMatrix(_convolution_kernel) => None,
        FilterEffect::Turbulence(_turbulence_filter) => None,
        FilterEffect::DisplacementMap(_displacement_map_filter) => None,
        FilterEffect::Image(_external_image_source) => None,
        FilterEffect::Tile => None,
        FilterEffect::DiffuseLighting(_diffuse_lighting_filter) => None,
        FilterEffect::SpecularLighting(_specular_lighting_filter) => None,
    };

    // If we do not implement given type of filter then the match statement above returns None
    // In that case, if we have an input then we pass that along instead. In the case of filter chains
    // with some unimplemented filters that allows the remaining filters to work
    filter.or(input_clone)
}

mod sk_peniko {
    use freya_engine::prelude::{
        AlphaType as SkAlphaType,
        BlendMode as SkBlendMode,
        Color4f as SkColor4f,
        ColorType as SkColorType,
        Colors,
        Data as SkData,
        FilterMode,
        ImageInfo as SkImageInfo,
        MipmapMode,
        PathFillType as SkPathFillType,
        SamplingOptions as SkSamplingOptions,
        Shader as SkShader,
        TileMode as SkTileMode,
        gradient::{
            self,
            interpolation::{
                ColorSpace as SkGradientShaderColorSpace,
                HueMethod as SkGradientShaderHueMethod,
            },
        },
        images,
        shaders::{
            self,
            linear_gradient,
            radial_gradient,
            sweep_gradient,
            two_point_conical_gradient,
        },
    };
    use peniko::{
        BlendMode,
        Compose,
        Extend,
        Fill,
        Gradient,
        GradientKind,
        ImageAlphaType,
        ImageBrush,
        ImageData,
        ImageFormat,
        Mix,
        color::{
            AlphaColor,
            ColorSpaceTag,
            DynamicColor,
            HueDirection,
            Srgb,
        },
    };

    pub(super) fn shader_from_image_brush(
        image_brush: ImageBrush<&ImageData>,
        brush_transform: Option<kurbo::Affine>,
    ) -> Option<SkShader> {
        let image_data = image_brush.image;

        let image_info = SkImageInfo::new(
            (image_data.width as i32, image_data.height as i32),
            match image_data.format {
                ImageFormat::Rgba8 => SkColorType::RGBA8888,
                ImageFormat::Bgra8 => SkColorType::BGRA8888,
                _ => unreachable!(),
            },
            match image_data.alpha_type {
                ImageAlphaType::Alpha => SkAlphaType::Unpremul,
                ImageAlphaType::AlphaPremultiplied => SkAlphaType::Premul,
            },
            None,
        );
        let pixels = unsafe {
            SkData::new_bytes(image_data.data.data()) // We have to ensure the src image data lives long enough
        };
        let image =
            images::raster_from_data(&image_info, pixels, image_info.min_row_bytes()).unwrap();

        let sampling = match image_brush.sampler.quality {
            peniko::ImageQuality::Low => {
                SkSamplingOptions::new(FilterMode::Nearest, MipmapMode::None)
            }
            peniko::ImageQuality::Medium => {
                SkSamplingOptions::new(FilterMode::Linear, MipmapMode::Nearest)
            }
            peniko::ImageQuality::High => {
                SkSamplingOptions::new(FilterMode::Linear, MipmapMode::Linear)
            }
        };

        shaders::image(
            image,
            (
                tile_mode_from_extend(image_brush.sampler.x_extend),
                tile_mode_from_extend(image_brush.sampler.y_extend),
            ),
            &sampling,
            &brush_transform.map(super::sk_kurbo::matrix_from_affine),
        )
    }

    pub(super) fn shader_from_gradient(
        gradient: &Gradient,
        brush_transform: Option<kurbo::Affine>,
    ) -> SkShader {
        fn rad_to_deg(rad: f32) -> f32 {
            if rad == 0.0 {
                return 0.0;
            }

            rad * 180.0 / std::f32::consts::PI
        }

        match gradient.kind {
            GradientKind::Linear(linear_gradient_position) => {
                let mut colors: Vec<SkColor4f> = vec![];
                let mut positions: Vec<f32> = vec![];

                for color_stop in gradient.stops.iter() {
                    colors.push(color4f_from_dynamic_color(color_stop.color));
                    positions.push(color_stop.offset);
                }
                let start = super::sk_kurbo::pt_from(linear_gradient_position.start);
                let end = super::sk_kurbo::pt_from(linear_gradient_position.end);

                let interpolation = gradient::Interpolation {
                    color_space: gradient_shader_cs_from_cs_tag(gradient.interpolation_cs),
                    in_premul: gradient::interpolation::InPremul::Yes,
                    hue_method: gradient_shader_hue_method_from_hue_direction(
                        gradient.hue_direction,
                    ),
                };

                linear_gradient(
                    (start, end),
                    &gradient::Gradient::new(
                        Colors::new(
                            &colors[..],
                            Some(&positions[..]),
                            tile_mode_from_extend(gradient.extend),
                            None,
                        ),
                        interpolation,
                    ),
                    &brush_transform.map(super::sk_kurbo::matrix_from_affine),
                )
                .unwrap()
            }
            GradientKind::Radial(radial_gradient_position) => {
                let mut colors: Vec<SkColor4f> = vec![];
                let mut positions: Vec<f32> = vec![];

                for color_stop in gradient.stops.iter() {
                    colors.push(color4f_from_dynamic_color(color_stop.color));
                    positions.push(color_stop.offset);
                }

                let start_center = super::sk_kurbo::pt_from(radial_gradient_position.start_center);
                let start_radius = radial_gradient_position.start_radius;
                let end_center = super::sk_kurbo::pt_from(radial_gradient_position.end_center);
                let end_radius = radial_gradient_position.end_radius;

                let interpolation = gradient::Interpolation {
                    color_space: gradient_shader_cs_from_cs_tag(gradient.interpolation_cs),
                    in_premul: gradient::interpolation::InPremul::Yes,
                    hue_method: gradient_shader_hue_method_from_hue_direction(
                        gradient.hue_direction,
                    ),
                };

                if start_center == end_center && start_radius == end_radius {
                    radial_gradient(
                        (start_center, start_radius),
                        &gradient::Gradient::new(
                            Colors::new(
                                &colors[..],
                                Some(&positions[..]),
                                tile_mode_from_extend(gradient.extend),
                                None,
                            ),
                            interpolation,
                        ),
                        &brush_transform.map(super::sk_kurbo::matrix_from_affine),
                    )
                    .unwrap()
                } else {
                    two_point_conical_gradient(
                        (start_center, start_radius),
                        (end_center, end_radius),
                        &gradient::Gradient::new(
                            Colors::new(
                                &colors[..],
                                Some(&positions[..]),
                                tile_mode_from_extend(gradient.extend),
                                None,
                            ),
                            interpolation,
                        ),
                        &brush_transform.map(super::sk_kurbo::matrix_from_affine),
                    )
                    .unwrap()
                }
            }
            GradientKind::Sweep(sweep_gradient_position) => {
                let mut colors: Vec<SkColor4f> = vec![];
                let mut positions: Vec<f32> = vec![];

                for color_stop in gradient.stops.iter() {
                    colors.push(color4f_from_dynamic_color(color_stop.color));
                    positions.push(color_stop.offset);
                }
                let center = super::sk_kurbo::pt_from(sweep_gradient_position.center);

                let interpolation = gradient::Interpolation {
                    color_space: gradient_shader_cs_from_cs_tag(gradient.interpolation_cs),
                    in_premul: gradient::interpolation::InPremul::Yes,
                    hue_method: gradient_shader_hue_method_from_hue_direction(
                        gradient.hue_direction,
                    ),
                };

                sweep_gradient(
                    center,
                    (
                        rad_to_deg(sweep_gradient_position.start_angle),
                        rad_to_deg(sweep_gradient_position.end_angle),
                    ),
                    &gradient::Gradient::new(
                        Colors::new(
                            &colors[..],
                            Some(&positions[..]),
                            tile_mode_from_extend(gradient.extend),
                            None,
                        ),
                        interpolation,
                    ),
                    &brush_transform.map(super::sk_kurbo::matrix_from_affine),
                )
                .unwrap()
            }
        }
    }

    pub(super) fn path_fill_type_from_fill(fill: Fill) -> SkPathFillType {
        match fill {
            Fill::NonZero => SkPathFillType::Winding,
            Fill::EvenOdd => SkPathFillType::EvenOdd,
        }
    }

    pub(super) fn color4f_from_alpha_color(color: AlphaColor<Srgb>) -> SkColor4f {
        SkColor4f::new(
            color.components[0],
            color.components[1],
            color.components[2],
            color.components[3],
        )
    }

    pub(super) fn color4f_from_dynamic_color(color: DynamicColor) -> SkColor4f {
        let color = color.to_alpha_color::<Srgb>();
        SkColor4f::new(
            color.components[0],
            color.components[1],
            color.components[2],
            color.components[3],
        )
    }

    pub(super) fn gradient_shader_cs_from_cs_tag(
        color_space: ColorSpaceTag,
    ) -> SkGradientShaderColorSpace {
        match color_space {
            ColorSpaceTag::Srgb => SkGradientShaderColorSpace::SRGB,
            ColorSpaceTag::LinearSrgb => SkGradientShaderColorSpace::SRGBLinear,
            ColorSpaceTag::Lab => SkGradientShaderColorSpace::Lab,
            ColorSpaceTag::Lch => SkGradientShaderColorSpace::LCH,
            ColorSpaceTag::Hsl => SkGradientShaderColorSpace::HSL,
            ColorSpaceTag::Hwb => SkGradientShaderColorSpace::HWB,
            ColorSpaceTag::Oklab => SkGradientShaderColorSpace::OKLab,
            ColorSpaceTag::Oklch => SkGradientShaderColorSpace::OKLCH,
            ColorSpaceTag::DisplayP3 => SkGradientShaderColorSpace::DisplayP3,
            ColorSpaceTag::A98Rgb => SkGradientShaderColorSpace::A98RGB,
            ColorSpaceTag::ProphotoRgb => SkGradientShaderColorSpace::ProphotoRGB,
            ColorSpaceTag::Rec2020 => SkGradientShaderColorSpace::Rec2020,
            _ => SkGradientShaderColorSpace::SRGB, // ToDo: overview unsupported color space tags and possibly document it, for now just fallback
        }
    }

    pub(super) fn gradient_shader_hue_method_from_hue_direction(
        direction: HueDirection,
    ) -> SkGradientShaderHueMethod {
        match direction {
            HueDirection::Shorter => SkGradientShaderHueMethod::Shorter,
            HueDirection::Longer => SkGradientShaderHueMethod::Longer,
            HueDirection::Increasing => SkGradientShaderHueMethod::Increasing,
            HueDirection::Decreasing => SkGradientShaderHueMethod::Decreasing,
            _ => unreachable!(),
        }
    }

    pub(super) fn tile_mode_from_extend(extend: Extend) -> SkTileMode {
        match extend {
            Extend::Pad => SkTileMode::Clamp,
            Extend::Repeat => SkTileMode::Repeat,
            Extend::Reflect => SkTileMode::Mirror,
        }
    }

    pub(super) fn blend_mode_from(blend_mode: BlendMode) -> SkBlendMode {
        if blend_mode.mix == Mix::Normal {
            match blend_mode.compose {
                Compose::Clear => SkBlendMode::Clear,
                Compose::Copy => SkBlendMode::Src,
                Compose::Dest => SkBlendMode::Dst,
                Compose::SrcOver => SkBlendMode::SrcOver,
                Compose::DestOver => SkBlendMode::DstOver,
                Compose::SrcIn => SkBlendMode::SrcIn,
                Compose::DestIn => SkBlendMode::DstIn,
                Compose::SrcOut => SkBlendMode::SrcOut,
                Compose::DestOut => SkBlendMode::DstOut,
                Compose::SrcAtop => SkBlendMode::SrcATop,
                Compose::DestAtop => SkBlendMode::DstATop,
                Compose::Xor => SkBlendMode::Xor,
                Compose::Plus => SkBlendMode::Plus,
                Compose::PlusLighter => SkBlendMode::Plus,
            }
        } else {
            match blend_mode.mix {
                Mix::Normal => unreachable!(), // Handled above
                Mix::Multiply => SkBlendMode::Multiply,
                Mix::Screen => SkBlendMode::Screen,
                Mix::Overlay => SkBlendMode::Overlay,
                Mix::Darken => SkBlendMode::Darken,
                Mix::Lighten => SkBlendMode::Lighten,
                Mix::ColorDodge => SkBlendMode::ColorDodge,
                Mix::ColorBurn => SkBlendMode::ColorBurn,
                Mix::HardLight => SkBlendMode::HardLight,
                Mix::SoftLight => SkBlendMode::SoftLight,
                Mix::Difference => SkBlendMode::Difference,
                Mix::Exclusion => SkBlendMode::Exclusion,
                Mix::Hue => SkBlendMode::Hue,
                Mix::Saturation => SkBlendMode::Saturation,
                Mix::Color => SkBlendMode::Color,
                Mix::Luminosity => SkBlendMode::Luminosity,
            }
        }
    }
}

mod sk_kurbo {
    use freya_engine::prelude::{
        M44 as SkM44,
        Matrix as SkMatrix,
        Path as SkPath,
        PathBuilder as SkPathBuilder,
        Point as SkPoint,
        RRect as SkRRect,
        Rect as SkRect,
    };
    use kurbo::{
        Affine,
        PathEl,
        Point,
        Rect,
        RoundedRect,
        Shape,
    };

    pub(super) fn rect_from(rect: Rect) -> SkRect {
        SkRect::new(
            rect.x0 as f32,
            rect.y0 as f32,
            rect.x1 as f32,
            rect.y1 as f32,
        )
    }

    pub(super) fn rrect_from(rrect: RoundedRect) -> SkRRect {
        let rect = rect_from(rrect.rect());
        SkRRect::new_nine_patch(
            rect,
            rrect.radii().bottom_left as f32,
            rrect.radii().top_left as f32,
            rrect.radii().top_right as f32,
            rrect.radii().bottom_right as f32,
        )
    }

    pub(super) fn m44_from_affine(affine: Affine) -> SkM44 {
        let m = affine.as_coeffs();
        let scale_x = m[0] as f32;
        let shear_y = m[1] as f32;
        let shear_x = m[2] as f32;
        let scale_y = m[3] as f32;
        let translate_x = m[4] as f32;
        let translate_y = m[5] as f32;

        SkM44::col_major(&[
            scale_x,
            shear_y,
            0.0,
            0.0,
            shear_x,
            scale_y,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            translate_x,
            translate_y,
            0.0,
            1.0,
        ])
    }

    pub(super) fn matrix_from_affine(affine: Affine) -> SkMatrix {
        let m = affine.as_coeffs();
        let scale_x = m[0] as f32;
        let shear_y = m[1] as f32;
        let shear_x = m[2] as f32;
        let scale_y = m[3] as f32;
        let translate_x = m[4] as f32;
        let translate_y = m[5] as f32;

        SkMatrix::new_all(
            scale_x,
            shear_x,
            translate_x,
            shear_y,
            scale_y,
            translate_y,
            0.0,
            0.0,
            1.0,
        )
    }

    pub(super) fn pt_from(p: Point) -> SkPoint {
        SkPoint::new(p.x as f32, p.y as f32)
    }

    pub(super) fn path_from_shape(shape: &impl Shape) -> SkPath {
        let mut sk_path = SkPathBuilder::new();

        if let Some(path_els) = shape.as_path_slice() {
            for path_el in path_els {
                append_path_el_to_sk_path(path_el, &mut sk_path);
            }
        } else {
            for path_el in shape.path_elements(0.1) {
                append_path_el_to_sk_path(&path_el, &mut sk_path);
            }
        }

        sk_path.detach()
    }

    fn append_path_el_to_sk_path(path_el: &PathEl, sk_path: &mut SkPathBuilder) {
        match path_el {
            PathEl::MoveTo(p) => _ = sk_path.move_to(pt_from(*p)),
            PathEl::LineTo(p) => _ = sk_path.line_to(pt_from(*p)),
            PathEl::QuadTo(p1, p2) => _ = sk_path.quad_to(pt_from(*p1), pt_from(*p2)),
            PathEl::CurveTo(p1, p2, p3) => {
                _ = sk_path.cubic_to(pt_from(*p1), pt_from(*p2), pt_from(*p3))
            }
            PathEl::ClosePath => _ = sk_path.close(),
        };
    }
}
