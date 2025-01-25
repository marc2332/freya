pub use events::*;

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
            impl_element!(
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
                   pub use crate::attributes::$fil::$fil;
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

def_element!(
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
        // Layout
        height,
        width,
        min_height,
        min_width,
        max_height,
        max_width,
        margin,
        padding,
        position,
        position_top,
        position_right,
        position_bottom,
        position_left,
        layer,

        // Children layout
        direction,
        content,
        main_align,
        cross_align,
        spacing,
        overflow,
        offset_x,
        offset_y,

        // Style
        background,
        border,
        shadow,
        corner_radius,
        corner_smoothing,

        // Font style
        color,
        font_size,
        font_family,
        font_style,
        font_weight,
        font_width,
        text_align,
        line_height,
        text_shadow,
        max_lines,
        decoration,
        decoration_style,
        decoration_color,
        text_overflow,
        letter_spacing,
        word_spacing,
        text_height,

        // Transform
        rotate,
        opacity,

        // Reference
        canvas_reference,
        reference,
        cursor_reference,

        // Accessibility
        a11y_id,
        a11y_focusable,
        a11y_auto_focus,
        a11y_name,
        a11y_description,
        a11y_value,
        a11y_access_key,
        a11y_author_id,
        a11y_keyboard_shortcut,
        a11y_language,
        a11y_placeholder,
        a11y_role_description,
        a11y_state_description,
        a11y_tooltip,
        a11y_url,
        a11y_row_index_text,
        a11y_column_index_text,
        a11y_scroll_x,
        a11y_scroll_x_min,
        a11y_scroll_x_max,
        a11y_scroll_y,
        a11y_scroll_y_min,
        a11y_scroll_y_max,
        a11y_numeric_value,
        a11y_min_numeric_value,
        a11y_max_numeric_value,
        a11y_numeric_value_step,
        a11y_numeric_value_jump,
        a11y_row_count,
        a11y_column_count,
        a11y_row_index,
        a11y_column_index,
        a11y_row_span,
        a11y_column_span,
        a11y_level,
        a11y_size_of_set,
        a11y_position_in_set,
        a11y_color_value,
        a11y_expanded,
        a11y_selected,
        a11y_hovered,
        a11y_hidden,
        a11y_linked,
        a11y_multiselectable,
        a11y_required,
        a11y_visited,
        a11y_busy,
        a11y_live_atomic,
        a11y_modal,
        a11y_touch_transparent,
        a11y_read_only,
        a11y_disabled,
        a11y_is_spelling_error,
        a11y_is_grammar_error,
        a11y_is_search_match,
        a11y_is_suggestion,
        a11y_role,
        a11y_invalid,
        a11y_toggled,
        a11y_live,
        a11y_default_action_verb,
        a11y_orientation,
        a11y_sort_direction,
        a11y_current,
        a11y_auto_complete,
        a11y_has_popup,
        a11y_list_style,
        a11y_vertical_offset,
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
        // Layout
        height,
        width,
        min_height,
        min_width,
        max_height,
        max_width,
        margin,
        position,
        position_top,
        position_right,
        position_bottom,
        position_left,
        layer,

        // Children layout
        main_align,

        // Font style
        color,
        font_size,
        font_family,
        font_style,
        font_weight,
        font_width,
        text_align,
        line_height,
        text_shadow,
        max_lines,
        decoration,
        decoration_style,
        decoration_color,
        text_overflow,
        letter_spacing,
        word_spacing,
        text_height,

        // Transform
        rotate,
        opacity,

        // Reference
        reference,

        // Accessibility
        a11y_id,
        a11y_auto_focus,
        a11y_focusable,
        a11y_name,
        a11y_description,
        a11y_value,
        a11y_access_key,
        a11y_author_id,
        a11y_keyboard_shortcut,
        a11y_language,
        a11y_placeholder,
        a11y_role_description,
        a11y_state_description,
        a11y_tooltip,
        a11y_url,
        a11y_row_index_text,
        a11y_column_index_text,
        a11y_scroll_x,
        a11y_scroll_x_min,
        a11y_scroll_x_max,
        a11y_scroll_y,
        a11y_scroll_y_min,
        a11y_scroll_y_max,
        a11y_numeric_value,
        a11y_min_numeric_value,
        a11y_max_numeric_value,
        a11y_numeric_value_step,
        a11y_numeric_value_jump,
        a11y_row_count,
        a11y_column_count,
        a11y_row_index,
        a11y_column_index,
        a11y_row_span,
        a11y_column_span,
        a11y_level,
        a11y_size_of_set,
        a11y_position_in_set,
        a11y_color_value,
        a11y_expanded,
        a11y_selected,
        a11y_hovered,
        a11y_hidden,
        a11y_linked,
        a11y_multiselectable,
        a11y_required,
        a11y_visited,
        a11y_busy,
        a11y_live_atomic,
        a11y_modal,
        a11y_touch_transparent,
        a11y_read_only,
        a11y_disabled,
        a11y_is_spelling_error,
        a11y_is_grammar_error,
        a11y_is_search_match,
        a11y_is_suggestion,
        a11y_role,
        a11y_invalid,
        a11y_toggled,
        a11y_live,
        a11y_default_action_verb,
        a11y_orientation,
        a11y_sort_direction,
        a11y_current,
        a11y_auto_complete,
        a11y_has_popup,
        a11y_list_style,
        a11y_vertical_offset,
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
        // Layout
        height,
        width,
        min_height,
        min_width,
        max_height,
        max_width,
        margin,
        position,
        position_top,
        position_right,
        position_bottom,
        position_left,
        layer,

        // Children layout
        main_align,

        // Font style
        color,
        font_size,
        font_family,
        font_style,
        font_weight,
        font_width,
        text_align,
        line_height,
        text_shadow,
        max_lines,
        decoration,
        decoration_style,
        decoration_color,
        text_overflow,
        letter_spacing,
        word_spacing,
        text_height,

        // Transform
        rotate,
        opacity,

        // Text Editing
        cursor_index,
        cursor_color,
        cursor_mode,
        cursor_id,
        highlights,
        highlight_color,
        highlight_mode,

        // Accessibility
        a11y_id,
        a11y_focusable,
        a11y_auto_focus,
        a11y_name,
        a11y_description,
        a11y_value,
        a11y_access_key,
        a11y_author_id,
        a11y_keyboard_shortcut,
        a11y_language,
        a11y_placeholder,
        a11y_role_description,
        a11y_state_description,
        a11y_tooltip,
        a11y_url,
        a11y_row_index_text,
        a11y_column_index_text,
        a11y_scroll_x,
        a11y_scroll_x_min,
        a11y_scroll_x_max,
        a11y_scroll_y,
        a11y_scroll_y_min,
        a11y_scroll_y_max,
        a11y_numeric_value,
        a11y_min_numeric_value,
        a11y_max_numeric_value,
        a11y_numeric_value_step,
        a11y_numeric_value_jump,
        a11y_row_count,
        a11y_column_count,
        a11y_row_index,
        a11y_column_index,
        a11y_row_span,
        a11y_column_span,
        a11y_level,
        a11y_size_of_set,
        a11y_position_in_set,
        a11y_color_value,
        a11y_expanded,
        a11y_selected,
        a11y_hovered,
        a11y_hidden,
        a11y_linked,
        a11y_multiselectable,
        a11y_required,
        a11y_visited,
        a11y_busy,
        a11y_live_atomic,
        a11y_modal,
        a11y_touch_transparent,
        a11y_read_only,
        a11y_disabled,
        a11y_is_spelling_error,
        a11y_is_grammar_error,
        a11y_is_search_match,
        a11y_is_suggestion,
        a11y_role,
        a11y_invalid,
        a11y_toggled,
        a11y_live,
        a11y_default_action_verb,
        a11y_orientation,
        a11y_sort_direction,
        a11y_current,
        a11y_auto_complete,
        a11y_has_popup,
        a11y_list_style,
        a11y_vertical_offset,
    };
    /// `text` element is simply a text span used for the `paragraph` element.
    text {
        // Font style
        color,
        font_size,
        font_family,
        font_style,
        font_weight,
        font_width,
        text_align,
        line_height,
        text_shadow,
        decoration,
        decoration_style,
        decoration_color,
        letter_spacing,
        word_spacing,
    };
    /// `image` element let's you show an image.
    ///
    /// For dynamic Images you may use `dynamic_bytes`.
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
    ///             width: "100%", // You must specify size otherwhise it will default to 0
    ///             height: "100%",
    ///         }
    ///     )
    /// }
    /// ```
    image {
        // Layout
        height,
        width,
        min_height,
        min_width,
        max_height,
        max_width,
        margin,
        position,
        position_top,
        position_right,
        position_bottom,
        position_left,
        layer,

        // Transform
        rotate,
        opacity,

        // Image
        image_data,

        // Reference
        reference,
        image_reference,

        // Accessibility
        a11y_id,
        a11y_focusable,
        a11y_auto_focus,
        a11y_name,
        a11y_description,
        a11y_value,
        a11y_access_key,
        a11y_author_id,
        a11y_keyboard_shortcut,
        a11y_language,
        a11y_placeholder,
        a11y_role_description,
        a11y_state_description,
        a11y_tooltip,
        a11y_url,
        a11y_row_index_text,
        a11y_column_index_text,
        a11y_scroll_x,
        a11y_scroll_x_min,
        a11y_scroll_x_max,
        a11y_scroll_y,
        a11y_scroll_y_min,
        a11y_scroll_y_max,
        a11y_numeric_value,
        a11y_min_numeric_value,
        a11y_max_numeric_value,
        a11y_numeric_value_step,
        a11y_numeric_value_jump,
        a11y_row_count,
        a11y_column_count,
        a11y_row_index,
        a11y_column_index,
        a11y_row_span,
        a11y_column_span,
        a11y_level,
        a11y_size_of_set,
        a11y_position_in_set,
        a11y_color_value,
        a11y_expanded,
        a11y_selected,
        a11y_hovered,
        a11y_hidden,
        a11y_linked,
        a11y_multiselectable,
        a11y_required,
        a11y_visited,
        a11y_busy,
        a11y_live_atomic,
        a11y_modal,
        a11y_touch_transparent,
        a11y_read_only,
        a11y_disabled,
        a11y_is_spelling_error,
        a11y_is_grammar_error,
        a11y_is_search_match,
        a11y_is_suggestion,
        a11y_role,
        a11y_invalid,
        a11y_toggled,
        a11y_live,
        a11y_default_action_verb,
        a11y_orientation,
        a11y_sort_direction,
        a11y_current,
        a11y_auto_complete,
        a11y_has_popup,
        a11y_list_style,
        a11y_vertical_offset,
    };
    /// `svg` element let's you display SVG code.
    ///
    /// For dynamic SVGs you may use `dynamic_bytes`.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// # use freya::prelude::*;
    /// static FERRIS: &[u8] = include_bytes!("./ferris.svg");
    ///
    /// fn app() -> Element {
    ///     let ferris = static_bytes(FERRIS);
    ///     rsx!(
    ///         svg {
    ///             svg_data: ferris,
    ///             width: "100%", // You must specify size otherwhise it will default to 0
    ///             height: "100%",
    ///         }
    ///     )
    /// }
    /// ```
    svg {
        // Layout
        height,
        width,
        min_height,
        min_width,
        max_height,
        max_width,
        margin,
        position,
        position_top,
        position_right,
        position_bottom,
        position_left,
        layer,

        // Transform
        rotate,
        opacity,

        // Svg
        color,
        svg_data,
        svg_content,
        fill,
        stroke,

        // Accessibility
        a11y_id,
        a11y_focusable,
        a11y_auto_focus,
        a11y_name,
        a11y_description,
        a11y_value,
        a11y_access_key,
        a11y_author_id,
        a11y_keyboard_shortcut,
        a11y_language,
        a11y_placeholder,
        a11y_role_description,
        a11y_state_description,
        a11y_tooltip,
        a11y_url,
        a11y_row_index_text,
        a11y_column_index_text,
        a11y_scroll_x,
        a11y_scroll_x_min,
        a11y_scroll_x_max,
        a11y_scroll_y,
        a11y_scroll_y_min,
        a11y_scroll_y_max,
        a11y_numeric_value,
        a11y_min_numeric_value,
        a11y_max_numeric_value,
        a11y_numeric_value_step,
        a11y_numeric_value_jump,
        a11y_row_count,
        a11y_column_count,
        a11y_row_index,
        a11y_column_index,
        a11y_row_span,
        a11y_column_span,
        a11y_level,
        a11y_size_of_set,
        a11y_position_in_set,
        a11y_color_value,
        a11y_expanded,
        a11y_selected,
        a11y_hovered,
        a11y_hidden,
        a11y_linked,
        a11y_multiselectable,
        a11y_required,
        a11y_visited,
        a11y_busy,
        a11y_live_atomic,
        a11y_modal,
        a11y_touch_transparent,
        a11y_read_only,
        a11y_disabled,
        a11y_is_spelling_error,
        a11y_is_grammar_error,
        a11y_is_search_match,
        a11y_is_suggestion,
        a11y_role,
        a11y_invalid,
        a11y_toggled,
        a11y_live,
        a11y_default_action_verb,
        a11y_orientation,
        a11y_sort_direction,
        a11y_current,
        a11y_auto_complete,
        a11y_has_popup,
        a11y_list_style,
        a11y_vertical_offset,
    };
);

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
                        ::dioxus_core::AttributeValue::listener(move |e: ::dioxus_core::Event<crate::PlatformEventData>| {
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

    impl_event! [
        MouseData;

        onclick
        onglobalclick
        onmiddleclick
        onrightclick
        onmouseup
        onmousedown
        onglobalmousedown
        onmousemove
        onglobalmousemove
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

        onglobalkeydown
        onglobalkeyup
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
        onglobalpointerup
        onpointermove
        onpointerenter
        onpointerleave
    ];

    impl_event! [
        FileData;

        onfiledrop
        onglobalfilehover
        onglobalfilehovercancelled
    ];
}
