#[macro_export]
macro_rules! def_element {
    (
        $(
            $(#[$attr:meta])*
            $name:ident {
                $(
                    $(#[$attr_method:meta])*
                    $fil:ident,
                )*
            };
         )*
        ) => {
        $(
            $crate::impl_element!(
                $(#[$attr])*
                $name {
                    $(
                        $(#[$attr_method])*
                        $fil,
                    )*
                };
            );
        )*

        /// This module contains helpers for rust analyzer autocompletion
        #[doc(hidden)]
        pub mod completions {
            /// This helper tells rust analyzer that it should autocomplete the element name with braces.
            #[allow(non_camel_case_types)]
            pub enum CompleteWithBraces {
                $(
                    $(#[$attr])*
                    $name {}
                ),*
            }
        }
    };
}

#[macro_export]
macro_rules! impl_element {
    (
        $(
            $(#[$attr:meta])*
            $name:ident {
                $(
                    $(#[$attr_method:meta])*
                    $fil:ident,
                )*
            };
         )*
    ) => {
        $(
            #[allow(non_camel_case_types)]
            $(#[$attr])*
            pub mod $name {
                #[doc(hidden)]
                pub const TAG_NAME: &'static str = stringify!($name);
                #[doc(hidden)]
                pub const NAME_SPACE: Option<&'static str> = None;

                $(
                   pub use $crate::attributes::$fil::$fil;
                )*
            }
        )*
    };
}

#[macro_export]
macro_rules! def_attribute {
    (
        $(
            $(#[$attr:meta])*
            $fil:ident,
         )*
    ) => {
        $(
            #[allow(non_camel_case_types)]
            pub mod $fil {

                #[allow(non_upper_case_globals)]
                $(#[$attr])*
                pub const $fil: (&'static str, Option<&'static str>, bool) = (stringify!($fil), None, false);
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_event {
        (
            $data:ty;
            $(
                $( #[$attr:meta] )*
                $name:ident $(: $event:literal)?
            )*
        ) => {
            $(
                $( #[$attr] )*
                #[inline]
                pub fn $name<__Marker>(mut _f: impl ::dioxus_core::prelude::SuperInto<::dioxus_core::prelude::EventHandler<::dioxus_core::Event<$data>>, __Marker>) -> ::dioxus_core::Attribute {
                    // super into will make a closure that is owned by the current owner (either the child component or the parent component).
                    // We can't change that behavior in a minor version because it would cause issues with Components that accept event handlers.
                    // Instead we run super into with an owner that is moved into the listener closure so it will be dropped when the closure is dropped.
                    let owner = <::generational_box::UnsyncStorage as ::generational_box::AnyStorage>::owner();
                    let event_handler = ::dioxus_core::prelude::with_owner(owner.clone(), || _f.super_into());
                    ::dioxus_core::Attribute::new(
                        impl_event!(@name $name $($event)?),
                        ::dioxus_core::AttributeValue::listener(move |e: ::dioxus_core::Event<$crate::events::ErasedEventData>| {
                            // Force the owner to be moved into the event handler
                            _ = &owner;
                            event_handler.call(e.map(|e| e.into()));
                        }),
                        None,
                        false,
                    ).into()
                }

                #[doc(hidden)]
                $( #[$attr] )*
                pub mod $name {
                    use super::*;

                    // When expanding the macro, we use this version of the function if we see an inline closure to give better type inference
                    $( #[$attr] )*
                    pub fn call_with_explicit_closure<
                        __Marker,
                        Return: ::dioxus_core::SpawnIfAsync<__Marker> + 'static,
                    >(
                        event_handler: impl FnMut(::dioxus_core::Event<$data>) -> Return + 'static,
                    ) -> ::dioxus_core::Attribute {
                        #[allow(deprecated)]
                        super::$name(event_handler)
                    }
                }
            )*
        };

        (@name $name:ident) => {
            stringify!($name)
        };
    }

#[doc(hidden)]
#[allow(dead_code)]
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
