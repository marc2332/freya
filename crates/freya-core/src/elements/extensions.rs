use std::{
    borrow::Cow,
    hash::{
        Hash,
        Hasher,
    },
};

use paste::paste;
use ragnarok::CursorPoint;
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
    size::Size,
};

use crate::{
    data::{
        AccessibilityData,
        EffectData,
        LayoutData,
        Overflow,
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
        scale::Scale,
        shader::ShaderFill,
        text_height::TextHeightBehavior,
        text_overflow::TextOverflow,
        text_shadow::TextShadow,
        transform_origin::TransformOrigin,
    },
};

/// Trait for composing child elements.
pub trait ChildrenExt: Sized {
    /// Returns a mutable reference to the internal children vector.
    ///
    /// # Example
    /// ```ignore
    /// impl ChildrenExt for MyElement {
    ///     fn get_children(&mut self) -> &mut Vec<Element> {
    ///         &mut self.elements
    ///     }
    /// }
    /// ```
    fn get_children(&mut self) -> &mut Vec<Element>;

    /// Extends the children with an iterable of [`Element`]s.
    ///
    /// # Example
    /// ```ignore
    /// rect().children(["Hello", "World"].map(|t| label().text(t).into_element()))
    /// ```
    fn children(mut self, children: impl IntoIterator<Item = Element>) -> Self {
        self.get_children().extend(children);
        self
    }

    /// Appends a child only when the [`Option`] is [`Some`].
    ///
    /// # Example
    /// ```ignore
    /// rect().maybe_child(show_badge.then(|| label().text("New")))
    /// ```
    fn maybe_child<C: IntoElement>(mut self, child: Option<C>) -> Self {
        if let Some(child) = child {
            self.get_children().push(child.into_element());
        }
        self
    }

    /// Appends a single child element.
    ///
    /// # Example
    /// ```ignore
    /// rect().child(label().text("Hello"))
    /// ```
    fn child<C: IntoElement>(mut self, child: C) -> Self {
        self.get_children().push(child.into_element());
        self
    }
}

/// Trait for giving an element a stable identity across renders.
pub trait KeyExt: Sized {
    /// Returns a mutable reference to the element's diff key.
    fn write_key(&mut self) -> &mut DiffKey;

    /// Assign a key derived from any hashable value, used to reconcile elements in dynamic lists.
    fn key(mut self, key: impl Hash) -> Self {
        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        *self.write_key() = DiffKey::U64(hasher.finish());
        self
    }
}

/// Trait for concatenating two lists into one.
pub trait ListExt {
    /// Append the contents of `other`, returning the combined list.
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
            $(#[$attr:meta])*
            $name:ident => $event_variant:expr ;
        )*
    ) => {
        paste! {
            $(
                $(#[$attr])*
                fn [<on_$name>](mut self, [<on_$name>]: impl Into<EventHandler<Event<$event_data>>>) -> Self {
                    self.get_event_handlers()
                        .insert($event_variant, EventHandlerType::$handler_variant([<on_$name>].into()));
                    self
                }
            )*
        }
    };
}

/// Methods for attaching event handlers to an element.
///
/// Many events come in three flavors: the plain one fires only while the pointer is over the
/// element; the `global_` variants fire no matter where the event happens; and the `capture_`
/// variants fire during the top-down capture phase, before the event reaches the inner element.
///
/// For high-level press handling, prefer [`on_press`](EventHandlersExt::on_press) over the raw mouse/pointer events.
pub trait EventHandlersExt: Sized {
    /// Returns a mutable reference to the element's event handler map.
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType>;

    /// Replace all of this element's event handlers with the given map.
    fn with_event_handlers(
        mut self,
        event_handlers: FxHashMap<EventName, EventHandlerType>,
    ) -> Self {
        *self.get_event_handlers() = event_handlers;
        self
    }

    event_handlers! {
        Mouse,
        MouseEventData;

        /// Fires when a mouse button is pressed down over the element.
        mouse_down => EventName::MouseDown;
        /// Fires when a mouse button is released over the element.
        mouse_up => EventName::MouseUp;
        /// Fires when the cursor moves over the element.
        mouse_move => EventName::MouseMove;

    }

