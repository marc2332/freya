use std::{
    borrow::Cow,
    hash::{
        Hash,
        Hasher,
    },
};

use paste::paste;
use rustc_hash::{
    FxHashMap,
    FxHasher,
};
use torin::{
    content::Content,
    gaps::Gaps,
    prelude::{
        Alignment,
        Direction,
        Length,
        Position,
        VisibleSize,
    },
    size::{
        Size,
        SizeFn,
        SizeFnContext,
    },
};

use crate::{
    data::{
        AccessibilityData,
        LayoutData,
        TextStyleData,
    },
    diff_key::DiffKey,
    element::{
        Element,
        EventHandlerType,
    },
    elements::image::{
        AspectRatio,
        ImageCover,
        ImageData,
        SamplingMode,
    },
    event_handler::EventHandler,
    events::{
        data::{
            Event,
            KeyboardEventData,
            MouseEventData,
            SizedEventData,
            WheelEventData,
        },
        name::EventName,
    },
    layers::Layer,
    prelude::*,
    style::{
        font_size::FontSize,
        font_slant::FontSlant,
        font_weight::FontWeight,
        font_width::FontWidth,
        text_height::TextHeightBehavior,
        text_overflow::TextOverflow,
        text_shadow::TextShadow,
    },
};

pub trait SizeExt {
    fn auto() -> Size;
    fn fill() -> Size;
    fn fill_minimum() -> Size;
    fn percent(percent: impl Into<f32>) -> Size;
    fn px(px: impl Into<f32>) -> Size;
    fn window_percent(percent: impl Into<f32>) -> Size;
    fn flex(flex: impl Into<f32>) -> Size;
    fn func(func: impl Fn(SizeFnContext) -> Option<f32> + 'static + Sync + Send) -> Size;
    fn func_data<D: Hash>(
        func: impl Fn(SizeFnContext) -> Option<f32> + 'static + Sync + Send,
        data: &D,
    ) -> Size;
}

impl SizeExt for Size {
    fn auto() -> Size {
        Size::Inner
    }

    fn fill() -> Size {
        Size::Fill
    }

    fn fill_minimum() -> Size {
        Size::FillMinimum
    }

    fn percent(percent: impl Into<f32>) -> Size {
        Size::Percentage(Length::new(percent.into()))
    }

    fn px(px: impl Into<f32>) -> Size {
        Size::Pixels(Length::new(px.into()))
    }

    fn window_percent(percent: impl Into<f32>) -> Size {
        Size::RootPercentage(Length::new(percent.into()))
    }

    fn flex(flex: impl Into<f32>) -> Size {
        Size::Flex(Length::new(flex.into()))
    }

    fn func(func: impl Fn(SizeFnContext) -> Option<f32> + 'static + Sync + Send) -> Size {
        Self::Fn(Box::new(SizeFn::new(func)))
    }

    fn func_data<D: Hash>(
        func: impl Fn(SizeFnContext) -> Option<f32> + 'static + Sync + Send,
        data: &D,
    ) -> Size {
        Self::Fn(Box::new(SizeFn::new_data(func, data)))
    }
}

pub trait DirectionExt {
    fn vertical() -> Direction;
    fn horizontal() -> Direction;
}

impl DirectionExt for Direction {
    fn vertical() -> Direction {
        Direction::Vertical
    }
    fn horizontal() -> Direction {
        Direction::Horizontal
    }
}

pub trait AlignmentExt {
    fn start() -> Alignment;
    fn center() -> Alignment;
    fn end() -> Alignment;
    fn space_between() -> Alignment;
    fn space_evenly() -> Alignment;
    fn space_around() -> Alignment;
}

impl AlignmentExt for Alignment {
    fn start() -> Alignment {
        Alignment::Start
    }

    fn center() -> Alignment {
        Alignment::Center
    }

    fn end() -> Alignment {
        Alignment::End
    }

    fn space_between() -> Alignment {
        Alignment::SpaceBetween
    }

    fn space_evenly() -> Alignment {
        Alignment::SpaceEvenly
    }

