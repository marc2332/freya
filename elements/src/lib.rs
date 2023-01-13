pub mod events_data;
pub use dioxus_core::AttributeValue;
pub use events_data::*;

pub type AttributeDescription = (&'static str, Option<&'static str>, bool);

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
        $(
            #[allow(non_camel_case_types)]
            $(#[$attr])*
            pub struct $name;

            impl $name {
                pub const TAG_NAME: &'static str = stringify!($name);
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
        layer: String,
        scroll_y: String,
        scroll_x: String,
        direction: String,
        shadow: String,
        radius: String,
        color: String,
        display: String,
        reference: Reference,
        cursor_reference: CursorReference,
    };
    container {
        padding: String,
        height: String,
        width: String,
        min_height: String,
        min_width: String,
        max_height: String,
        max_width: String,
        background: String,
        layer: String,
        scroll_y: String,
        scroll_x: String,
        direction: String,
        shadow: String,
        radius: String,
        color: String,
        display: String,
        reference: Reference,
        cursor_reference: CursorReference,
    };
    label {
        color: String,
        layer: String,
        height: String,
        width: String,
        font_size: String,
        font_family: String,
        align: String,
        max_lines: String,
        font_style: String,
    };
    paragraph {
        layer: String,
        width: String,
        height: String,
        align: String,
        cursor_index: String,
        max_lines: String,
        cursor_color: String,
        cursor_mode: String,
        line_height: String,
        cursor_id: String,
        direction: String,
    };
    text {
        color: String,
        layer: String,
        height: String,
        width: String,
        font_size: String,
        font_family: String,
        line_height: String,
        font_style: String,
    };
    image {
        image_data: String,
        image_reference: String,
        width: String,
        height: String,
    };
    svg {
        svg_data: String,
        svg_content: String,
        width: String,
        height: String,
    };
}

pub mod events {
    use crate::events_data::{KeyboardData, MouseData, WheelData};

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
        onmousedown
        onmouseover
        onmouseleave
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
}

pub trait GlobalAttributes {}

pub trait SvgAttributes {}