    event_handlers! {
        Pointer,
        PointerEventData;

        /// Fires when a pointer (mouse or touch) is pressed anywhere, even outside the element.
        global_pointer_press => EventName::GlobalPointerPress;
        /// Fires when a pointer (mouse or touch) goes down anywhere, even outside the element.
        global_pointer_down => EventName::GlobalPointerDown;
        /// Fires when a pointer (mouse or touch) moves anywhere, even outside the element.
        global_pointer_move => EventName::GlobalPointerMove;

        /// Like [`on_global_pointer_move`](Self::on_global_pointer_move), but fires during the top-down capture phase.
        capture_global_pointer_move => EventName::CaptureGlobalPointerMove;
        /// Like [`on_global_pointer_press`](Self::on_global_pointer_press), but fires during the top-down capture phase.
        capture_global_pointer_press => EventName::CaptureGlobalPointerPress;
    }

    event_handlers! {
        Keyboard,
        KeyboardEventData;

        /// Fires when a key is pressed down while the element is focused.
        key_down => EventName::KeyDown;
        /// Fires when a key is released while the element is focused.
        key_up => EventName::KeyUp;

        /// Fires when a key is pressed down, regardless of which element is focused.
        global_key_down => EventName::GlobalKeyDown;
        /// Fires when a key is released, regardless of which element is focused.
        global_key_up => EventName::GlobalKeyUp;
    }

    event_handlers! {
        Wheel,
        WheelEventData;

        /// Fires when the scroll wheel is used over the element.
        wheel => EventName::Wheel;
    }

    event_handlers! {
        Touch,
        TouchEventData;

        /// Fires when an ongoing touch is cancelled by the system.
        touch_cancel => EventName::TouchCancel;
        /// Fires when a touch point is placed on the element.
        touch_start => EventName::TouchStart;
        /// Fires when a touch point moves across the element.
        touch_move => EventName::TouchMove;
        /// Fires when a touch point is lifted from the element.
        touch_end => EventName::TouchEnd;
    }

    event_handlers! {
        Pointer,
        PointerEventData;

        /// Fires when the element is pressed and released by a pointer (mouse or touch).
        pointer_press => EventName::PointerPress;
        /// Fires when a pointer (mouse or touch) goes down over the element.
        pointer_down => EventName::PointerDown;
        /// Fires when a pointer (mouse or touch) moves over the element.
        pointer_move => EventName::PointerMove;
        /// Fires when a pointer enters the element.
        pointer_enter => EventName::PointerEnter;
        /// Fires when a pointer leaves the element.
        pointer_leave => EventName::PointerLeave;
        /// Fires when a pointer is over the element, including over its children.
        pointer_over => EventName::PointerOver;
        /// Fires when a pointer leaves the element or one of its children.
        pointer_out => EventName::PointerOut;
    }

    event_handlers! {
        File,
        FileEventData;

        /// Fires when a file is dropped onto the element.
        file_drop => EventName::FileDrop;
        /// Fires when a dragged file hovers anywhere over the window.
        global_file_hover => EventName::GlobalFileHover;
        /// Fires when a dragged file stops hovering over the window.
        global_file_hover_cancelled => EventName::GlobalFileHoverCancelled;
    }

    event_handlers! {
        ImePreedit,
        ImePreeditEventData;

        /// Fires while text is being composed through an input method editor (IME).
        ime_preedit => EventName::ImePreedit;
    }

    /// Fires when the element's measured size or position changes.
    fn on_sized(mut self, on_sized: impl Into<EventHandler<Event<SizedEventData>>>) -> Self
    where
        Self: LayoutExt,
    {
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
        .on_key_down(move |e: Event<KeyboardEventData>| {
            if e.is_press_event() {
                on_press.call(e.map(PressEventData::Keyboard))
            }
        })
    }

