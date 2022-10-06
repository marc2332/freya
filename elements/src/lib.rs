use dioxus_core::AttributeDiscription;
pub use dioxus_core::AttributeValue;
use dioxus_core::*;
use std::fmt::Display;
use tokio::sync::mpsc::UnboundedSender;

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
                $(
                    #[allow(non_upper_case_globals)]
                    pub const $fil: AttributeDiscription = AttributeDiscription{
                        name: stringify!($fil),
                        namespace: None,
                        volatile: false
                    };
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
        reference: NodeRefWrapper,
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
        reference: NodeRefWrapper,
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
    image {
        image_data: String,
        width: String,
        height: String,
    };
    svg {
        svg_data: String,
        width: String,
        height: String,
    };
}

#[derive(Clone)]
pub struct NodeRefWrapper(pub UnboundedSender<NodeLayout>);

// Hacky
impl PartialEq for NodeRefWrapper {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

// Hacky
impl Display for NodeRefWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeRefWrapper").finish_non_exhaustive()
    }
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
