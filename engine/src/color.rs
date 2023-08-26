use std::{default, ops::*};

use bitflags::bitflags;

#[derive(Clone, Debug, PartialEq, Copy, Eq)]
pub struct Color(u32);

impl From<u32> for Color {
    fn from(argb: u32) -> Self {
        Color(argb)
    }
}

impl Default for Color {
    fn default() -> Self {
        unimplemented!("This is mocked")
    }
}

impl Color {
    pub const TRANSPARENT: Self = Color(0);
    pub const BLACK: Self = Color(4278190080);
    pub const DARK_GRAY: Self = Color(4282664004);
    pub const GRAY: Self = Color(4287137928);
    pub const LIGHT_GRAY: Self = Color(4291611852);
    pub const WHITE: Self = Color(4294967295);
    pub const RED: Self = Color(4294901760);
    pub const GREEN: Self = Color(4278255360);
    pub const BLUE: Self = Color(4278190335);
    pub const YELLOW: Self = Color(4294967040);
    pub const CYAN: Self = Color(4278255615);
    pub const MAGENTA: Self = Color(4294902015);

    #[inline]
    pub fn new(argb: u32) -> Self {
        unimplemented!("This is mocked")
    }

    // Don't use the u8cpu type in the arguments here, because we trust the Rust compiler to
    // optimize the storage type.
    #[inline]
    pub fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Color {
        unimplemented!("This is mocked")
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        unimplemented!("This is mocked")
    }

    #[inline]
    pub fn a(self) -> u8 {
        unimplemented!("This is mocked")
    }

    #[inline]
    pub fn r(self) -> u8 {
        unimplemented!("This is mocked")
    }

    #[inline]
    pub fn g(self) -> u8 {
        unimplemented!("This is mocked")
    }

    #[inline]
    pub fn b(self) -> u8 {
        unimplemented!("This is mocked")
    }

    #[inline]
    #[must_use]
    pub fn with_a(self, a: u8) -> Self {
        unimplemented!("This is mocked")
    }

    #[inline]
    pub fn to_rgb(self) -> RGB {
        unimplemented!("This is mocked")
    }