    /// Also called the context menu click in other platforms.
    /// Gets triggered when:
    /// - **Click**: There is a `MouseDown` (Right button) event
    fn on_secondary_down(
        self,
        on_secondary_down: impl Into<EventHandler<Event<PressEventData>>>,
    ) -> Self {
        let on_secondary_down = on_secondary_down.into();
        self.on_pointer_down(move |e: Event<PointerEventData>| {
            let event = e.try_map(|d| match d {
                PointerEventData::Mouse(m) if m.button == Some(MouseButton::Right) => {
                    Some(PressEventData::Mouse(m))
                }
                _ => None,
            });
            if let Some(event) = event {
                on_secondary_down.call(event);
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
                let event = e.map(|d| match d {
                    PointerEventData::Mouse(m) => PressEventData::Mouse(m),
                    PointerEventData::Touch(t) => PressEventData::Touch(t),
                });
                on_press.call(event);
            }
        })
        .on_key_down(move |e: Event<KeyboardEventData>| {
            if e.is_press_event() {
                on_press.call(e.map(PressEventData::Keyboard))
            }
        })
    }
    /// Gets triggered when:
    /// - **Started clicking**: There is a `MouseDown` event (Left button)
    /// - **Touched**: There is a `TouchEnd` event in the same element that there had been a `TouchStart` just before
    ///
    /// This event is intended to focus elements such as text inputs following each platform style.
    fn on_focus_press(
        self,
        on_focus_press: impl Into<EventHandler<Event<FocusPressEventData>>>,
    ) -> Self {
        let on_focus_press = on_focus_press.into();
        if cfg!(target_os = "android") {
            self.on_pointer_press(move |e: Event<PointerEventData>| {
                let event = e.try_map(|d| match d {
                    PointerEventData::Mouse(m) if m.button == Some(MouseButton::Left) => {
                        Some(FocusPressEventData::Mouse(m))
                    }
                    PointerEventData::Touch(t) => Some(FocusPressEventData::Touch(t)),
                    _ => None,
                });
                if let Some(event) = event {
                    on_focus_press.call(event);
                }
            })
        } else {
            self.on_pointer_down(move |e: Event<PointerEventData>| {
                let event = e.try_map(|d| match d {
                    PointerEventData::Mouse(m) if m.button == Some(MouseButton::Left) => {
                        Some(FocusPressEventData::Mouse(m))
                    }
                    PointerEventData::Touch(t) => Some(FocusPressEventData::Touch(t)),
                    _ => None,
                });
                if let Some(event) = event {
                    on_focus_press.call(event);
                }
            })
        }
    }
}

/// Data delivered to [`on_focus_press`](EventHandlersExt::on_focus_press), which can originate from a mouse or a touch.
#[derive(Debug, Clone, PartialEq)]
pub enum FocusPressEventData {
    Mouse(MouseEventData),
    Touch(TouchEventData),
}

impl FocusPressEventData {
    pub fn global_location(&self) -> CursorPoint {
        match self {
            Self::Mouse(m) => m.global_location,
            Self::Touch(t) => t.global_location,
        }
    }

    pub fn element_location(&self) -> CursorPoint {
        match self {
            Self::Mouse(m) => m.element_location,
            Self::Touch(t) => t.element_location,
        }
    }

    pub fn button(&self) -> Option<MouseButton> {
        match self {
            Self::Mouse(m) => m.button,
            Self::Touch(_) => None,
        }
    }
}

/// Data delivered to [`on_press`](EventHandlersExt::on_press), which can originate from a mouse, the keyboard or a touch.
#[derive(Debug, Clone, PartialEq)]
pub enum PressEventData {
    Mouse(MouseEventData),
    Keyboard(KeyboardEventData),
    Touch(TouchEventData),
}

