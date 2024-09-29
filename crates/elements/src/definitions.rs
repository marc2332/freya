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
        #[doc = include_str!("_docs/attributes/line_height.md")]
        line_height: String,
        #[doc = include_str!("_docs/attributes/spacing.md")]
        spacing: String,

        canvas_reference: String,
        layer: String,
        offset_y: String,
        offset_x: String,
        reference: Reference,
        cursor_reference: CursorReference,

        a11y_id: String,
        a11y_focusable: String,
        a11y_auto_focus: String,
        a11y_name: String,
        a11y_description: String,
        a11y_value: String,
        a11y_access_key: String,
        a11y_author_id: String,
        a11y_keyboard_shortcut: String,
        a11y_language: String,
        a11y_placeholder: String,
        a11y_role_description: String,
        a11y_state_description: String,
        a11y_tooltip: String,
        a11y_url: String,
        a11y_row_index_text: String,
        a11y_column_index_text: String,
        a11y_scroll_x: String,
        a11y_scroll_x_min: String,
        a11y_scroll_x_max: String,
        a11y_scroll_y: String,
        a11y_scroll_y_min: String,
        a11y_scroll_y_max: String,
        a11y_numeric_value: String,
        a11y_min_numeric_value: String,
        a11y_max_numeric_value: String,
        a11y_numeric_value_step: String,
        a11y_numeric_value_jump: String,
        a11y_row_count: String,
        a11y_column_count: String,
        a11y_row_index: String,
        a11y_column_index: String,
        a11y_row_span: String,
        a11y_column_span: String,
        a11y_level: String,
        a11y_size_of_set: String,
        a11y_position_in_set: String,
        a11y_color_value: String,
        a11y_expanded: String,
        a11y_selected: String,
        a11y_hovered: String,
        a11y_hidden: String,
        a11y_linked: String,
        a11y_multiselectable: String,
        a11y_required: String,
        a11y_visited: String,
        a11y_busy: String,
        a11y_live_atomic: String,
        a11y_modal: String,
        a11y_touch_transparent: String,
        a11y_read_only: String,
        a11y_disabled: String,
        a11y_is_spelling_error: String,
        a11y_is_grammar_error: String,
        a11y_is_search_match: String,
        a11y_is_suggestion: String,
        a11y_role: String,
        a11y_invalid: String,
        a11y_toggled: String,
        a11y_live: String,
        a11y_default_action_verb: String,
        a11y_orientation: String,
        a11y_sort_direction: String,
        a11y_current: String,
        a11y_auto_complete: String,
        a11y_has_popup: String,
        a11y_list_style: String,
        a11y_vertical_offset: String,
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
        #[doc = include_str!("_docs/attributes/main_align_cross_align.md")]
        main_align: String,
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
        #[doc = include_str!("_docs/attributes/margin.md")]
        margin: String,
        #[doc = include_str!("_docs/attributes/opacity.md")]
        opacity: String,

        layer: String,

        a11y_id: String,
        a11y_auto_focus: String,
        a11y_focusable: String,
        a11y_name: String,
        a11y_description: String,
        a11y_value: String,
        a11y_access_key: String,
        a11y_author_id: String,
        a11y_keyboard_shortcut: String,
        a11y_language: String,
        a11y_placeholder: String,
        a11y_role_description: String,
        a11y_state_description: String,
        a11y_tooltip: String,
        a11y_url: String,
        a11y_row_index_text: String,
        a11y_column_index_text: String,
        a11y_scroll_x: String,
        a11y_scroll_x_min: String,
        a11y_scroll_x_max: String,
        a11y_scroll_y: String,
        a11y_scroll_y_min: String,
        a11y_scroll_y_max: String,
        a11y_numeric_value: String,
        a11y_min_numeric_value: String,
        a11y_max_numeric_value: String,
        a11y_numeric_value_step: String,
        a11y_numeric_value_jump: String,
        a11y_row_count: String,
        a11y_column_count: String,
        a11y_row_index: String,
        a11y_column_index: String,
        a11y_row_span: String,
        a11y_column_span: String,
        a11y_level: String,
        a11y_size_of_set: String,
        a11y_position_in_set: String,
        a11y_color_value: String,
        a11y_expanded: String,
        a11y_selected: String,
        a11y_hovered: String,
        a11y_hidden: String,
        a11y_linked: String,
        a11y_multiselectable: String,
        a11y_required: String,
        a11y_visited: String,
        a11y_busy: String,
        a11y_live_atomic: String,
        a11y_modal: String,
        a11y_touch_transparent: String,
        a11y_read_only: String,
        a11y_disabled: String,
        a11y_is_spelling_error: String,
        a11y_is_grammar_error: String,
        a11y_is_search_match: String,
        a11y_is_suggestion: String,
        a11y_role: String,
        a11y_invalid: String,
        a11y_toggled: String,
        a11y_live: String,
        a11y_default_action_verb: String,
        a11y_orientation: String,
        a11y_sort_direction: String,
        a11y_current: String,
        a11y_auto_complete: String,
        a11y_has_popup: String,
        a11y_list_style: String,
        a11y_vertical_offset: String,
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
        #[doc = include_str!("_docs/attributes/main_align_cross_align.md")]
        main_align: String,
        #[doc = include_str!("_docs/attributes/text_align.md")]
        text_align: String,
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

        highlights: String,
        highlight_color: String,
        highlight_mode: String,

        a11y_id: String,
        a11y_focusable: String,
        a11y_auto_focus: String,
        a11y_name: String,
        a11y_description: String,
        a11y_value: String,
        a11y_access_key: String,
        a11y_author_id: String,
        a11y_keyboard_shortcut: String,
        a11y_language: String,
        a11y_placeholder: String,
        a11y_role_description: String,
        a11y_state_description: String,
        a11y_tooltip: String,
        a11y_url: String,
        a11y_row_index_text: String,
        a11y_column_index_text: String,
        a11y_scroll_x: String,
        a11y_scroll_x_min: String,
        a11y_scroll_x_max: String,
        a11y_scroll_y: String,
        a11y_scroll_y_min: String,
        a11y_scroll_y_max: String,
        a11y_numeric_value: String,
        a11y_min_numeric_value: String,
        a11y_max_numeric_value: String,
        a11y_numeric_value_step: String,
        a11y_numeric_value_jump: String,
        a11y_row_count: String,
        a11y_column_count: String,
        a11y_row_index: String,
        a11y_column_index: String,
        a11y_row_span: String,
        a11y_column_span: String,
        a11y_level: String,
        a11y_size_of_set: String,
        a11y_position_in_set: String,
        a11y_color_value: String,
        a11y_expanded: String,
        a11y_selected: String,
        a11y_hovered: String,
        a11y_hidden: String,
        a11y_linked: String,
        a11y_multiselectable: String,
        a11y_required: String,
        a11y_visited: String,
        a11y_busy: String,
        a11y_live_atomic: String,
        a11y_modal: String,
        a11y_touch_transparent: String,
        a11y_read_only: String,
        a11y_disabled: String,
        a11y_is_spelling_error: String,
        a11y_is_grammar_error: String,
        a11y_is_search_match: String,
        a11y_is_suggestion: String,
        a11y_role: String,
        a11y_invalid: String,
        a11y_toggled: String,
        a11y_live: String,
        a11y_default_action_verb: String,
        a11y_orientation: String,
        a11y_sort_direction: String,
        a11y_current: String,
        a11y_auto_complete: String,
        a11y_has_popup: String,
        a11y_list_style: String,
        a11y_vertical_offset: String,
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

        a11y_id: String,
        a11y_auto_focus: String,
        a11y_focusable: String,
        a11y_name: String,
        a11y_description: String,
        a11y_value: String,
        a11y_access_key: String,
        a11y_author_id: String,
        a11y_keyboard_shortcut: String,
        a11y_language: String,
        a11y_placeholder: String,
        a11y_role_description: String,
        a11y_state_description: String,
        a11y_tooltip: String,
        a11y_url: String,
        a11y_row_index_text: String,
        a11y_column_index_text: String,
        a11y_scroll_x: String,
        a11y_scroll_x_min: String,
        a11y_scroll_x_max: String,
        a11y_scroll_y: String,
        a11y_scroll_y_min: String,
        a11y_scroll_y_max: String,
        a11y_numeric_value: String,
        a11y_min_numeric_value: String,
        a11y_max_numeric_value: String,
        a11y_numeric_value_step: String,
        a11y_numeric_value_jump: String,
        a11y_row_count: String,
        a11y_column_count: String,
        a11y_row_index: String,
        a11y_column_index: String,
        a11y_row_span: String,
        a11y_column_span: String,
        a11y_level: String,
        a11y_size_of_set: String,
        a11y_position_in_set: String,
        a11y_color_value: String,
        a11y_expanded: String,
        a11y_selected: String,
        a11y_hovered: String,
        a11y_hidden: String,
        a11y_linked: String,
        a11y_multiselectable: String,
        a11y_required: String,
        a11y_visited: String,
        a11y_busy: String,
        a11y_live_atomic: String,
        a11y_modal: String,
        a11y_touch_transparent: String,
        a11y_read_only: String,
        a11y_disabled: String,
        a11y_is_spelling_error: String,
        a11y_is_grammar_error: String,
        a11y_is_search_match: String,
        a11y_is_suggestion: String,
        a11y_role: String,
        a11y_invalid: String,
        a11y_toggled: String,
        a11y_live: String,
        a11y_default_action_verb: String,
        a11y_orientation: String,
        a11y_sort_direction: String,
        a11y_current: String,
        a11y_auto_complete: String,
        a11y_has_popup: String,
        a11y_list_style: String,
        a11y_vertical_offset: String,
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

        a11y_id: String,
        a11y_focusable: String,
        a11y_auto_focus: String,
        a11y_name: String,
        a11y_description: String,
        a11y_value: String,
        a11y_access_key: String,
        a11y_author_id: String,
        a11y_keyboard_shortcut: String,
        a11y_language: String,
        a11y_placeholder: String,
        a11y_role_description: String,
        a11y_state_description: String,
        a11y_tooltip: String,
        a11y_url: String,
        a11y_row_index_text: String,
        a11y_column_index_text: String,
        a11y_scroll_x: String,
        a11y_scroll_x_min: String,
        a11y_scroll_x_max: String,
        a11y_scroll_y: String,
        a11y_scroll_y_min: String,
        a11y_scroll_y_max: String,
        a11y_numeric_value: String,
        a11y_min_numeric_value: String,
        a11y_max_numeric_value: String,
        a11y_numeric_value_step: String,
        a11y_numeric_value_jump: String,
        a11y_row_count: String,
        a11y_column_count: String,
        a11y_row_index: String,
        a11y_column_index: String,
        a11y_row_span: String,
        a11y_column_span: String,
        a11y_level: String,
        a11y_size_of_set: String,
        a11y_position_in_set: String,
        a11y_color_value: String,
        a11y_expanded: String,
        a11y_selected: String,
        a11y_hovered: String,
        a11y_hidden: String,
        a11y_linked: String,
        a11y_multiselectable: String,
        a11y_required: String,
        a11y_visited: String,
        a11y_busy: String,
        a11y_live_atomic: String,
        a11y_modal: String,
        a11y_touch_transparent: String,
        a11y_read_only: String,
        a11y_disabled: String,
        a11y_is_spelling_error: String,
        a11y_is_grammar_error: String,
        a11y_is_search_match: String,
        a11y_is_suggestion: String,
        a11y_role: String,
        a11y_invalid: String,
        a11y_toggled: String,
        a11y_live: String,
        a11y_default_action_verb: String,
        a11y_orientation: String,
        a11y_sort_direction: String,
        a11y_current: String,
        a11y_auto_complete: String,
        a11y_has_popup: String,
        a11y_list_style: String,
        a11y_vertical_offset: String,
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
        #[doc = include_str!("_docs/events/middleclick.md")]
        onmiddleclick
        #[doc = include_str!("_docs/events/rightclick.md")]
        onrightclick
        #[doc = include_str!("_docs/events/mouseup.md")]
        onmouseup
        #[doc = include_str!("_docs/events/mousedown.md")]
        onmousedown
        #[doc = include_str!("_docs/events/globalmousedown.md")]
        onglobalmousedown
        #[doc = include_str!("_docs/events/mousemove.md")]
        onmousemove
        #[doc = include_str!("_docs/events/globalmousemove.md")]
        onglobalmousemove
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

        onkeydown

        onkeyup

        #[doc = include_str!("_docs/events/globalkeydown.md")]
        onglobalkeydown
        #[doc = include_str!("_docs/events/globalkeyup.md")]
        onglobalkeyup
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
        #[doc = include_str!("_docs/events/globalpointerup.md")]
        onglobalpointerup
        #[doc = include_str!("_docs/events/pointermove.md")]
        onpointermove
        #[doc = include_str!("_docs/events/pointerenter.md")]
        onpointerenter
        #[doc = include_str!("_docs/events/pointerleave.md")]
        onpointerleave
    ];

    impl_event! [
        FileData;

        #[doc = include_str!("_docs/events/filedrop.md")]
        onfiledrop
        #[doc = include_str!("_docs/events/globalfilehover.md")]
        onglobalfilehover
        #[doc = include_str!("_docs/events/globalfilehovercancelled.md")]
        onglobalfilehovercancelled
    ];
}
