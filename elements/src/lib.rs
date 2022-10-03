pub use dioxus_core::AttributeValue;
use dioxus_core::*;
use std::fmt::Arguments;

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

            impl DioxusElement for $name {
                const TAG_NAME: &'static str = stringify!($name);
                const NAME_SPACE: Option<&'static str> = None;
            }

            impl $name {
                pub fn reference<'a, T: Clone + 'static + PartialEq>(
                    &self,
                    cx: NodeFactory<'a>,
                    val: &'a T,
                ) -> Attribute<'a> {
                    cx.custom_attr(
                        "reference",
                        AttributeValue::Any(ArbitraryAttributeValue {
                            value: val,
                            cmp: |a, b| a.downcast_ref::<T>() == b.downcast_ref::<T>(),
                        }),
                        None,
                        true,
                        false,
                    )
                }
            }

            impl $name {
                $(
                    $(#[$attr_method])*
                    pub fn $fil<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
                        cx.attr(stringify!($fil), val, None, false)
                    }
                )*
            }
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
        background: String,
        layer: String,
        scroll_y: String,
        scroll_x: String,
        direction: String,
        shadow: String,
        radius: String,
        color: String,
    };
    container {
        padding: String,
        height: String,
        width: String,
        min_height: String,
        min_width: String,
        background: String,
        layer: String,
        scroll_y: String,
        scroll_x: String,
        direction: String,
        shadow: String,
        radius: String,
        color: String,
    };
    label {
        color: String,
        layer: String,
        height: String,
        width: String,
        font_size: String,
        font_family: String,
    };
    paragraph {
        layer: String,
        width: String,
    };
    text {
        color: String,
        layer: String,
        height: String,
        width: String,
        font_size: String,
        font_family: String,
    };
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub inner_height: f32,
    pub inner_width: f32,
}

impl NodeLayout {
    pub fn new() -> Self {
        NodeLayout {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            inner_height: 0.0,
            inner_width: 0.0,
        }
    }
}

// I am still bad at Macros so I created this element directly
#[allow(non_camel_case_types)]
pub struct image;

impl DioxusElement for image {
    const TAG_NAME: &'static str = "image";
    const NAME_SPACE: Option<&'static str> = None;
}

impl image {
    pub fn image_data<'a>(&self, cx: NodeFactory<'a>, val: AttributeValue<'a>) -> Attribute<'a> {
        cx.custom_attr("image_data", val, None, false, false)
    }

    pub fn width<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
        cx.attr("width", val, None, false)
    }

    pub fn height<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
        cx.attr("height", val, None, false)
    }
}

#[allow(non_camel_case_types)]
pub struct svg;

impl DioxusElement for svg {
    const TAG_NAME: &'static str = "svg";
    const NAME_SPACE: Option<&'static str> = None;
}

impl svg {
    pub fn svg_data<'a>(&self, cx: NodeFactory<'a>, val: AttributeValue<'a>) -> Attribute<'a> {
        cx.custom_attr("svg_data", val, None, false, false)
    }

    pub fn svg_content<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
        cx.attr("svg_content", val, None, false)
    }

    pub fn width<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
        cx.attr("width", val, None, false)
    }

    pub fn height<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
        cx.attr("height", val, None, false)
    }
}

pub mod on {
    use dioxus_core::*;
    use dioxus_html::on::{MouseData, MouseEvent, WheelData, WheelEvent};

    use bumpalo::boxed::Box as BumpBox;

    macro_rules! event_directory {
        ( $(
            $( #[$attr:meta] )*
            $wrapper:ident($data:ident): [
                $(
                    $( #[$method_attr:meta] )*
                    $name:ident
                )*
            ];
        )* ) => {
            $(
                $(
                    $(#[$method_attr])*
                    pub fn $name<'a>(
                        factory: NodeFactory<'a>,
                        mut callback: impl FnMut($wrapper) + 'a,
                        // mut callback: impl FnMut(UiEvent<$data>) + 'a,
                    ) -> Listener<'a>
                    {
                        let bump = &factory.bump();

                        // we can't allocate unsized in bumpalo's box, so we need to craft the box manually
                        // safety: this is essentially the same as calling Box::new() but manually
                        // The box is attached to the lifetime of the bumpalo allocator
                        let cb: &mut dyn FnMut(AnyEvent) = bump.alloc(move |evt: AnyEvent| {
                            let event = evt.downcast::<$data>().unwrap();
                            callback(event)
                        });

                        let callback: BumpBox<dyn FnMut(AnyEvent) + 'a> = unsafe { BumpBox::from_raw(cb) };

                        // ie oncopy
                        let event_name = stringify!($name);

                        // ie copy
                        let shortname: &'static str = &event_name[2..];

                        let handler = bump.alloc(std::cell::RefCell::new(Some(callback)));
                        factory.listener(shortname, handler)
                    }
                )*
            )*
        };
    }

    event_directory! {
        MouseEvent(MouseData): [
            onclick
            onmousedown
            onmouseover
            onmouseleave
        ];
        WheelEvent(WheelData): [
            onwheel
        ];
    }
}

pub trait GlobalAttributes {}

pub trait SvgAttributes {}