/// Layout methods for containers that arrange children along a direction axis.
pub trait ContainerWithContentExt
where
    Self: LayoutExt,
{
    /// Set the axis children are stacked along. See [`Direction`].
    fn direction(mut self, direction: Direction) -> Self {
        self.get_layout().layout.direction = direction;
        self
    }
    /// Set how children are aligned along the direction axis. See [`Alignment`].
    fn main_align(mut self, main_align: Alignment) -> Self {
        self.get_layout().layout.main_alignment = main_align;
        self
    }

    /// Set how children are aligned across the direction axis. See [`Alignment`].
    fn cross_align(mut self, cross_align: Alignment) -> Self {
        self.get_layout().layout.cross_alignment = cross_align;
        self
    }

    /// Set the gap inserted between adjacent children, in pixels.
    fn spacing(mut self, spacing: impl Into<f32>) -> Self {
        self.get_layout().layout.spacing = Length::new(spacing.into());
        self
    }

    /// Set how children share the available space along the direction axis. See [`Content`].
    fn content(mut self, content: Content) -> Self {
        self.get_layout().layout.content = content;
        self
    }
    /// Center children on both axes. Shorthand for [`main_align`](Self::main_align) and [`cross_align`](Self::cross_align) set to [`Alignment::Center`].
    fn center(mut self) -> Self {
        self.get_layout().layout.main_alignment = Alignment::Center;
        self.get_layout().layout.cross_alignment = Alignment::Center;

        self
    }

    /// Shift the element's children horizontally by the given pixels.
    fn offset_x(mut self, offset_x: impl Into<f32>) -> Self {
        self.get_layout().layout.offset_x = Length::new(offset_x.into());
        self
    }

    /// Shift the element's children vertically by the given pixels.
    fn offset_y(mut self, offset_y: impl Into<f32>) -> Self {
        self.get_layout().layout.offset_y = Length::new(offset_y.into());
        self
    }

    /// Stack children vertically. Shorthand for [`direction`](Self::direction) set to [`Direction::Vertical`].
    fn vertical(mut self) -> Self {
        self.get_layout().layout.direction = Direction::vertical();
        self
    }

    /// Stack children horizontally. Shorthand for [`direction`](Self::direction) set to [`Direction::Horizontal`].
    fn horizontal(mut self) -> Self {
        self.get_layout().layout.direction = Direction::horizontal();
        self
    }
}

/// Methods for setting an element's width and height.
pub trait ContainerSizeExt
where
    Self: LayoutExt,
{
    /// Set the element's width. See [`Size`].
    fn width(mut self, width: impl Into<Size>) -> Self {
        self.get_layout().layout.width = width.into();
        self
    }

    /// Set the element's height. See [`Size`].
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

/// Methods controlling an element's position and size constraints.
pub trait ContainerExt
where
    Self: LayoutExt,
{
    /// Set how the element is placed relative to its parent or the window. See [`Position`].
    fn position(mut self, position: impl Into<Position>) -> Self {
        self.get_layout().layout.position = position.into();
        self
    }

    /// Set the inner spacing between the element's edges and its content. See [`Gaps`].
    fn padding(mut self, padding: impl Into<Gaps>) -> Self {
        self.get_layout().layout.padding = padding.into();
        self
    }

    /// Set the outer spacing between the element's edges and its surroundings. See [`Gaps`].
    fn margin(mut self, margin: impl Into<Gaps>) -> Self {
        self.get_layout().layout.margin = margin.into();
        self
    }

    /// Set the minimum width the element can shrink to. See [`Size`].
    fn min_width(mut self, minimum_width: impl Into<Size>) -> Self {
        self.get_layout().layout.minimum_width = minimum_width.into();
        self
    }

    /// Set the minimum height the element can shrink to. See [`Size`].
    fn min_height(mut self, minimum_height: impl Into<Size>) -> Self {
        self.get_layout().layout.minimum_height = minimum_height.into();
        self
    }

    /// Set the maximum width the element can grow to. See [`Size`].
    fn max_width(mut self, maximum_width: impl Into<Size>) -> Self {
        self.get_layout().layout.maximum_width = maximum_width.into();
        self
    }

    /// Set the maximum height the element can grow to. See [`Size`].
    fn max_height(mut self, maximum_height: impl Into<Size>) -> Self {
        self.get_layout().layout.maximum_height = maximum_height.into();
        self
    }

    /// Set how much of the measured width is actually used in layout. See [`VisibleSize`].
    fn visible_width(mut self, visible_width: impl Into<VisibleSize>) -> Self {
        self.get_layout().layout.visible_width = visible_width.into();
        self
    }

    /// Set how much of the measured height is actually used in layout. See [`VisibleSize`].
    fn visible_height(mut self, visible_height: impl Into<VisibleSize>) -> Self {
        self.get_layout().layout.visible_height = visible_height.into();
        self
    }
}

/// Low-level access to an element's [`LayoutData`].
pub trait LayoutExt
where
    Self: Sized,
{
    /// Returns a mutable reference to the element's layout data.
    fn get_layout(&mut self) -> &mut LayoutData;

    /// Replace all of the element's layout data at once. See [`LayoutData`].
    fn layout(mut self, layout: LayoutData) -> Self {
        *self.get_layout() = layout;
        self
    }
}

/// Methods for configuring how an image is scaled and sampled.
pub trait ImageExt
where
    Self: LayoutExt,
{
    /// Returns a mutable reference to the element's image data.
    fn get_image_data(&mut self) -> &mut ImageData;

    /// Replace all of the element's image data at once. See [`ImageData`].
    fn image_data(mut self, image_data: ImageData) -> Self {
        *self.get_image_data() = image_data;
        self
    }

    /// Set the filtering used when the image is scaled. See [`SamplingMode`].
    fn sampling_mode(mut self, sampling_mode: SamplingMode) -> Self {
        self.get_image_data().sampling_mode = sampling_mode;
        self
    }

    /// Set how the image is scaled to fit its bounds. See [`AspectRatio`].
    fn aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.get_image_data().aspect_ratio = aspect_ratio;
        self
    }

    /// Set how the image is positioned within its bounds. See [`ImageCover`].
    fn image_cover(mut self, image_cover: ImageCover) -> Self {
        self.get_image_data().image_cover = image_cover;
        self
    }
}