    fn space_around() -> Alignment {
        Alignment::SpaceAround
    }
}

pub trait ContentExt {
    fn normal() -> Content;
    fn fit() -> Content;
    fn flex() -> Content;
    fn wrap() -> Content;
}

impl ContentExt for Content {
    fn normal() -> Content {
        Content::Normal
    }

    fn fit() -> Content {
        Content::Fit
    }

    fn flex() -> Content {
        Content::Flex
    }

    fn wrap() -> Content {
        Content::Wrap
    }
}

pub trait VisibleSizeExt {
    fn full() -> VisibleSize;
    fn inner_percent(value: impl Into<f32>) -> VisibleSize;
}

impl VisibleSizeExt for VisibleSize {
    fn full() -> VisibleSize {
        VisibleSize::Full
    }

    fn inner_percent(value: impl Into<f32>) -> VisibleSize {
        VisibleSize::InnerPercentage(Length::new(value.into()))
    }
}

pub trait ChildrenExt: Sized {
    fn get_children(&mut self) -> &mut Vec<Element>;

    fn children(mut self, children: impl IntoIterator<Item = Element>) -> Self        
    {
        self.get_children().extend(children);
        self
    }

    fn maybe_child<C: IntoElement>(mut self, child: Option<C>) -> Self {
        if let Some(child) = child {
            self.get_children().push(child.into_element());
        }
        self
    }

    fn child<C: IntoElement>(mut self, child: C) -> Self {
        self.get_children().push(child.into_element());
        self
    }
}

pub trait KeyExt: Sized {
    fn write_key(&mut self) -> &mut DiffKey;

