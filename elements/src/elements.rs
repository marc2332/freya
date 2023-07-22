pub use dioxus_core::AttributeValue;
use dioxus_rsx::HotReloadingContext;
pub use events::*;

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
                    pub const $fil: AttributeDescription = (stringify!($fil), None, false);
                )*
            }

            impl GlobalAttributes for $name {}
        )*
    };
}

builder_constructors! {
    rect {
        padding: String,
        height: String,
        width: String,
        min_height: String,
        min_width: String,
        max_height: String,
        max_width: String,
        background: String,
        border: String,
        border_align: String,
        layer: String,
        offset_y: String,
        offset_x: String,
        direction: String,
        shadow: String,
        corner_radius: String,
        corner_smoothing: String,
        color: String,
        display: String,
        reference: Reference,
        cursor_reference: CursorReference,
        rotate: String,
        role: String,
        focus_id: AccessibilityId,
        alt: String,
        canvas_reference: String,
        overflow: String,
        name: String,
        focusable: String,
        margin: String,
    };
    label {
        color: String,
        text_shadow: String,
        layer: String,
        height: String,
        width: String,
        font_size: String,
        font_family: String,
        font_style: String,
        font_weight: String,
        font_width: String,
        align: String,
        max_lines: String,
        rotate: String,
        role: String,
        alt: String,
        focus_id: AccessibilityId,
        name: String,
        letter_spacing: String,
        word_spacing: String,
        decoration: String,
        decoration_style: String,
        decoration_color: String,
        text_overflow: String,
        focusable: String,
    };
    paragraph {
        layer: String,
        height: String,
        width: String,
        min_height: String,
        min_width: String,
        max_height: String,
        max_width: String,
        align: String,
        cursor_index: String,
        max_lines: String,
        cursor_color: String,
        cursor_mode: String,
        line_height: String,
        cursor_id: String,
        direction: String,
        rotate: String,
        role: String,
        focus_id: AccessibilityId,
        highlights: String,
        highlight_color: String,
        font_size: String,
        font_family: String,
        alt: String,
        name: String,
        font_style: String,
        font_weight: String,
        font_width: String,
        letter_spacing: String,
        word_spacing: String,
        decoration: String,
        decoration_style: String,
        decoration_color: String,
        text_overflow: String,
        focusable: String,
    };
    text {
        color: String,
        text_shadow: String,
        layer: String,
        height: String,
        width: String,
        font_size: String,
        font_family: String,
        font_style: String,
        font_weight: String,
        font_width: String,
        line_height: String,
        letter_spacing: String,
        word_spacing: String,
        decoration: String,
        decoration_style: String,
        decoration_color: String,
    };
    image {
        image_data: String,
        image_reference: String,
        width: String,
        height: String,
        rotate: String,
        role: String,
        focus_id: AccessibilityId,
        alt: String,
        name: String,
        focusable: String,
    };
    svg {
        svg_data: String,
        svg_content: String,
        width: String,
        height: String,
        rotate: String,
        role: String,
        focus_id: AccessibilityId,
        alt: String,
        name: String,
        focusable: String,
    };
}

pub mod events {
    use crate::events::{KeyboardData, MouseData, PointerData, TouchData, WheelData};

    macro_rules! impl_event {
        (
            $data:ty;
            $(
                $( #[$attr:meta] )*
                $name:ident
            )*
        ) => {
            $(
                $( #[$attr] )*
                pub fn $name<'a>(_cx: &'a ::dioxus_core::ScopeState, _f: impl FnMut(::dioxus_core::Event<$data>) + 'a) -> ::dioxus_core::Attribute<'a> {
                    ::dioxus_core::Attribute {
                        name: stringify!($name),
                        value: _cx.listener(_f),
                        namespace: None,
                        mounted_element: Default::default(),
                        volatile: false,
                    }
                }
            )*
        };
    }

    impl_event! [
        MouseData;

        onclick
        onglobalclick
        onmousedown
        onmouseover
        onglobalmouseover
        onmouseleave
        onmouseenter
    ];

    impl_event! [
        WheelData;

        onwheel
    ];

    impl_event! [
        KeyboardData;

        onkeydown
        onkeyup
    ];

    impl_event! [
        TouchData;

        ontouchcancel
        ontouchend
        ontouchmove
        ontouchstart
    ];

    impl_event! [
        PointerData;

        onpointerdown
        onpointerup
        onpointerover
        onpointerenter
        onpointerleave
    ];
}

pub trait GlobalAttributes {}

pub trait SvgAttributes {}