/// Methods for describing an element in the accessibility tree.
pub trait AccessibilityExt: Sized {
    /// Returns a mutable reference to the element's accessibility data.
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData;

    /// Replace all of the element's accessibility data at once. See [`AccessibilityData`].
    fn accessibility(mut self, accessibility: AccessibilityData) -> Self {
        *self.get_accessibility_data() = accessibility;
        self
    }

    /// Set an explicit accessibility id instead of an autogenerated one. See [`AccessibilityId`].
    fn a11y_id(mut self, a11y_id: impl Into<Option<AccessibilityId>>) -> Self {
        self.get_accessibility_data().a11y_id = a11y_id.into();
        self
    }

    /// Set whether the element can receive keyboard focus. See [`Focusable`].
    fn a11y_focusable(mut self, a11y_focusable: impl Into<Focusable>) -> Self {
        self.get_accessibility_data().a11y_focusable = a11y_focusable.into();
        self
    }

    /// Request that the element be focused automatically when it is mounted.
    fn a11y_auto_focus(mut self, a11y_auto_focus: impl Into<bool>) -> Self {
        self.get_accessibility_data().a11y_auto_focus = a11y_auto_focus.into();
        self
    }

    /// Mark the element as a member of the group identified by the given [`AccessibilityId`].
    fn a11y_member_of(mut self, a11y_member_of: impl Into<AccessibilityId>) -> Self {
        self.get_accessibility_data()
            .builder
            .set_member_of(a11y_member_of.into());
        self
    }

    /// Set the accessibility role exposed in the accessibility tree. See [`AccessibilityRole`].
    fn a11y_role(mut self, a11y_role: impl Into<AccessibilityRole>) -> Self {
        self.get_accessibility_data()
            .builder
            .set_role(a11y_role.into());
        self
    }

    /// Set the text label that describes the element in the accessibility tree.
    fn a11y_alt(mut self, value: impl Into<Box<str>>) -> Self {
        self.get_accessibility_data().builder.set_label(value);
        self
    }

    /// Edit the underlying `accesskit` node directly for advanced accessibility properties.
    fn a11y_builder(mut self, with: impl FnOnce(&mut accesskit::Node)) -> Self {
        with(&mut self.get_accessibility_data().builder);
        self
    }
}