    fn key(mut self, key: impl Hash) -> Self {
        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        *self.write_key() = DiffKey::U64(hasher.finish());
        self
    }
}

pub trait ListExt {
    fn with(self, other: Self) -> Self;
}

impl<T> ListExt for Vec<T> {
    fn with(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

macro_rules! event_handlers {
    (
        $handler_variant:ident, $event_data:ty;
        $(
            $name:ident => $event_variant:expr ;
        )*
    ) => {
        paste! {
            $(
                fn [<on_$name>](mut self, [<on_$name>]: impl Into<EventHandler<Event<$event_data>>>) -> Self {
                    self.get_event_handlers()
                        .insert($event_variant, EventHandlerType::$handler_variant([<on_$name>].into()));
                    self
                }
            )*
        }
    };
}

pub trait EventHandlersExt: Sized + LayoutExt {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType>;

    event_handlers! {
        Mouse,
        MouseEventData;

        mouse_down => EventName::MouseDown;
        mouse_up => EventName::MouseUp;
        mouse_move => EventName::MouseMove;

        global_mouse_up => EventName::GlobalMouseUp;
        global_mouse_down => EventName::GlobalMouseDown;
        global_mouse_move => EventName::GlobalMouseMove;

        capture_global_mouse_move => EventName::CaptureGlobalMouseMove;
        capture_global_mouse_up => EventName::CaptureGlobalMouseUp;
    }

    event_handlers! {
        Keyboard,
        KeyboardEventData;

        key_down => EventName::KeyDown;
        key_up => EventName::KeyUp;

        global_key_down => EventName::GlobalKeyDown;
        global_key_up => EventName::GlobalKeyUp;
    }

    event_handlers! {
        Wheel,
        WheelEventData;

        wheel => EventName::Wheel;
    }

    event_handlers! {
        Touch,
        TouchEventData;

        touch_cancel => EventName::TouchCancel;
        touch_start => EventName::TouchStart;
        touch_move => EventName::TouchMove;
        touch_end => EventName::TouchEnd;
    }

    event_handlers! {
        Pointer,
        PointerEventData;

        pointer_press => EventName::PointerPress;
        pointer_down => EventName::PointerDown;
        pointer_enter => EventName::PointerEnter;
        pointer_leave => EventName::PointerLeave;
    }

    event_handlers! {
        File,
        FileEventData;

        file_drop => EventName::FileDrop;
        global_file_hover => EventName::GlobalFileHover;
        global_file_hover_cancelled => EventName::GlobalFileHoverCancelled;
    }

    event_handlers! {
        ImePreedit,
        ImePreeditEventData;

        ime_preedit => EventName::ImePreedit;
    }

    fn on_sized(mut self, on_sized: impl Into<EventHandler<Event<SizedEventData>>>) -> Self {
        self.get_event_handlers()
            .insert(EventName::Sized, EventHandlerType::Sized(on_sized.into()));
        self.get_layout().layout.has_layout_references = true;
        self
    }

    /// This is generally the best event in which to run "press" logic, this might be called `onClick`, `onActivate`, or `onConnect` in other platforms.
    ///
    /// Gets triggered when:
    /// - **Click**: There is a `MouseUp` event (Left button) with the in the same element that there had been a `MouseDown` just before
    /// - **Touched**: There is a `TouchEnd` event in the same element that there had been a `TouchStart` just before
    /// - **Activated**: The element is focused and there is a keydown event pressing the OS activation key (e.g Space, Enter)
    fn on_press(self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        let on_press = on_press.into();
        self.on_pointer_press({
            let on_press = on_press.clone();
            move |e: Event<PointerEventData>| {
                let event = e.try_map(|d| match d {
                    PointerEventData::Mouse(m) if m.button == Some(MouseButton::Left) => {
                        Some(PressEventData::Mouse(m))
                    }
                    PointerEventData::Touch(t) => Some(PressEventData::Touch(t)),
                    _ => None,
                });
                if let Some(event) = event {
                    on_press.call(event);
                }
            }
        })
        .on_key_down({
            let on_press = on_press.clone();
            move |e: Event<KeyboardEventData>| {
                if Focus::is_pressed(&e) {
                    on_press.call(e.map(PressEventData::Keyboard))
                }
            }
        })
    }

    /// Also called the context menu click in other platforms.
    /// Gets triggered when:
    /// - **Click**: There is a `MouseUp` (Right button) event in the same element that there had been a `MouseDown` just before
    fn on_secondary_press(
        self,
        on_pointer_press: impl Into<EventHandler<Event<PressEventData>>>,
    ) -> Self {
        let on_pointer_press = on_pointer_press.into();
        self.on_pointer_press({
            let on_pointer_press = on_pointer_press.clone();
            move |e: Event<PointerEventData>| {
                let event = e.try_map(|d| match d {
                    PointerEventData::Mouse(m) if m.button == Some(MouseButton::Right) => {
                        Some(PressEventData::Mouse(m))
                    }
                    _ => None,
                });
                if let Some(event) = event {
                    on_pointer_press.call(event);
                }
            }
        })
    }

    /// Gets triggered when:
    /// - **Click**: There is a `MouseUp` event (Any button) with the in the same element that there had been a `MouseDown` just before
    /// - **Touched**: There is a `TouchEnd` event in the same element that there had been a `TouchStart` just before
    /// - **Activated**: The element is focused and there is a keydown event pressing the OS activation key (e.g Space, Enter)
    fn on_all_press(self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        let on_press = on_press.into();
        self.on_pointer_press({
            let on_press = on_press.clone();
            move |e: Event<PointerEventData>| {
                let event = e.try_map(|d| match d {
                    PointerEventData::Mouse(m) => Some(PressEventData::Mouse(m)),
                    PointerEventData::Touch(t) => Some(PressEventData::Touch(t)),
                });
                if let Some(event) = event {
                    on_press.call(event);
                }
            }
        })
        .on_key_down({
            let on_press = on_press.clone();
            move |e: Event<KeyboardEventData>| {
                if Focus::is_pressed(&e) {
                    on_press.call(e.map(PressEventData::Keyboard))
                }
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PressEventData {
    Mouse(MouseEventData),
    Keyboard(KeyboardEventData),
    Touch(TouchEventData),
}

pub trait ContainerWithContentExt
where
    Self: LayoutExt,
{
    fn direction(mut self, direction: Direction) -> Self {
        self.get_layout().layout.direction = direction;
        self
    }
    fn main_align(mut self, main_align: Alignment) -> Self {
        self.get_layout().layout.main_alignment = main_align;
        self
    }

    fn cross_align(mut self, cross_align: Alignment) -> Self {
        self.get_layout().layout.cross_alignment = cross_align;
        self
    }

    fn spacing(mut self, spacing: impl Into<f32>) -> Self {
        self.get_layout().layout.spacing = Length::new(spacing.into());
        self
    }

    fn content(mut self, content: Content) -> Self {
        self.get_layout().layout.content = content;
        self
    }
    fn center(mut self) -> Self {
        self.get_layout().layout.main_alignment = Alignment::Center;
        self.get_layout().layout.cross_alignment = Alignment::Center;

        self
    }

    fn offset_x(mut self, offset_x: impl Into<f32>) -> Self {
        self.get_layout().layout.offset_x = Length::new(offset_x.into());
        self
    }

    fn offset_y(mut self, offset_y: impl Into<f32>) -> Self {
        self.get_layout().layout.offset_y = Length::new(offset_y.into());
        self
    }

    fn vertical(mut self) -> Self {
        self.get_layout().layout.direction = Direction::vertical();
        self
    }

    fn horizontal(mut self) -> Self {
        self.get_layout().layout.direction = Direction::horizontal();
        self
    }
}

pub trait ContainerSizeExt
where
    Self: LayoutExt,
{
    fn width(mut self, width: impl Into<Size>) -> Self {
        self.get_layout().layout.width = width.into();
        self
    }

    fn height(mut self, height: impl Into<Size>) -> Self {
        self.get_layout().layout.height = height.into();
        self
    }

    /// Expand both `width` and `height` using [Size::fill()].
    fn expanded(mut self) -> Self {
        self.get_layout().layout.width = Size::fill();
        self.get_layout().layout.height = Size::fill();
        self
    }
}

impl<T: ContainerExt> ContainerSizeExt for T {}

pub trait ContainerExt
where
    Self: LayoutExt,
{
    fn position(mut self, position: impl Into<Position>) -> Self {
        self.get_layout().layout.position = position.into();
        self
    }

    fn padding(mut self, padding: impl Into<Gaps>) -> Self {
        self.get_layout().layout.padding = padding.into();
        self
    }

    fn margin(mut self, margin: impl Into<Gaps>) -> Self {
        self.get_layout().layout.margin = margin.into();
        self
    }

    fn min_width(mut self, minimum_width: impl Into<Size>) -> Self {
        self.get_layout().layout.minimum_width = minimum_width.into();
        self
    }

    fn min_height(mut self, minimum_height: impl Into<Size>) -> Self {
        self.get_layout().layout.minimum_height = minimum_height.into();
        self
    }

    fn max_width(mut self, maximum_width: impl Into<Size>) -> Self {
        self.get_layout().layout.maximum_width = maximum_width.into();
        self
    }

    fn max_height(mut self, maximum_height: impl Into<Size>) -> Self {
        self.get_layout().layout.maximum_height = maximum_height.into();
        self
    }

    fn visible_width(mut self, visible_width: impl Into<VisibleSize>) -> Self {
        self.get_layout().layout.visible_width = visible_width.into();
        self
    }

    fn visible_height(mut self, visible_height: impl Into<VisibleSize>) -> Self {
        self.get_layout().layout.visible_height = visible_height.into();
        self
    }
}

pub trait LayoutExt
where
    Self: Sized,
{
    fn get_layout(&mut self) -> &mut LayoutData;

    fn layout(mut self, layout: LayoutData) -> Self {
        *self.get_layout() = layout;
        self
    }
}

pub trait ImageExt
where
    Self: LayoutExt,
{
    fn width(mut self, width: Size) -> Self {
        self.get_layout().layout.width = width;
        self
    }

    fn height(mut self, height: Size) -> Self {
        self.get_layout().layout.height = height;
        self
    }

    fn get_image_data(&mut self) -> &mut ImageData;

    fn image_data(mut self, image_data: ImageData) -> Self {
        *self.get_image_data() = image_data;
        self
    }

    fn sampling_mode(mut self, sampling_mode: SamplingMode) -> Self {
        self.get_image_data().sampling_mode = sampling_mode;
        self
    }

    fn aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.get_image_data().aspect_ratio = aspect_ratio;
        self
    }

    fn image_cover(mut self, image_cover: ImageCover) -> Self {
        self.get_image_data().image_cover = image_cover;
        self
    }
}

pub trait AccessibilityExt: Sized {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData;

    fn accessibility(mut self, accessibility: AccessibilityData) -> Self {
        *self.get_accessibility_data() = accessibility;
        self
    }

    fn a11y_id(mut self, a11y_id: impl Into<Option<AccessibilityId>>) -> Self {
        self.get_accessibility_data().a11y_id = a11y_id.into();
        self
    }

    fn a11y_focusable(mut self, a11y_focusable: impl Into<Focusable>) -> Self {
        self.get_accessibility_data().a11y_focusable = a11y_focusable.into();
        self
    }

    fn a11y_auto_focus(mut self, a11y_auto_focus: impl Into<bool>) -> Self {
        self.get_accessibility_data().a11y_auto_focus = a11y_auto_focus.into();
        self
    }

    fn a11y_member_of(mut self, a11y_member_of: impl Into<AccessibilityId>) -> Self {
        self.get_accessibility_data()
            .builder
            .set_member_of(a11y_member_of.into());
        self
    }

    fn a11y_role(mut self, a11y_role: impl Into<AccessibilityRole>) -> Self {
        self.get_accessibility_data()
            .builder
            .set_role(a11y_role.into());
        self
    }

    fn a11y_alt(mut self, value: impl Into<Box<str>>) -> Self {
        self.get_accessibility_data().builder.set_label(value);
        self
    }

    fn a11y_builder(mut self, with: impl FnOnce(&mut accesskit::Node)) -> Self {
        with(&mut self.get_accessibility_data().builder);
        self
    }
}

pub trait TextStyleExt
where
    Self: Sized,
{
    fn get_text_style_data(&mut self) -> &mut TextStyleData;

    fn color(mut self, color: impl Into<Color>) -> Self {
        self.get_text_style_data().color = Some(color.into());
        self
    }

    fn text_align(mut self, text_align: impl Into<TextAlign>) -> Self {
        self.get_text_style_data().text_align = Some(text_align.into());
        self
    }

    fn font_size(mut self, font_size: impl Into<FontSize>) -> Self {
        self.get_text_style_data().font_size = Some(font_size.into());
        self
    }

    fn font_family(mut self, font_family: impl Into<Cow<'static, str>>) -> Self {
        self.get_text_style_data()
            .font_families
            .push(font_family.into());
        self
    }

    fn font_slant(mut self, font_slant: impl Into<FontSlant>) -> Self {
        self.get_text_style_data().font_slant = Some(font_slant.into());
        self
    }

    fn font_weight(mut self, font_weight: impl Into<FontWeight>) -> Self {
        self.get_text_style_data().font_weight = Some(font_weight.into());
        self
    }

    fn font_width(mut self, font_width: impl Into<FontWidth>) -> Self {
        self.get_text_style_data().font_width = Some(font_width.into());
        self
    }

    fn text_height(mut self, text_height: impl Into<TextHeightBehavior>) -> Self {
        self.get_text_style_data().text_height = Some(text_height.into());
        self
    }

    fn text_overflow(mut self, text_overflow: impl Into<TextOverflow>) -> Self {
        self.get_text_style_data().text_overflow = Some(text_overflow.into());
        self
    }

    fn text_shadow(mut self, text_shadow: impl Into<TextShadow>) -> Self {
        self.get_text_style_data()
            .text_shadows
            .push(text_shadow.into());
        self
    }
}

pub trait StyleExt
where
    Self: Sized,
{
    fn get_style(&mut self) -> &mut StyleState;

    fn background<S: Into<Color>>(mut self, background: S) -> Self {
        self.get_style().background = Fill::Color(background.into());
        self
    }

    fn background_conic_gradient<S: Into<ConicGradient>>(mut self, background: S) -> Self {
        self.get_style().background = Fill::ConicGradient(Box::new(background.into()));
        self
    }

    fn background_linear_gradient<S: Into<LinearGradient>>(mut self, background: S) -> Self {
        self.get_style().background = Fill::LinearGradient(Box::new(background.into()));
        self
    }

    fn background_radial_gradient<S: Into<RadialGradient>>(mut self, background: S) -> Self {
        self.get_style().background = Fill::RadialGradient(Box::new(background.into()));
        self
    }

    fn border(mut self, border: impl Into<Option<Border>>) -> Self {
        if let Some(border) = border.into() {
            self.get_style().borders.push(border);
        }
        self
    }

    fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.get_style().shadows.push(shadow.into());
        self
    }

    fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.get_style().corner_radius = corner_radius.into();
        self
    }
}

impl<T: StyleExt> CornerRadiusExt for T {
    fn with_corner_radius(mut self, corner_radius: f32) -> Self {
        self.get_style().corner_radius = CornerRadius::new_all(corner_radius);
        self
    }
}

pub trait CornerRadiusExt: Sized {
    fn with_corner_radius(self, corner_radius: f32) -> Self;

    /// Shortcut for `corner_radius(0.)` - removes border radius.
    fn rounded_none(self) -> Self {
        self.with_corner_radius(0.)
    }

    /// Shortcut for `corner_radius(6.)` - default border radius.
    fn rounded(self) -> Self {
        self.with_corner_radius(6.)
    }

    /// Shortcut for `corner_radius(4.)` - small border radius.
    fn rounded_sm(self) -> Self {
        self.with_corner_radius(4.)
    }

    /// Shortcut for `corner_radius(6.)` - medium border radius.
    fn rounded_md(self) -> Self {
        self.with_corner_radius(6.)
    }

    /// Shortcut for `corner_radius(8.)` - large border radius.
    fn rounded_lg(self) -> Self {
        self.with_corner_radius(8.)
    }

    /// Shortcut for `corner_radius(12.)` - extra large border radius.
    fn rounded_xl(self) -> Self {
        self.with_corner_radius(12.)
    }

    /// Shortcut for `corner_radius(16.)` - extra large border radius.
    fn rounded_2xl(self) -> Self {
        self.with_corner_radius(16.)
    }

    /// Shortcut for `corner_radius(24.)` - extra large border radius.
    fn rounded_3xl(self) -> Self {
        self.with_corner_radius(24.)
    }

    /// Shortcut for `corner_radius(32.)` - extra large border radius.
    fn rounded_4xl(self) -> Self {
        self.with_corner_radius(32.)
    }

    /// Shortcut for `corner_radius(99.)` - fully rounded (pill shape).
    fn rounded_full(self) -> Self {
        self.with_corner_radius(99.)
    }
}

pub trait MaybeExt
where
    Self: Sized,
{
    /// Imperatively modify self with the given closure.
    fn map<U>(self, f: impl FnOnce(Self) -> U) -> U
    {
        f(self)
    }

    /// Conditionally modify self with the given closure.
    fn maybe(self, condition: impl Into<bool>, then: impl FnOnce(Self) -> Self) -> Self {
        if condition.into() { then(self) } else { self }
    }

    /// Conditionally modify self with the given closure.
    fn maybe_else(self, condition: impl Into<bool>, then: impl FnOnce(Self) -> Self, else_fn: impl FnOnce(Self) -> Self) -> Self {
        if condition.into() { then(self) } else { else_fn(self) }
    }

    /// Conditionally unwrap and modify self with the given closure, if the given option is Some.
    fn maybe_some<T>(self, option: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self {
        if let Some(value) = option {
            then(self, value)
        } else {
            self
        }
    }

    /// Conditionally unwrap and modify self with the given closure, if the given option is None.
    fn maybe_none<T>(self, option: &Option<T>, then: impl FnOnce(Self) -> Self) -> Self {
        if option.is_none() { then(self) } else { self }
    }
}

pub trait LayerExt
where
    Self: Sized,
{
    fn get_layer(&mut self) -> &mut Layer;

    fn layer(mut self, layer: impl Into<Layer>) -> Self {
        *self.get_layer() = layer.into();
        self
    }
}

pub trait ScrollableExt
where
    Self: Sized,
{
    fn get_effect(&mut self) -> &mut EffectData;

    fn scrollable(mut self, scrollable: impl Into<bool>) -> Self {
        self.get_effect().scrollable = scrollable.into();
        self
    }
}

pub trait InteractiveExt
where
    Self: Sized,
{
    fn get_effect(&mut self) -> &mut EffectData;

    fn interactive(mut self, interactive: impl Into<Interactive>) -> Self {
        self.get_effect().interactive = interactive.into();
        self
    }
}
