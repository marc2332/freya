use dioxus_rsx::HotReloadingContext;

pub use events::*;

#[doc(hidden)]
pub type AttributeDescription = (&'static str, Option<&'static str>, bool);

macro_rules! impl_element_match {
    (
        $el:ident $name:ident None {
            $(
                $fil:ident: $vil:ident,
            )*
        }
    ) => {
        if $el == stringify!($name) {
            return Some((stringify!($name), None));
        }
    };
}

macro_rules! impl_attribute_match {
    (
        $attr:ident $fil:ident: $vil:ident,
    ) => {
        if $attr == stringify!($fil) {
            return Some((stringify!($fil), None));
        }
    };
}

macro_rules! impl_element_match_attributes {
    (
        $el:ident $attr:ident $name:ident None {
            $(
                $fil:ident: $vil:ident,
            )*
        }
    ) => {
        if $el == stringify!($name) {
            $(
                impl_attribute_match!(
                    $attr $fil: $vil,
                );
            )*
        }
    };

    (
        $el:ident $attr:ident $name:ident  {
            $(
                $fil:ident: $vil:ident,
            )*
        }
    ) => {
        if $el == stringify!($name) {
            $(
                impl_attribute_match!(
                    $attr $fil: $vil,
                );
            )*
        }
    }
}