/// Methods for styling the text rendered by an element and inherited by its children.
pub trait TextStyleExt
where
    Self: Sized,
{
    /// Returns a mutable reference to the element's text style data.
    fn get_text_style_data(&mut self) -> &mut TextStyleData;

    /// Replace all of the element's text style data at once. See [`TextStyleData`].
    fn text_style(mut self, data: TextStyleData) -> Self {
        *self.get_text_style_data() = data;
        self
    }

    /// Set the text color to a solid [`Color`].
    fn color(mut self, color: impl Into<Color>) -> Self {
        self.get_text_style_data().color = Some(Fill::Color(color.into()));
        self
    }

    /// Paint the text with a [`ConicGradient`].
    fn color_conic_gradient<S: Into<ConicGradient>>(mut self, color: S) -> Self {
        self.get_text_style_data().color = Some(Fill::ConicGradient(Box::new(color.into())));
        self
    }

    /// Paint the text with a [`LinearGradient`].
    fn color_linear_gradient<S: Into<LinearGradient>>(mut self, color: S) -> Self {
        self.get_text_style_data().color = Some(Fill::LinearGradient(Box::new(color.into())));
        self
    }

    /// Paint the text with a [`RadialGradient`].
    fn color_radial_gradient<S: Into<RadialGradient>>(mut self, color: S) -> Self {
        self.get_text_style_data().color = Some(Fill::RadialGradient(Box::new(color.into())));
        self
    }

    /// Paint the text with a custom shader. See [`ShaderFill`].
    fn color_shader(mut self, color: impl Into<ShaderFill>) -> Self {
        self.get_text_style_data().color = Some(Fill::Shader(Box::new(color.into())));
        self
    }

    /// Set the horizontal alignment of the text. See [`TextAlign`].
    fn text_align(mut self, text_align: impl Into<TextAlign>) -> Self {
        self.get_text_style_data().text_align = Some(text_align.into());
        self
    }

    /// Set the text size in pixels. See [`FontSize`].
    fn font_size(mut self, font_size: impl Into<FontSize>) -> Self {
        self.get_text_style_data().font_size = Some(font_size.into());
        self
    }

    /// Add a font family to try, in order of preference.
    fn font_family(mut self, font_family: impl Into<Cow<'static, str>>) -> Self {
        self.get_text_style_data()
            .font_families
            .push(font_family.into());
        self
    }

    /// Set the slant (style) of the font. See [`FontSlant`].
    fn font_slant(mut self, font_slant: impl Into<FontSlant>) -> Self {
        self.get_text_style_data().font_slant = Some(font_slant.into());
        self
    }

    /// Set the thickness of the font. See [`FontWeight`].
    fn font_weight(mut self, font_weight: impl Into<FontWeight>) -> Self {
        self.get_text_style_data().font_weight = Some(font_weight.into());
        self
    }

    /// Set the horizontal width of the font. See [`FontWidth`].
    fn font_width(mut self, font_width: impl Into<FontWidth>) -> Self {
        self.get_text_style_data().font_width = Some(font_width.into());
        self
    }

    /// Set how the leading of the first and last lines is handled. See [`TextHeightBehavior`].
    fn text_height(mut self, text_height: impl Into<TextHeightBehavior>) -> Self {
        self.get_text_style_data().text_height = Some(text_height.into());
        self
    }

    /// Set how text that does not fit its bounds is truncated. See [`TextOverflow`].
    fn text_overflow(mut self, text_overflow: impl Into<TextOverflow>) -> Self {
        self.get_text_style_data().text_overflow = Some(text_overflow.into());
        self
    }

    /// Add a shadow cast behind the text. See [`TextShadow`].
    fn text_shadow(mut self, text_shadow: impl Into<TextShadow>) -> Self {
        self.get_text_style_data()
            .text_shadows
            .push(text_shadow.into());
        self
    }

    /// Set a line drawn through, under or over the text. See [`TextDecoration`].
    fn text_decoration(mut self, text_decoration: impl Into<TextDecoration>) -> Self {
        self.get_text_style_data().text_decoration = Some(text_decoration.into());
        self
    }
}

/// Methods for styling an element's box: background, borders, shadows and corners.
pub trait StyleExt
where
    Self: Sized,
{
    /// Returns a mutable reference to the element's style data.
    fn get_style(&mut self) -> &mut StyleState;

    /// Set the background to a solid [`Color`].
    fn background<S: Into<Color>>(mut self, background: S) -> Self {
        self.get_style().background = Fill::Color(background.into());
        self
    }

    /// Paint the background with a [`ConicGradient`].
    fn background_conic_gradient<S: Into<ConicGradient>>(mut self, background: S) -> Self {
        self.get_style().background = Fill::ConicGradient(Box::new(background.into()));
        self
    }

    /// Paint the background with a [`LinearGradient`].
    fn background_linear_gradient<S: Into<LinearGradient>>(mut self, background: S) -> Self {
        self.get_style().background = Fill::LinearGradient(Box::new(background.into()));
        self
    }

    /// Paint the background with a [`RadialGradient`].
    fn background_radial_gradient<S: Into<RadialGradient>>(mut self, background: S) -> Self {
        self.get_style().background = Fill::RadialGradient(Box::new(background.into()));
        self
    }

    /// Paint the background with a custom shader. See [`ShaderFill`].
    fn background_shader(mut self, background: impl Into<ShaderFill>) -> Self {
        self.get_style().background = Fill::Shader(Box::new(background.into()));
        self
    }

    /// Add an outline around the element. See [`Border`].
    fn border(mut self, border: impl Into<Option<Border>>) -> Self {
        if let Some(border) = border.into() {
            self.get_style().borders.push(border);
        }
        self
    }

    /// Add a shadow cast by the element. See [`Shadow`].
    fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.get_style().shadows.push(shadow.into());
        self
    }

    /// Round the element's corners. See [`CornerRadius`].
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

