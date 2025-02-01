use crate::def_element;

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
        visible_width,
        visible_height,
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
        scale,

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
        a11y_member_of,
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
        aspect_ratio,
        cover,
        cache_key,

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
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// static FERRIS: &[u8] = include_bytes!("_docs/ferris.svg");
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