macro_rules! builder_constructors {
    (
        $(
            $(#[$attr:meta])*
            $name:ident {
                $(
                    $(#[$attr_method:meta])*
                    $fil:ident: $vil:ident,
                )*
            };
         )*
        ) => {
        #[doc(hidden)]
        pub struct FreyaCtx;

        impl HotReloadingContext for FreyaCtx {
            fn map_attribute(element: &str, attribute: &str) -> Option<(&'static str, Option<&'static str>)> {
                $(
                    impl_element_match_attributes!(
                        element attribute $name {
                            $(
                                $fil: $vil,
                            )*
                        }
                    );
                )*
               None
            }

            fn map_element(element: &str) -> Option<(&'static str, Option<&'static str>)> {
                $(
                    impl_element_match!(
                        element $name None {
                            $(
                                $fil: $vil,
                            )*
                        }
                    );
                )*
                None
            }
        }

        $(
            impl_element!(
                $(#[$attr])*
                $name {
                    $(
                        $(#[$attr_method])*
                        $fil: $vil,
                    )*
                };
            );
        )*
    };
}

macro_rules! impl_element {
    (
        $(
            $(#[$attr:meta])*
            $name:ident {
                $(
                    $(#[$attr_method:meta])*
                    $fil:ident: $vil:ident,
                )*
            };
         )*
    ) => {
        $(
            #[allow(non_camel_case_types)]
            $(#[$attr])*
            pub struct $name;

            impl $name {
                #[doc(hidden)]
                pub const TAG_NAME: &'static str = stringify!($name);
                #[doc(hidden)]
                pub const NAME_SPACE: Option<&'static str> = None;

                $(
                    #[allow(non_upper_case_globals)]
                    $(#[$attr_method])*
                    pub const $fil: AttributeDescription = (stringify!($fil), None, false);
                )*
            }
        )*
    };
}

builder_constructors! {
    /// `rect` is a generic element that acts as a container for other elements.
    ///
    /// You can specify things like [`width`](#width-and-height), [`padding`](#padding) or even in what [`direction`](#direction) the inner elements are stacked.
    ///
    /// ### Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             direction: "vertical",
    ///             label { "Hi!" }
    ///             label { "Hi again!"}
    ///         }
    ///     )
    /// }
    /// ```
    rect {
        #[doc = include_str!("_docs/attributes/padding.md")]
        padding: String,
        #[doc = include_str!("_docs/attributes/width_height.md")]
        height: String,
        width: String,
        #[doc = include_str!("_docs/attributes/min_width_min_height.md")]
        min_height: String,
        min_width: String,
        #[doc = include_str!("_docs/attributes/max_width_max_height.md")]
        max_height: String,
        max_width: String,
        #[doc = include_str!("_docs/attributes/background.md")]
        background: String,
        #[doc = include_str!("_docs/attributes/border.md")]
        border: String,
        border_align: String,
        #[doc = include_str!("_docs/attributes/direction.md")]
        direction: String,
        #[doc = include_str!("_docs/attributes/shadow.md")]
        shadow: String,
        #[doc = include_str!("_docs/attributes/corner.md")]
        corner_radius: String,
        corner_smoothing: String,
        #[doc = include_str!("_docs/attributes/color.md")]
        color: String,
        #[doc = include_str!("_docs/attributes/font_size.md")]
        font_size: String,
        #[doc = include_str!("_docs/attributes/font_family.md")]
        font_family: String,
        #[doc = include_str!("_docs/attributes/font_style.md")]
        font_style: String,
        #[doc = include_str!("_docs/attributes/font_weight.md")]
        font_weight: String,
        #[doc = include_str!("_docs/attributes/font_width.md")]
        font_width: String,
        #[doc = include_str!("_docs/attributes/main_align_cross_align.md")]
        main_align: String,
        cross_align: String,
        #[doc = include_str!("_docs/attributes/text_align.md")]
        text_align: String,
        #[doc = include_str!("_docs/attributes/rotate.md")]
        rotate: String,
        #[doc = include_str!("_docs/attributes/overflow.md")]
        overflow: String,
        #[doc = include_str!("_docs/attributes/margin.md")]
        margin: String,
        #[doc = include_str!("_docs/attributes/position.md")]
        position: String,
        position_top: String,
        position_right: String,
        position_bottom: String,
        position_left: String,
        #[doc = include_str!("_docs/attributes/opacity.md")]
        opacity: String,
        #[doc = include_str!("_docs/attributes/content.md")]
        content: String,

        name: String,
        focusable: String,
        role: String,
        focus_id: AccessibilityId,
        alt: String,
        canvas_reference: String,
        layer: String,
        offset_y: String,
        offset_x: String,
        reference: Reference,
        cursor_reference: CursorReference,
    };
    /// `label` simply let's you display some text.
    ///
    /// ### Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             "Hello World"
    ///         }
    ///     )
    /// }
    /// ```
    label {
        #[doc = include_str!("_docs/attributes/color.md")]
        color: String,
        #[doc = include_str!("_docs/attributes/text_shadow.md")]
        text_shadow: String,
        #[doc = include_str!("_docs/attributes/width_height.md")]
        height: String,
        width: String,
        #[doc = include_str!("_docs/attributes/font_size.md")]
        font_size: String,
        #[doc = include_str!("_docs/attributes/font_family.md")]
        font_family: String,
        #[doc = include_str!("_docs/attributes/font_style.md")]
        font_style: String,
        #[doc = include_str!("_docs/attributes/font_weight.md")]
        font_weight: String,
        #[doc = include_str!("_docs/attributes/font_width.md")]
        font_width: String,
        #[doc = include_str!("_docs/attributes/text_align.md")]
        text_align: String,
        #[doc = include_str!("_docs/attributes/max_lines.md")]
        max_lines: String,
        #[doc = include_str!("_docs/attributes/rotate.md")]
        rotate: String,
        #[doc = include_str!("_docs/attributes/line_height.md")]
        line_height: String,
        #[doc = include_str!("_docs/attributes/letter_spacing.md")]
        letter_spacing: String,
        #[doc = include_str!("_docs/attributes/word_spacing.md")]
        word_spacing: String,
        #[doc = include_str!("_docs/attributes/decoration.md")]
        decoration: String,
        #[doc = include_str!("_docs/attributes/decoration_style.md")]
        decoration_style: String,
        #[doc = include_str!("_docs/attributes/decoration_color.md")]
        decoration_color: String,
        #[doc = include_str!("_docs/attributes/text_overflow.md")]
        text_overflow: String,
        focusable: String,
        #[doc = include_str!("_docs/attributes/margin.md")]
        margin: String,
        #[doc = include_str!("_docs/attributes/opacity.md")]
        opacity: String,

        layer: String,
        role: String,
        alt: String,
        focus_id: AccessibilityId,
        name: String,
    };
    /// `paragraph` element let's you build texts with different styles.
    ///
    /// This used used with the `text` element.
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         paragraph {
    ///             text {
    ///                 font_size: "15",
    ///                 "Hello, "
    ///             }
    ///             text {
    ///                 font_size: "30",
    ///                 "World!"
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    paragraph {
        #[doc = include_str!("_docs/attributes/width_height.md")]
        height: String,
        width: String,
        #[doc = include_str!("_docs/attributes/min_width_min_height.md")]
        min_height: String,
        min_width: String,
        #[doc = include_str!("_docs/attributes/max_width_max_height.md")]
        max_height: String,
        max_width: String,
        #[doc = include_str!("_docs/attributes/text_align.md")]
        text_align: String,
        direction: String,
        #[doc = include_str!("_docs/attributes/rotate.md")]
        rotate: String,
        #[doc = include_str!("_docs/attributes/font_size.md")]
        font_size: String,
        #[doc = include_str!("_docs/attributes/font_family.md")]
        font_family: String,
        #[doc = include_str!("_docs/attributes/font_style.md")]
        font_style: String,
        #[doc = include_str!("_docs/attributes/font_weight.md")]
        font_weight: String,
        #[doc = include_str!("_docs/attributes/font_width.md")]
        font_width: String,
        #[doc = include_str!("_docs/attributes/line_height.md")]
        line_height: String,
        #[doc = include_str!("_docs/attributes/letter_spacing.md")]
        letter_spacing: String,
        #[doc = include_str!("_docs/attributes/word_spacing.md")]
        word_spacing: String,
        #[doc = include_str!("_docs/attributes/decoration.md")]
        decoration: String,
        #[doc = include_str!("_docs/attributes/decoration_style.md")]
        decoration_style: String,
        #[doc = include_str!("_docs/attributes/decoration_color.md")]
        text_overflow: String,
        #[doc = include_str!("_docs/attributes/overflow.md")]
        overflow: String,
        focusable: String,
        #[doc = include_str!("_docs/attributes/margin.md")]
        margin: String,
        #[doc = include_str!("_docs/attributes/opacity.md")]
        opacity: String,

        layer: String,
        cursor_index: String,
        max_lines: String,
        cursor_color: String,
        cursor_mode: String,
        cursor_id: String,
        alt: String,
        name: String,
        role: String,
        focus_id: AccessibilityId,
        highlights: String,
        highlight_color: String,
    };
    /// `text` element is simply a text span used for the `paragraph` element.
    text {
        #[doc = include_str!("_docs/attributes/color.md")]
        color: String,
        #[doc = include_str!("_docs/attributes/font_size.md")]
        text_shadow: String,
        #[doc = include_str!("_docs/attributes/width_height.md")]
        height: String,
        width: String,
        #[doc = include_str!("_docs/attributes/font_size.md")]
        font_size: String,
        #[doc = include_str!("_docs/attributes/font_family.md")]
        font_family: String,
        #[doc = include_str!("_docs/attributes/font_style.md")]
        font_style: String,
        #[doc = include_str!("_docs/attributes/font_weight.md")]
        font_weight: String,
        #[doc = include_str!("_docs/attributes/font_width.md")]
        font_width: String,
        #[doc = include_str!("_docs/attributes/line_height.md")]
        line_height: String,
        #[doc = include_str!("_docs/attributes/letter_spacing.md")]
        letter_spacing: String,
        #[doc = include_str!("_docs/attributes/word_spacing.md")]
        word_spacing: String,
        #[doc = include_str!("_docs/attributes/decoration.md")]
        decoration: String,
        #[doc = include_str!("_docs/attributes/decoration_style.md")]
        decoration_style: String,
        #[doc = include_str!("_docs/attributes/decoration_color.md")]
        decoration_color: String,
    };
    /// `image` element let's you show an image.
    ///
    /// ### Example
    ///
    /// ```rust, ignore, no_run
    /// # use freya::prelude::*;
    /// static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");
    ///
    /// fn app() -> Element {
    ///     let image_data = static_bytes(RUST_LOGO);
    ///     rsx!(
    ///         image {
    ///             image_data: image_data,
    ///             width: "{size}",
    ///             height: "{size}",
    ///         }
    ///     )
    /// }
    /// ```
    image {
       #[doc = include_str!("_docs/attributes/width_height.md")]
        height: String,
        width: String,
        #[doc = include_str!("_docs/attributes/rotate.md")]
        rotate: String,
        #[doc = include_str!("_docs/attributes/opacity.md")]
        opacity: String,

        image_data: String,
        image_reference: String,
        role: String,
        focus_id: AccessibilityId,
        alt: String,
        name: String,
        focusable: String,
    };
    /// `svg` element let's you display SVG code.
    ///
    /// You will need to use the [`dynamic_bytes`](https://docs.freyaui.dev/freya/prelude/fn.dynamic_bytes.html)
    /// to transform the bytes into data the element can recognize.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// # use freya::prelude::*;
    /// static FERRIS: &[u8] = include_bytes!("./ferris.svg");
    ///
    /// fn app() -> Element {
    ///     let ferris = dynamic_bytes(FERRIS);
    ///     rsx!(
    ///         svg {
    ///             svg_data: ferris,
    ///         }
    ///     )
    /// }
    /// ```
    svg {
        #[doc = include_str!("_docs/attributes/margin.md")]
        margin: String,
       #[doc = include_str!("_docs/attributes/width_height.md")]
        height: String,
        width: String,
        #[doc = include_str!("_docs/attributes/rotate.md")]
        rotate: String,
        #[doc = include_str!("_docs/attributes/opacity.md")]
        opacity: String,

        svg_data: String,
        svg_content: String,
        role: String,
        focus_id: AccessibilityId,
        alt: String,
        name: String,
        focusable: String,
    };
}

pub mod events {
    use std::any::Any;

    use crate::events::*;

    #[doc(hidden)]
    pub trait EventReturn<P>: Sized {
        fn spawn(self) {}
    }

    impl EventReturn<()> for () {}
    #[doc(hidden)]
    pub struct AsyncMarker;

    impl<T> EventReturn<AsyncMarker> for T
    where
        T: std::future::Future<Output = ()> + 'static,
    {
        #[inline]
        fn spawn(self) {
            dioxus_core::prelude::spawn(self);
        }
    }

    /// A platform specific event.
    #[doc(hidden)]
    pub struct PlatformEventData {
        event: Box<dyn Any>,
    }

    impl PlatformEventData {
        pub fn new(event: Box<dyn Any>) -> Self {
            Self { event }
        }

        pub fn downcast<T: 'static>(&self) -> Option<&T> {
            self.event.downcast_ref::<T>()
        }

        pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
            self.event.downcast_mut::<T>()
        }

        pub fn into_inner<T: 'static>(self) -> Option<T> {
            self.event.downcast::<T>().ok().map(|e| *e)
        }
    }

    macro_rules! impl_event {
        (
            $data:ty;
            $(
                $( #[$attr:meta] )*
                $name:ident $(: $js_name:literal)?
            )*
        ) => {
            $(
                $( #[$attr] )*
                #[inline]
                pub fn $name<E: EventReturn<T>, T>(mut _f: impl FnMut(::dioxus_core::Event<$data>) -> E + 'static) -> ::dioxus_core::Attribute {
                    ::dioxus_core::Attribute::new(
                        stringify!($name),
    ::dioxus_core::AttributeValue::listener(move |e: ::dioxus_core::Event<PlatformEventData>| {
                            _f(e.map(|e|e.into())).spawn();
                        }),
                        None,
                        false,
                    ).into()
                }
            )*
        };
    }

    impl_event! [
        MouseData;

        #[doc = include_str!("_docs/events/click.md")]
        onclick
        #[doc = include_str!("_docs/events/globalclick.md")]
        onglobalclick
        #[doc = include_str!("_docs/events/onmiddleclick.md")]
        onmiddleclick
        #[doc = include_str!("_docs/events/onrightclick.md")]
        onrightclick
        #[doc = include_str!("_docs/events/mousedown.md")]
        onmousedown
        #[doc = include_str!("_docs/events/globalmousedown.md")]
        onglobalmousedown
        #[doc = include_str!("_docs/events/mouseover.md")]
        onmouseover
        #[doc = include_str!("_docs/events/globalmouseover.md")]
        onglobalmouseover
        #[doc = include_str!("_docs/events/mouseleave.md")]
        onmouseleave
        #[doc = include_str!("_docs/events/mouseenter.md")]
        onmouseenter
    ];

    impl_event! [
        WheelData;

        #[doc = include_str!("_docs/events/wheel.md")]
        onwheel
    ];

    impl_event! [
        KeyboardData;

        #[doc = include_str!("_docs/events/keydown.md")]
        onkeydown
        #[doc = include_str!("_docs/events/keyup.md")]
        onkeyup
    ];

    impl_event! [
        TouchData;

        #[doc = include_str!("_docs/events/touchcancel.md")]
        ontouchcancel
        #[doc = include_str!("_docs/events/touchend.md")]
        ontouchend
        #[doc = include_str!("_docs/events/touchmove.md")]
        ontouchmove
        #[doc = include_str!("_docs/events/touchstart.md")]
        ontouchstart
    ];

    impl_event! [
        PointerData;

        #[doc = include_str!("_docs/events/pointerdown.md")]
        onpointerdown
        #[doc = include_str!("_docs/events/pointerup.md")]
        onpointerup
        #[doc = include_str!("_docs/events/onglobalpointerup.md")]
        onglobalpointerup
        #[doc = include_str!("_docs/events/pointerover.md")]
        onpointerover
        #[doc = include_str!("_docs/events/pointerenter.md")]
        onpointerenter
        #[doc = include_str!("_docs/events/pointerleave.md")]
        onpointerleave
    ];

    impl_event! [
        FileData;

        #[doc = include_str!("_docs/events/onfiledrop.md")]
        onfiledrop
        #[doc = include_str!("_docs/events/onglobalfilehover.md")]
        onglobalfilehover
        #[doc = include_str!("_docs/events/onglobalfilehovercancelled.md")]
        onglobalfilehovercancelled
    ];
}