/// Shorthand methods for setting an element's [`CornerRadius`] to common values.
pub trait CornerRadiusExt: Sized {
    /// Round all four corners to the given radius in pixels.
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

/// Methods for applying changes to an element conditionally.
pub trait MaybeExt
where
    Self: Sized,
{
    /// Apply `then` to the element only when the condition is `true`.
    fn maybe(self, bool: impl Into<bool>, then: impl FnOnce(Self) -> Self) -> Self {
        if bool.into() { then(self) } else { self }
    }

    /// Apply `then` to the element only when the [`Option`] is [`Some`], passing the inner value.
    fn map<T>(self, data: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self {
        if let Some(data) = data {
            then(self, data)
        } else {
            self
        }
    }
}

/// Method for controlling which painting layer an element belongs to.
pub trait LayerExt
where
    Self: Sized,
{
    /// Returns a mutable reference to the element's layer.
    fn get_layer(&mut self) -> &mut Layer;

    /// Set the painting layer of the element. See [`Layer`].
    fn layer(mut self, layer: impl Into<Layer>) -> Self {
        *self.get_layer() = layer.into();
        self
    }
}

pub trait ScrollableExt
where
    Self: Sized,
{
    /// Returns a mutable reference to the element's effect data.
    fn get_effect(&mut self) -> &mut EffectData;

    /// Mark this element as scrollable.
    /// You are probably looking for the `ScrollView` component instead.
    fn scrollable(mut self, scrollable: impl Into<bool>) -> Self {
        self.get_effect().scrollable = scrollable.into();
        self
    }
}

/// Method for controlling whether an element responds to pointer events.
pub trait InteractiveExt
where
    Self: Sized,
{
    /// Returns a mutable reference to the element's effect data.
    fn get_effect(&mut self) -> &mut EffectData;

    /// Set whether the element receives pointer events. See [`Interactive`].
    fn interactive(mut self, interactive: impl Into<Interactive>) -> Self {
        self.get_effect().interactive = interactive.into();
        self
    }
}

/// Methods for visual effects applied to an element: clipping, blur, rotation, opacity and scale.
pub trait EffectExt: Sized {
    /// Returns a mutable reference to the element's effect data.
    fn get_effect(&mut self) -> &mut EffectData;

    /// Replace all of the element's effect data at once. See [`EffectData`].
    fn effect(mut self, effect: EffectData) -> Self {
        *self.get_effect() = effect;
        self
    }

    /// Set whether content overflowing the element's bounds is clipped. See [`Overflow`].
    fn overflow(mut self, overflow: impl Into<Overflow>) -> Self {
        self.get_effect().overflow = overflow.into();
        self
    }

    /// Apply a gaussian blur of the given radius to the element.
    fn blur(mut self, blur: impl Into<f32>) -> Self {
        self.get_effect().blur = Some(blur.into());
        self
    }

    /// Rotate the element by the given angle in degrees.
    fn rotation(mut self, rotation: impl Into<f32>) -> Self {
        self.get_effect().rotation = Some(rotation.into());
        self
    }

    /// Set the element's opacity, from `0.0` (transparent) to `1.0` (opaque).
    fn opacity(mut self, opacity: impl Into<f32>) -> Self {
        self.get_effect().opacity = Some(opacity.into());
        self
    }

    /// Scale the element. See [`Scale`].
    fn scale(mut self, scale: impl Into<Scale>) -> Self {
        self.get_effect().scale = Some(scale.into());
        self
    }

    /// Set the point that the scale and rotation effects pivot around.
    ///
    /// Defaults to the element's center.
    fn transform_origin(mut self, transform_origin: impl Into<TransformOrigin>) -> Self {
        self.get_effect().transform_origin = transform_origin.into();
        self
    }
}