    #[inline]
    pub fn to_hsv(self) -> HSV {
        unimplemented!("This is mocked")
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<(u8, u8, u8)> for RGB {
    fn from(rgb: (u8, u8, u8)) -> Self {
        unimplemented!("This is mocked")
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl From<(f32, f32, f32)> for HSV {
    fn from(hsv: (f32, f32, f32)) -> Self {
        unimplemented!("This is mocked")
    }
}

impl HSV {
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        unimplemented!("This is mocked")
    }

    pub fn to_color(self, alpha: u8) -> Color {
        unimplemented!("This is mocked")
    }
}

pub enum GradientShaderColors<'a> {
    Colors(&'a [Color]),
    // ColorsInSpace(&'a [Color4f], Option<ColorSpace>),
}

pub struct Shader;

impl Shader {
    pub fn linear_gradient<'a>(
        points: (impl Into<Point>, impl Into<Point>),
        colors: impl Into<GradientShaderColors<'a>>,
        pos: impl Into<Option<&'a [f32]>>,
        mode: TileMode,
        flags: impl Into<Option<GradientFlags>>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Self> {
        unimplemented!("This is mocked")
    }
}

pub enum TileMode {
    Clamp = 0,
    Repeat = 1,
    Mirror = 2,
    Decal = 3,
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GradientFlags: u32 {
        const INTERPOLATE_COLORS_IN_PREMUL = 1;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Matrix {
    mat: [f32; 9usize],
    type_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(_: f32, _: f32) -> Self {
        unimplemented!("This is mocked")
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul<f32> for Point {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Point {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TextShadow {
    pub color: Color,
    pub offset: Point,
    pub blur_sigma: f64,
}

impl Default for TextShadow {
    fn default() -> Self {
        unimplemented!("This is mocked")
    }
}

impl From<(f32, f32)> for Point {
    fn from(source: (f32, f32)) -> Self {
        Point::new(source.0, source.1)
    }
}

impl From<(i32, i32)> for Point {
    fn from(source: (i32, i32)) -> Self {
        (source.0 as f32, source.1 as f32).into()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct Weight(i32);

#[allow(non_upper_case_globals)]
impl Weight {
    pub const INVISIBLE: Self = Self(0);
    pub const THIN: Self = Self(100);
    pub const EXTRA_LIGHT: Self = Self(200);
    pub const LIGHT: Self = Self(300);
    pub const NORMAL: Self = Self(400);
    pub const MEDIUM: Self = Self(500);
    pub const SEMI_BOLD: Self = Self(600);
    pub const BOLD: Self = Self(700);
    pub const EXTRA_BOLD: Self = Self(800);
    pub const BLACK: Self = Self(900);
    pub const EXTRA_BLACK: Self = Self(1000);
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Slant {
    Upright = 0,
    Italic = 1,
    Oblique = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct Width(i32);

#[allow(non_upper_case_globals)]
impl Width {
    pub const ULTRA_CONDENSED: Self = Self(1);
    pub const EXTRA_CONDENSED: Self = Self(2);
    pub const CONDENSED: Self = Self(3);
    pub const SEMI_CONDENSED: Self = Self(4);
    pub const NORMAL: Self = Self(5);
    pub const SEMI_EXPANDED: Self = Self(6);
    pub const EXPANDED: Self = Self(7);
    pub const EXTRA_EXPANDED: Self = Self(8);
    pub const ULTRA_EXPANDED: Self = Self(9);
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextDecoration: u32 {
        const NO_DECORATION = 0;
        const UNDERLINE = 1;
        const OVERLINE = 2;
        const LINE_THROUGH = 4;
    }
}

impl Default for TextDecoration {
    fn default() -> Self {
        TextDecoration::NO_DECORATION
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Decoration {
    pub ty: TextDecoration,
    pub mode: TextDecorationMode,
    pub color: Color,
    pub style: TextDecorationStyle,
    pub thickness_multiplier: f32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum TextDecorationMode {
    #[default]
    Gaps = 0,
    Through = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum TextDecorationStyle {
    #[default]
    Solid = 0,
    Double = 1,
    Dotted = 2,
    Dashed = 3,
    Wavy = 4,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left = 0,
    Right = 1,
    Center = 2,
    Justify = 3,
    Start = 4,
    End = 5,
}

pub struct TextStyle;

impl TextStyle {
    pub fn new() -> Self {
        unimplemented!("This is mocked")
    }

    #[deprecated(since = "0.51.0", note = "Use clone_for_placeholder")]
    #[must_use]
    pub fn to_placeholder(&self) -> Self {
        unimplemented!("This is mocked")
    }

    #[must_use]
    pub fn clone_for_placeholder(&self) -> Self {
        unimplemented!("This is mocked")
    }

    pub fn equals(&self, other: &TextStyle) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn equals_by_fonts(&self, that: &TextStyle) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn color(&self) -> Color {
        unimplemented!("This is mocked")
    }

    pub fn set_color(&mut self, color: impl Into<Color>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn foreground(&self) -> Paint {
        unimplemented!("This is mocked")
    }

    pub fn set_foreground_color(&mut self, paint: &Paint) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn clear_foreground_color(&mut self) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn background(&self) -> Paint {
        unimplemented!("This is mocked")
    }

    pub fn set_background_color(&mut self, paint: &Paint) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn clear_background_color(&mut self) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn decoration(&self) -> &Decoration {
        unimplemented!("This is mocked")
    }

    pub fn decoration_mut(&mut self) -> &mut Decoration {
        unimplemented!("This is mocked")
    }

    pub fn font_style(&self) -> FontStyle {
        unimplemented!("This is mocked")
    }

    pub fn set_font_style(&mut self, font_style: FontStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn shadows(&self) -> &[TextShadow] {
        unimplemented!("This is mocked")
    }

    pub fn add_shadow(&mut self, shadow: TextShadow) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn reset_shadows(&mut self) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn font_features(&self) -> &[FontFeature] {
        unimplemented!("This is mocked")
    }

    pub fn add_font_feature(&mut self, font_feature: impl AsRef<str>, value: i32) {
        unimplemented!("This is mocked")
    }

    pub fn reset_font_features(&mut self) {
        unimplemented!("This is mocked")
    }

    pub fn font_size(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn set_font_size(&mut self, size: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn font_families(&self) -> FontFamilies {
        unimplemented!("This is mocked")
    }

    pub fn set_font_families(&mut self, families: &[impl AsRef<str>]) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn baseline_shift(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn set_baseline_shift(&mut self, baseline_shift: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn set_height(&mut self, height: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn height(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn set_height_override(&mut self, height_override: bool) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn height_override(&self) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn set_half_leading(&mut self, half_leading: bool) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn half_leading(&self) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn set_letter_spacing(&mut self, letter_spacing: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn letter_spacing(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn set_word_spacing(&mut self, word_spacing: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn word_spacing(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn typeface(&self) -> Option<Typeface> {
        unimplemented!("This is mocked")
    }

    pub fn set_typeface(&mut self, typeface: impl Into<Option<Typeface>>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn locale(&self) -> &str {
        unimplemented!("This is mocked")
    }

    pub fn set_locale(&mut self, locale: impl AsRef<str>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn text_baseline(&self) -> TextBaseline {
        unimplemented!("This is mocked")
    }

    pub fn set_text_baseline(&mut self, baseline: TextBaseline) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn font_metrics(&self) -> FontMetrics {
        unimplemented!("This is mocked")
    }

    pub fn is_placeholder(&self) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn set_placeholder(&mut self) -> &mut Self {
        unimplemented!("This is mocked")
    }
}

pub struct Typeface;

pub struct FontMetrics;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextBaseline {
    Alphabetic = 0,
    Ideographic = 1,
}

pub struct FontFamilies;

pub struct Paint;

pub struct FontStyle;

impl FontStyle {
    pub fn new(_weight: Weight, _width: Width, _slant: Slant) -> Self {
        unimplemented!("This is mocked")
    }
}

pub struct FontFeature;

pub struct FontCollection;

pub struct Paragraph;

impl Paragraph {
    pub fn max_width(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn height(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn min_intrinsic_width(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn max_intrinsic_width(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn alphabetic_baseline(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn ideographic_baseline(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn longest_line(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn did_exceed_max_lines(&self) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn layout(&mut self, width: f32) {
        unimplemented!("This is mocked")
    }

    pub fn paint(&self, canvas: &mut Canvas, p: impl Into<Point>) {
        unimplemented!("This is mocked")
    }

    /// Returns a vector of bounding boxes that enclose all text between
    /// start and end glyph indexes, including start and excluding end
    pub fn get_rects_for_range(
        &self,
        range: Range<usize>,
        rect_height_style: RectHeightStyle,
        rect_width_style: RectWidthStyle,
    ) -> Vec<TextBox> {
        unimplemented!("This is mocked")
    }

    pub fn get_rects_for_placeholders(&self) -> Vec<TextBox> {
        unimplemented!("This is mocked")
    }

    pub fn get_glyph_position_at_coordinate(&self, p: impl Into<Point>) -> PositionWithAffinity {
        unimplemented!("This is mocked")
    }

    pub fn get_word_boundary(&self, offset: u32) -> Range<usize> {
        unimplemented!("This is mocked")
    }

    pub fn get_line_metrics(&self) -> Vec<LineMetrics> {
        unimplemented!("This is mocked")
    }

    pub fn line_number(&self) -> usize {
        unimplemented!("This is mocked")
    }

    pub fn mark_dirty(&mut self) {
        unimplemented!("This is mocked")
    }

    pub fn unresolved_glyphs(&mut self) -> Option<usize> {
        unimplemented!("This is mocked")
    }

    pub fn get_line_number_at(&self, code_unit_index: usize) -> Option<usize> {
        unimplemented!("This is mocked")
    }

    pub fn get_line_metrics_at(&self, line_number: usize) -> Option<LineMetrics> {
        unimplemented!("This is mocked")
    }

    pub fn get_actual_text_range(&self, line_number: usize, include_spaces: bool) -> Range<usize> {
        unimplemented!("This is mocked")
    }

    pub fn get_glyph_cluster_at(&self, code_unit_index: usize) -> Option<GlyphClusterInfo> {
        unimplemented!("This is mocked")
    }

    pub fn get_closest_glyph_cluster_at(&self, d: impl Into<Point>) -> Option<GlyphClusterInfo> {
        unimplemented!("This is mocked")
    }

    pub fn get_font_at(&self, code_unit_index: usize) -> Font {
        unimplemented!("This is mocked")
    }

    pub fn get_fonts(&self) -> Vec<FontInfo> {
        unimplemented!("This is mocked")
    }
}

#[derive(Default)]
pub struct ParagraphStyle;

impl ParagraphStyle {
    pub fn new() -> Self {
        unimplemented!("This is mocked")
    }

    pub fn strut_style(&self) -> &StrutStyle {
        unimplemented!("This is mocked")
    }

    pub fn set_strut_style(&mut self, strut_style: StrutStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn text_style(&self) -> &TextStyle {
        unimplemented!("This is mocked")
    }

    pub fn set_text_style(&mut self, text_style: &TextStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn text_direction(&self) -> TextDirection {
        unimplemented!("This is mocked")
    }

    pub fn set_text_direction(&mut self, direction: TextDirection) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn text_align(&self) -> TextAlign {
        unimplemented!("This is mocked")
    }

    pub fn set_text_align(&mut self, align: TextAlign) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn max_lines(&self) -> Option<usize> {
        unimplemented!("This is mocked")
    }

    pub fn set_max_lines(&mut self, lines: impl Into<Option<usize>>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    // TODO: Support u16 ellipsis, but why? Doesn't SkString support UTF-8?

    pub fn ellipsis(&self) -> &str {
        unimplemented!("This is mocked")
    }

    pub fn set_ellipsis(&mut self, ellipsis: impl AsRef<str>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn height(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn set_height(&mut self, height: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn text_height_behavior(&self) -> TextHeightBehavior {
        unimplemented!("This is mocked")
    }

    pub fn set_text_height_behavior(&mut self, v: TextHeightBehavior) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn unlimited_lines(&self) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn ellipsized(&self) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn effective_align(&self) -> TextAlign {
        unimplemented!("This is mocked")
    }

    pub fn hinting_is_on(&self) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn turn_hinting_off(&mut self) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn replace_tab_characters(&self) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn set_replace_tab_characters(&mut self, value: bool) -> &mut Self {
        unimplemented!("This is mocked")
    }
}

pub struct ParagraphBuilder;

impl ParagraphBuilder {
    pub fn push_style(&mut self, style: &TextStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn pop(&mut self) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn peek_style(&mut self) -> TextStyle {
        unimplemented!("This is mocked")
    }

    pub fn add_text(&mut self, str: impl AsRef<str>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn add_placeholder(&mut self, placeholder_style: &PlaceholderStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn build(&mut self) -> Paragraph {
        unimplemented!("This is mocked")
    }

    pub fn reset(&mut self) {
        unimplemented!("This is mocked")
    }

    pub fn new(style: &ParagraphStyle, font_collection: &FontCollection) -> Self {
        unimplemented!("This is mocked")
    }
}

pub struct StrutStyle;

pub struct TextHeightBehavior;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextDirection {
    RTL = 0,
    LTR = 1,
}

pub struct PlaceholderStyle;

pub struct Canvas;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum RectHeightStyle {
    /// Provide tight bounding boxes that fit heights per run.
    #[default]
    Tight,
    // The height of the boxes will be the maximum height of all runs in the
    // line. All rects in the same line will be the same height.
    Max,
    // Extends the top and/or bottom edge of the bounds to fully cover any line
    // spacing. The top edge of each line should be the same as the bottom edge
    // of the line above. There should be no gaps in vertical coverage given any
    // ParagraphStyle line_height.
    //
    // The top and bottom of each rect will cover half of the
    // space above and half of the space below the line.
    IncludeLineSpacingMiddle,
    // The line spacing will be added to the top of the rect.
    IncludeLineSpacingTop,
    // The line spacing will be added to the bottom of the rect.
    IncludeLineSpacingBottom,
    Strut,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum RectWidthStyle {
    /// Provide tight bounding boxes that fit widths to the runs of each line
    /// independently.
    #[default]
    Tight,
    /// Extends the width of the last rect of each line to match the position of
    /// the widest rect over all the lines.
    Max,
}

pub struct LineMetrics;

pub struct GlyphClusterInfo;

pub struct TextBox;

pub struct Font;

pub struct FontInfo;

pub struct PositionWithAffinity {
    pub position: i32,
}

pub struct RuntimeEffect;

impl RuntimeEffect {
    pub fn uniforms(&self) -> &[Uniform] {
        unimplemented!("This is mocked")
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Uniform {
    Float = 0,
    Float2 = 1,
    Float3 = 2,
    Float4 = 3,
    Float2x2 = 4,
    Float3x3 = 5,
    Float4x4 = 6,
    Int = 7,
    Int2 = 8,
    Int3 = 9,
    Int4 = 10,
}
