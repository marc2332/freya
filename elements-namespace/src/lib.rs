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
                $(
                    $(#[$attr_method])*
                    pub fn $fil<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
                        cx.attr(stringify!($fil), val, None, false)
                    }
                )*
            }
        )*
    };

    ( $(
        $(#[$attr:meta])*
        $name:ident <> $namespace:tt {
            $($fil:ident: $vil:ident,)*
        };
    )* ) => {
        $(
            #[allow(non_camel_case_types)]
            $(#[$attr])*
            pub struct $name;

            impl DioxusElement for $name {
                const TAG_NAME: &'static str = stringify!($name);
                const NAME_SPACE: Option<&'static str> = Some($namespace);
            }


            impl $name {
                $(
                    pub fn $fil<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
                        cx.attr(stringify!($fil), val, Some(stringify!($namespace)), false)
                    }
                )*
            }
        )*
    };
}

builder_constructors! {
    view {
        padding: String,
        height: String,
        width: String,
        background: String,
        layer: String,
        scroll_y: String,
        direction: String,
        shadow: String,
        radius: String,
     };
    text {
        layer: String,
        height: String,
     };
}

pub trait GlobalAttributes {
    fn prevent_default<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
        cx.attr("dioxus-prevent-default", val, None, false)
    }
}

pub trait SvgAttributes {
    fn prevent_default<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
        cx.attr("dioxus-prevent-default", val, None, false)
    }
}

pub mod on {
    use dioxus_core::*;
    use dioxus_html::on::{MouseData, MouseEvent};

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


                    use dioxus_core::{AnyEvent};
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
            onmouseover
            onscroll
        ];
    }
}
