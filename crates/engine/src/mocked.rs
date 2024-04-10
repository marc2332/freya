#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::ops::*;

use bitflags::bitflags;
use glutin::context::PossiblyCurrentContext;

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
    pub fn new(_argb: u32) -> Self {
        unimplemented!("This is mocked")
    }

    #[inline]
    pub fn from_argb(_a: u8, _r: u8, _g: u8, _b: u8) -> Color {
        unimplemented!("This is mocked")
    }

    #[inline]
    pub fn from_rgb(_r: u8, _g: u8, _b: u8) -> Color {
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
    pub fn with_a(self, _a: u8) -> Self {
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
    fn from(_rgb: (u8, u8, u8)) -> Self {
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
    fn from(_hsv: (f32, f32, f32)) -> Self {
        unimplemented!("This is mocked")
    }
}

impl HSV {
    pub fn to_color(self, _alpha: u8) -> Color {
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
        _points: (impl Into<Point>, impl Into<Point>),
        _colors: impl Into<GradientShaderColors<'a>>,
        _pos: impl Into<Option<&'a [f32]>>,
        _mode: TileMode,
        _flags: impl Into<Option<GradientFlags>>,
        _local_matrix: impl Into<Option<&'a Matrix>>,
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

impl Matrix {
    pub fn new_identity() -> Self {
        unimplemented!("This is mocked")
    }

    pub fn set_rotate(&mut self, _degrees: f32, _pivot: impl Into<Option<Point>>) -> &mut Self {
        unimplemented!("This is mocked")
    }
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

impl TextShadow {
    pub fn new(color: Color, _: (f32, f32), _: f32) -> Self {
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

#[derive(Default)]
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

    pub fn equals(&self, _other: &TextStyle) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn equals_by_fonts(&self, _that: &TextStyle) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn color(&self) -> Color {
        unimplemented!("This is mocked")
    }

    pub fn set_color(&mut self, _color: impl Into<Color>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn foreground(&self) -> Paint {
        unimplemented!("This is mocked")
    }

    pub fn set_foreground_color(&mut self, _paint: &Paint) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn clear_foreground_color(&mut self) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn background(&self) -> Paint {
        unimplemented!("This is mocked")
    }

    pub fn set_background_color(&mut self, _paint: &Paint) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn clear_background_color(&mut self) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn decoration(&self) -> &Decoration {
        unimplemented!("This is mocked")
    }

    pub fn set_decoration(&mut self, decoration: &Decoration) {
        unimplemented!("This is mocked")
    }

    pub fn font_style(&self) -> FontStyle {
        unimplemented!("This is mocked")
    }

    pub fn set_font_style(&mut self, _font_style: FontStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn shadows(&self) -> &[TextShadow] {
        unimplemented!("This is mocked")
    }

    pub fn add_shadow(&mut self, _shadow: TextShadow) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn reset_shadows(&mut self) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn font_features(&self) -> &[FontFeature] {
        unimplemented!("This is mocked")
    }

    pub fn add_font_feature(&mut self, _font_feature: impl AsRef<str>, _value: i32) {
        unimplemented!("This is mocked")
    }

    pub fn reset_font_features(&mut self) {
        unimplemented!("This is mocked")
    }

    pub fn font_size(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn set_font_size(&mut self, _size: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn font_families(&self) -> FontFamilies {
        unimplemented!("This is mocked")
    }

    pub fn set_font_families(&mut self, _families: &[impl AsRef<str>]) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn baseline_shift(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn set_baseline_shift(&mut self, _baseline_shift: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn set_height(&mut self, _height: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn height(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn set_height_override(&mut self, _height_override: bool) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn height_override(&self) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn set_half_leading(&mut self, _half_leading: bool) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn half_leading(&self) -> bool {
        unimplemented!("This is mocked")
    }

    pub fn set_letter_spacing(&mut self, _letter_spacing: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn letter_spacing(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn set_word_spacing(&mut self, _word_spacing: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn word_spacing(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn typeface(&self) -> Option<Typeface> {
        unimplemented!("This is mocked")
    }

    pub fn set_typeface(&mut self, _typeface: impl Into<Option<Typeface>>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn locale(&self) -> &str {
        unimplemented!("This is mocked")
    }

    pub fn set_locale(&mut self, _locale: impl AsRef<str>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn text_baseline(&self) -> TextBaseline {
        unimplemented!("This is mocked")
    }

    pub fn set_text_baseline(&mut self, _baseline: TextBaseline) -> &mut Self {
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

#[derive(Default, Clone)]
pub struct Paint;

impl Paint {
    pub fn set_anti_alias(&mut self, _anti_alias: bool) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn set_color(&mut self, _color: impl Into<Color>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn set_style(&mut self, _style: PaintStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn set_shader(&mut self, _shader: impl Into<Option<Shader>>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn set_stroke_width(&mut self, _width: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn set_mask_filter(&mut self, _mask_filter: impl Into<Option<MaskFilter>>) -> &mut Self {
        unimplemented!("This is mocked")
    }
}

pub enum PaintStyle {
    Fill = 0,
    Stroke = 1,
    StrokeAndFill = 2,
}

pub struct FontStyle;

impl FontStyle {
    pub fn new(_weight: Weight, _width: Width, _slant: Slant) -> Self {
        unimplemented!("This is mocked")
    }
}

#[derive(Default, Clone)]
pub struct FontMgr;

impl FontMgr {
    pub fn new_from_data(
        &self,
        _bytes: &[u8],
        _ttc_index: impl Into<Option<usize>>,
    ) -> Option<Typeface> {
        unimplemented!("This is mocked")
    }
}

pub struct FontFeature;

pub struct TypefaceFontProvider;

impl TypefaceFontProvider {
    pub fn new() -> Self {
        unimplemented!("This is mocked")
    }

    pub fn register_typeface(
        &mut self,
        _typeface: Typeface,
        _alias: Option<impl AsRef<str>>,
    ) -> usize {
        unimplemented!("This is mocked")
    }
}

impl From<TypefaceFontProvider> for FontMgr {
    fn from(_provider: TypefaceFontProvider) -> Self {
        unimplemented!("This is mocked")
    }
}

#[derive(Clone)]
pub struct FontCollection;

impl FontCollection {
    pub fn new() -> Self {
        unimplemented!("This is mocked")
    }

    pub fn set_default_font_manager<'a>(
        &mut self,
        _font_manager: impl Into<Option<FontMgr>>,
        _default_family_name: impl Into<Option<&'a str>>,
    ) {
        unimplemented!("This is mocked")
    }

    pub fn set_dynamic_font_manager(&mut self, _font_manager: impl Into<Option<FontMgr>>) {
        unimplemented!("This is mocked")
    }
}

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

    pub fn layout(&mut self, _width: f32) {
        unimplemented!("This is mocked")
    }

    pub fn paint(&self, _canvas: &Canvas, _p: impl Into<Point>) {
        unimplemented!("This is mocked")
    }

    /// Returns a vector of bounding boxes that enclose all text between
    /// start and end glyph indexes, including start and excluding end
    pub fn get_rects_for_range(
        &self,
        _range: Range<usize>,
        _rect_height_style: RectHeightStyle,
        _rect_width_style: RectWidthStyle,
    ) -> Vec<TextBox> {
        unimplemented!("This is mocked")
    }

    pub fn get_rects_for_placeholders(&self) -> Vec<TextBox> {
        unimplemented!("This is mocked")
    }

    pub fn get_glyph_position_at_coordinate(&self, _p: impl Into<Point>) -> PositionWithAffinity {
        unimplemented!("This is mocked")
    }

    pub fn get_word_boundary(&self, _offset: u32) -> Range<usize> {
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

    pub fn get_line_number_at(&self, _code_unit_index: usize) -> Option<usize> {
        unimplemented!("This is mocked")
    }

    pub fn get_line_metrics_at(&self, _line_number: usize) -> Option<LineMetrics> {
        unimplemented!("This is mocked")
    }

    pub fn get_actual_text_range(
        &self,
        _line_number: usize,
        _include_spaces: bool,
    ) -> Range<usize> {
        unimplemented!("This is mocked")
    }

    pub fn get_glyph_cluster_at(&self, _code_unit_index: usize) -> Option<GlyphClusterInfo> {
        unimplemented!("This is mocked")
    }

    pub fn get_closest_glyph_cluster_at(&self, _d: impl Into<Point>) -> Option<GlyphClusterInfo> {
        unimplemented!("This is mocked")
    }

    pub fn get_font_at(&self, _code_unit_index: usize) -> Font {
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

    pub fn set_strut_style(&mut self, _strut_style: StrutStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn text_style(&self) -> &TextStyle {
        unimplemented!("This is mocked")
    }

    pub fn set_text_style(&mut self, _text_style: &TextStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn text_direction(&self) -> TextDirection {
        unimplemented!("This is mocked")
    }

    pub fn set_text_direction(&mut self, _direction: TextDirection) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn text_align(&self) -> TextAlign {
        unimplemented!("This is mocked")
    }

    pub fn set_text_align(&mut self, _align: TextAlign) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn max_lines(&self) -> Option<usize> {
        unimplemented!("This is mocked")
    }

    pub fn set_max_lines(&mut self, _lines: impl Into<Option<usize>>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    // TODO: Support u16 ellipsis, but why? Doesn't SkString support UTF-8?

    pub fn ellipsis(&self) -> &str {
        unimplemented!("This is mocked")
    }

    pub fn set_ellipsis(&mut self, _ellipsis: impl AsRef<str>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn height(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn set_height(&mut self, _height: f32) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn text_height_behavior(&self) -> TextHeightBehavior {
        unimplemented!("This is mocked")
    }

    pub fn set_text_height_behavior(&mut self, _v: TextHeightBehavior) -> &mut Self {
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

    pub fn set_replace_tab_characters(&mut self, _value: bool) -> &mut Self {
        unimplemented!("This is mocked")
    }
}

pub struct ParagraphBuilder;

impl ParagraphBuilder {
    pub fn push_style(&mut self, _style: &TextStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn pop(&mut self) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn peek_style(&mut self) -> TextStyle {
        unimplemented!("This is mocked")
    }

    pub fn add_text(&mut self, _str: impl AsRef<str>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn add_placeholder(&mut self, _placeholder_style: &PlaceholderStyle) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn build(&mut self) -> Paragraph {
        unimplemented!("This is mocked")
    }

    pub fn reset(&mut self) {
        unimplemented!("This is mocked")
    }

    pub fn new(_style: &ParagraphStyle, _font_collection: impl Into<FontCollection>) -> Self {
        unimplemented!("This is mocked")
    }
}

impl From<&FontCollection> for FontCollection {
    fn from(value: &FontCollection) -> Self {
        value.clone()
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

impl Canvas {
    pub fn save(&self) {
        unimplemented!("This is mocked")
    }

    pub fn restore(&self) {
        unimplemented!("This is mocked")
    }

    pub fn concat(&self, _matrix: &Matrix) {
        unimplemented!("This is mocked")
    }

    pub fn clip_rect(&self, _rect: Rect, _clip: ClipOp, _: bool) {
        unimplemented!("This is mocked")
    }

    pub fn draw_image_nine(
        &self,
        _image: Image,
        _center: IRect,
        _dst: Rect,
        _filter_mode: FilterMode,
        _paint: Option<&Paint>,
    ) -> &Self {
        unimplemented!("This is mocked")
    }

    pub fn draw_rect(&self, _rect: Rect, _paint: &Paint) -> &Self {
        unimplemented!("This is mocked")
    }

    pub fn draw_path(&self, _path: &Path, _paint: &Paint) -> &Self {
        unimplemented!("This is mocked")
    }

    pub fn clip_path(
        &self,
        _path: &Path,
        _op: impl Into<Option<ClipOp>>,
        _do_anti_alias: impl Into<Option<bool>>,
    ) -> &Self {
        unimplemented!("This is mocked")
    }

    pub fn translate(&self, _d: impl Into<Point>) -> &Self {
        unimplemented!("This is mocked")
    }

    pub fn scale(&self, _: impl Into<Point>) {
        unimplemented!("This is mocked")
    }

    pub fn clear(&self, _: Color) {
        unimplemented!("This is mocked")
    }

    pub fn draw_line(&self, _p1: impl Into<Point>, _p2: impl Into<Point>, _paint: &Paint) -> &Self {
        unimplemented!("This is mocked")
    }

    pub fn draw_circle(&self, _center: impl Into<Point>, _radius: f32, _paint: &Paint) -> &Self {
        unimplemented!("This is mocked")
    }

    pub fn save_layer_alpha_f(&self, bounds: impl Into<Option<Rect>>, alpha: f32) -> usize {
        unimplemented!("This is mocked")
    }
}

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

pub struct TextBox {
    pub rect: Rect,
}

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

impl Uniform {
    pub fn name(&self) -> &str {
        unimplemented!("This is mocked")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ClipOp {
    Difference = 0,
    Intersect = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    /// The x coordinate of the rectangle's left edge.
    pub left: f32,
    /// The y coordinate of the rectangle's top edge.
    pub top: f32,
    /// The x coordinate of the rectangle's right edge.
    pub right: f32,
    /// The y coordinate of the rectangle's bottom edge.
    pub bottom: f32,
}

impl Rect {
    pub fn new(_left: f32, _top: f32, _right: f32, _bottom: f32) -> Self {
        unimplemented!("This is mocked")
    }
}

pub struct Image;

impl Image {
    pub fn from_encoded(_data: Data) -> Option<Self> {
        unimplemented!("This is mocked")
    }
}

pub struct Data;

impl Data {
    pub fn new_copy(_bytes: &[u8]) -> Self {
        unimplemented!("This is mocked")
    }

    pub unsafe fn new_bytes(_bytes: &[u8]) -> Self {
        unimplemented!("This is mocked")
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct IRect {
    /// The x coordinate of the rectangle's left edge.
    pub left: i32,
    /// The y coordinate of the rectangle's top edge.
    pub top: i32,
    /// The x coordinate of the rectangle's right edge.
    pub right: i32,
    /// The y coordinate of the rectangle's bottom edge.
    pub bottom: i32,
}

impl IRect {
    pub fn new(_left: i32, _top: i32, _right: i32, _bottom: i32) -> Self {
        unimplemented!("This is mocked")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FilterMode {
    Nearest = 0,
    Linear = 1,
}

impl FilterMode {
    pub const Last: FilterMode = FilterMode::Linear;
}

pub struct Path;

impl Path {
    pub fn new() -> Self {
        unimplemented!("This is mocked")
    }

    pub fn add_path(
        &mut self,
        _src: &Path,
        _d: impl Into<Point>,
        _mode: Option<&PathAddPathMode>,
    ) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn move_to(&mut self, _p: impl Into<Point>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn line_to(&mut self, _p: impl Into<Point>) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn cubic_to(
        &mut self,
        _p1: impl Into<Point>,
        _p2: impl Into<Point>,
        _p3: impl Into<Point>,
    ) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn r_arc_to_rotated(
        &mut self,
        _r: impl Into<Point>,
        _x_axis_rotate: f32,
        _large_arc: ArcSize,
        _sweep: PathDirection,
        _d: impl Into<Point>,
    ) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn close(&self) {
        unimplemented!("This is mocked")
    }

    pub fn add_rrect(
        &mut self,
        _rrect: impl AsRef<RRect>,
        _dir_start: Option<(PathDirection, usize)>,
    ) -> &mut Self {
        unimplemented!("This is mocked")
    }

    pub fn offset(&mut self, _d: impl Into<Point>) -> &mut Self {
        unimplemented!("This is mocked")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PathAddPathMode {
    Append = 0,
    Extend = 1,
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct RRect;

impl AsRef<RRect> for RRect {
    fn as_ref(&self) -> &RRect {
        self
    }
}

impl RRect {
    pub fn new_rect_radii(_rect: Rect, _radii: &[Point; 4]) -> Self {
        unimplemented!("This is mocked")
    }

    pub fn width(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn height(&self) -> f32 {
        unimplemented!("This is mocked")
    }

    pub fn radii(&self, _corner: Corner) -> Point {
        unimplemented!("This is mocked")
    }

    pub fn with_outset(&self, _delta: impl Into<Point>) -> Self {
        unimplemented!("This is mocked")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ArcSize {
    Small = 0,
    Large = 1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Corner {
    UpperLeft = 0,
    UpperRight = 1,
    LowerRight = 2,
    LowerLeft = 3,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PathDirection {
    CW = 0,
    CCW = 1,
}

pub struct MaskFilter;

impl MaskFilter {
    pub fn blur(
        _style: BlurStyle,
        _sigma: f32,
        _respect_ctm: impl Into<Option<bool>>,
    ) -> Option<Self> {
        unimplemented!("This is mocked")
    }
}

impl BlurStyle {
    pub const LastEnum: BlurStyle = BlurStyle::Inner;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BlurStyle {
    Normal = 0,
    Solid = 1,
    Outer = 2,
    Inner = 3,
}

pub mod svg {
    use super::{Canvas, FontMgr, Size};

    pub struct Dom;

    impl Dom {
        pub fn from_bytes(_bytes: &[u8], font_mgr: &FontMgr) -> Result<Self, ()> {
            unimplemented!("This is mocked")
        }

        pub fn set_container_size(&mut self, _size: impl Into<Size>) {
            unimplemented!("This is mocked")
        }

        pub fn render(&self, _canvas: &Canvas) {
            unimplemented!("This is mocked")
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Size;

impl From<(f32, f32)> for Size {
    fn from(_source: (f32, f32)) -> Self {
        unimplemented!("This is mocked")
    }
}

impl From<(i32, i32)> for Size {
    fn from(_source: (i32, i32)) -> Self {
        unimplemented!("This is mocked")
    }
}

pub struct Surface;

impl Surface {
    pub fn canvas(&mut self) -> &Canvas {
        unimplemented!("This is mocked")
    }

    pub fn swap_buffers(&self, _: &PossiblyCurrentContext) {
        unimplemented!("This is mocked")
    }

    pub fn from_backend_render_target(
        _context: &mut RecordingContext,
        _backend_render_target: &BackendRenderTarget,
        _origin: SurfaceOrigin,
        _color_type: ColorType,
        _color_space: impl Into<Option<ColorSpace>>,
        _surface_props: Option<&SurfaceProps>,
    ) -> Option<Self> {
        unimplemented!("This is mocked")
    }
}

pub struct ColorSpace;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(i32)]
pub enum ColorType {
    RGBA8888 = 4,
}

pub struct SurfaceProps;

use std::ops::{Deref, DerefMut};

pub struct RecordingContext;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SurfaceOrigin {
    TopLeft = 0,
    BottomLeft = 1,
}

#[repr(C)]
#[derive(Debug)]
pub struct ContextOptions;

pub struct DirectContext;

impl From<DirectContext> for RecordingContext {
    fn from(_direct_context: DirectContext) -> Self {
        unimplemented!("This is mocked")
    }
}

impl Deref for DirectContext {
    type Target = RecordingContext;

    fn deref(&self) -> &Self::Target {
        unimplemented!("This is mocked")
    }
}

impl DerefMut for DirectContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unimplemented!("This is mocked")
    }
}

impl DirectContext {
    pub fn new_gl<'a>(
        _interface: impl Into<Option<Interface>>,
        _options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        unimplemented!("This is mocked")
    }

    pub fn flush_and_submit(&self) {
        unimplemented!("This is mocked")
    }

    pub fn abandon(&self) {
        unimplemented!("This is mocked")
    }
}

use std::ffi::c_void;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Protected {
    No,
    Yes,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct FramebufferInfo {
    pub fboid: i32,
    pub format: Format,
    pub protected: Protected,
}

impl Default for FramebufferInfo {
    fn default() -> Self {
        unimplemented!("This is mocked")
    }
}

pub fn wrap_backend_render_target(
    context: &mut RecordingContext,
    backend_render_target: &BackendRenderTarget,
    origin: SurfaceOrigin,
    color_type: ColorType,
    color_space: impl Into<Option<ColorSpace>>,
    surface_props: Option<&SurfaceProps>,
) -> Option<Surface> {
    unimplemented!("This is mocked")
}

pub struct Interface;

impl Interface {
    pub fn new_load_with<F>(_load_fn: F) -> Option<Self>
    where
        F: FnMut(&str) -> *const c_void,
    {
        unimplemented!("This is mocked")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Format {
    Unknown = 0,
    RGBA8 = 1,
    R8 = 2,
    ALPHA8 = 3,
    LUMINANCE8 = 4,
    LUMINANCE8_ALPHA8 = 5,
    BGRA8 = 6,
    RGB565 = 7,
    RGBA16F = 8,
    R16F = 9,
    RGB8 = 10,
    RGBX8 = 11,
    RG8 = 12,
    RGB10_A2 = 13,
    RGBA4 = 14,
    SRGB8_ALPHA8 = 15,
    COMPRESSED_ETC1_RGB8 = 16,
    COMPRESSED_RGB8_ETC2 = 17,
    COMPRESSED_RGB8_BC1 = 18,
    COMPRESSED_RGBA8_BC1 = 19,
    R16 = 20,
    RG16 = 21,
    RGBA16 = 22,
    RG16F = 23,
    LUMINANCE16F = 24,
    STENCIL_INDEX8 = 25,
    STENCIL_INDEX16 = 26,
    DEPTH24_STENCIL8 = 27,
}

pub struct BackendRenderTarget;

impl BackendRenderTarget {
    pub fn new_gl(
        (_width, _height): (i32, i32),
        _sample_count: impl Into<Option<usize>>,
        _stencil_bits: usize,
        _info: FramebufferInfo,
    ) -> Self {
        unimplemented!("This is mocked")
    }
}

pub mod backend_render_targets {
    use crate::prelude::*;
    pub fn make_gl(
        (width, height): (i32, i32),
        sample_count: impl Into<Option<usize>>,
        stencil_bits: usize,
        info: FramebufferInfo,
    ) -> BackendRenderTarget {
        unimplemented!("This is mocked")
    }
}

pub fn set_resource_cache_total_bytes_limit(new_limit: usize) -> usize {
    unimplemented!("This is mocked")
}

pub fn set_resource_cache_single_allocation_byte_limit(new_limit: Option<usize>) -> Option<usize> {
    unimplemented!("This is mocked")
}
