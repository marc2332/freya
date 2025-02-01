use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum AttributeName {
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    VisibleWidth,
    VisibleHeight,
    Padding,
    Background,
    Border,
    Direction,
    Shadow,
    CornerRadius,
    CornerSmoothing,
    Color,
    Fill,
    Stroke,
    FontSize,
    FontFamily,
    FontStyle,
    FontWeight,
    FontWidth,
    MainAlign,
    CrossAlign,
    TextAlign,
    TextShadow,
    MaxLines,
    LineHeight,
    LetterSpacing,
    WordSpacing,
    Decoration,
    DecorationColor,
    DecorationStyle,
    TextOverflow,
    TextHeight,
    Rotate,
    Overflow,
    Margin,
    Position,
    PositionTop,
    PositionRight,
    PositionBottom,
    PositionLeft,
    Opacity,
    Content,
    CanvasReference,
    Layer,
    OffsetY,
    OffsetX,
    Reference,
    CursorReference,
    CursorIndex,
    CursorColor,
    CursorMode,
    CursorId,
    Highlights,
    HighlightColor,
    HighlightMode,
    ImageReference,
    ImageData,
    SvgData,
    SvgContent,
    Spacing,
    Scale,

    // Image element
    AspectRatio,
    ImageCover,
    ImageCacheKey,

    // Focus
    A11yId,
    A11yFocusable,
    A11yAutoFocus,

    // Some internal notes about these accessibility attributes:
    //
    // - These are mostly derived from AccessKit's [`Node`] struct, with minor
    //   modifications to fit Freya's needs. These modifications are documented.
    //
    // - Some properties are commented out, meaning they are yet to be implemented.
    //   This is typically due to it being unclear how to represent these in Freya's
    //   attribute system (such as the association types, which will likely need
    //   some kind of ID system).
    //
    // - Any AccessKit properties that can be automatically calculated from style
    //   attributes or measured from torin are not included here, and are instead
    //   added in Freya's [`AccessibilityManager`] struct.

    // Vec<NodeIdVec> associations
    // A11yControls,
    // A11yDetails,
    // A11yDescribedBy,
    // A11yFlowTo,
    // A11yLabelledBy,
    // A11yOwns,
    // A11yRadioGroup,

    // NodeId associations
    // ActiveDescendant,
    // A11yErrorMessage,
    // A11yInPageLinkTarget,
    A11yMemberOf,
    // A11yNextOnLine,
    // A11yPreviousOnLine,
    // A11yPopupFor,

    // String
    A11yName,
    A11yDescription,
    A11yValue,
    A11yAccessKey,
    A11yAuthorId,
    // These three attributes are intended for assistive tech that parse MathML,
    // which we don't support at the moment anyways. Unlikely to be implemented.
    // A11yClassName,
    // A11yHtmlTag,
    // A11yInnerHtml,
    A11yKeyboardShortcut,
    A11yLanguage,
    A11yPlaceholder,
    A11yRoleDescription,
    A11yStateDescription,
    A11yTooltip,
    A11yUrl,
    A11yRowIndexText,
    A11yColumnIndexText,

    // f64
    A11yScrollX,
    A11yScrollXMin,
    A11yScrollXMax,
    A11yScrollY,
    A11yScrollYMin,
    A11yScrollYMax,
    A11yNumericValue,
    A11yMinNumericValue,
    A11yMaxNumericValue,
    A11yNumericValueStep,
    A11yNumericValueJump,

    // usize
    A11yRowCount,
    A11yColumnCount,
    A11yRowIndex,
    A11yColumnIndex,
    A11yRowSpan,
    A11yColumnSpan,
    A11yLevel,
    A11ySizeOfSet,
    A11yPositionInSet,

    // Color
    A11yColorValue,

    // TODO: The following two categories are for inline text. They should be implemented
    //       automatically in [`AccessibilityManager`] based on Skia text measurement on text.
    //       spans. These really shouldn't be here (they should never have to be manually provided
    //       as an attribute), but I've left them here as a reminder to implement inline text data.
    //
    // See AccessKit's documentation for inline text measurements here:
    // - <https://docs.rs/accesskit/latest/accesskit/struct.Node.html#method.character_lengths>
    //
    // Chromium also has a good writeup on how it measures inline text spans:
    // - <https://chromium.googlesource.com/chromium/src.git/+/HEAD/docs/accessibility/overview.md#text-bounding-boxes>

    // LengthSlice
    // A11yCharacterLengths,
    // A11yWordLengths,

    // CoordSlice
    // A11yCharacterPositions,
    // A11yCharacterWidths,

    // bool
    A11yExpanded,
    A11ySelected,

    // bitflag
    // TODO: This might be able to be determined automatically,
    //       but i'm not sure what ARIA property it corresponds to
    //       or its actual purpose.
    A11yHovered,
    A11yHidden,
    A11yLinked,
    A11yMultiselectable,
    A11yRequired,
    A11yVisited,
    A11yBusy,
    A11yLiveAtomic,
    A11yModal,
    A11yTouchTransparent,
    A11yReadOnly,
    A11yDisabled,
    A11yIsSpellingError,
    A11yIsGrammarError,
    A11yIsSearchMatch,
    A11yIsSuggestion,

    // Unique enums
    A11yRole,
    A11yInvalid,
    A11yToggled,
    A11yLive,
    A11yDefaultActionVerb,
    A11yOrientation,
    A11ySortDirection,
    A11yCurrent, // called AriaCurrent in accesskit, but that's a pretty poor name
    A11yAutoComplete,
    A11yHasPopup,
    // This one is kind of weird to include, given it's reflecting a CSS property
    // not in Freya for the HTML <ul>/<li> tags, but it can maybe be useful for
    // language-specific semantics.
    A11yListStyle,
    A11yVerticalOffset,
    // Other
    // This could probably be inferred from Freya's text editing hook, but it's also
    // a little strange in the data it expects.
    // A11yTextSelection,
    // A11yCustomActions, // Needs a special syntax or custom attribute value'

    // TODO: Some way to specify builtin AccessKit actions, as well as a way to
    //       handle actions in the form of an event.
}

impl FromStr for AttributeName {
    type Err = String;

    fn from_str(attr: &str) -> Result<Self, Self::Err> {
        match attr {
            "width" => Ok(AttributeName::Width),
            "height" => Ok(AttributeName::Height),
            "min_width" => Ok(AttributeName::MinWidth),
            "min_height" => Ok(AttributeName::MinHeight),
            "max_width" => Ok(AttributeName::MaxWidth),
            "max_height" => Ok(AttributeName::MaxHeight),
            "visible_width" => Ok(AttributeName::VisibleWidth),
            "visible_height" => Ok(AttributeName::VisibleHeight),
            "padding" => Ok(AttributeName::Padding),
            "background" => Ok(AttributeName::Background),
            "border" => Ok(AttributeName::Border),
            "direction" => Ok(AttributeName::Direction),
            "shadow" => Ok(AttributeName::Shadow),
            "corner_radius" => Ok(AttributeName::CornerRadius),
            "corner_smoothing" => Ok(AttributeName::CornerSmoothing),
            "color" => Ok(AttributeName::Color),
            "fill" => Ok(AttributeName::Fill),
            "stroke" => Ok(AttributeName::Stroke),
            "font_size" => Ok(AttributeName::FontSize),
            "font_family" => Ok(AttributeName::FontFamily),
            "font_style" => Ok(AttributeName::FontStyle),
            "font_weight" => Ok(AttributeName::FontWeight),
            "font_width" => Ok(AttributeName::FontWidth),
            "main_align" => Ok(AttributeName::MainAlign),
            "cross_align" => Ok(AttributeName::CrossAlign),
            "text_align" => Ok(AttributeName::TextAlign),
            "text_shadow" => Ok(AttributeName::TextShadow),
            "max_lines" => Ok(AttributeName::MaxLines),
            "line_height" => Ok(AttributeName::LineHeight),
            "letter_spacing" => Ok(AttributeName::LetterSpacing),
            "word_spacing" => Ok(AttributeName::WordSpacing),
            "decoration" => Ok(AttributeName::Decoration),
            "decoration_color" => Ok(AttributeName::DecorationColor),
            "decoration_style" => Ok(AttributeName::DecorationStyle),
            "text_overflow" => Ok(AttributeName::TextOverflow),
            "text_height" => Ok(AttributeName::TextHeight),
            "rotate" => Ok(AttributeName::Rotate),
            "overflow" => Ok(AttributeName::Overflow),
            "margin" => Ok(AttributeName::Margin),
            "position" => Ok(AttributeName::Position),
            "position_top" => Ok(AttributeName::PositionTop),
            "position_right" => Ok(AttributeName::PositionRight),
            "position_bottom" => Ok(AttributeName::PositionBottom),
            "position_left" => Ok(AttributeName::PositionLeft),
            "opacity" => Ok(AttributeName::Opacity),
            "content" => Ok(AttributeName::Content),
            "canvas_reference" => Ok(AttributeName::CanvasReference),
            "layer" => Ok(AttributeName::Layer),
            "offset_y" => Ok(AttributeName::OffsetY),
            "offset_x" => Ok(AttributeName::OffsetX),
            "reference" => Ok(AttributeName::Reference),
            "cursor_reference" => Ok(AttributeName::CursorReference),
            "cursor_index" => Ok(AttributeName::CursorIndex),
            "cursor_color" => Ok(AttributeName::CursorColor),
            "cursor_mode" => Ok(AttributeName::CursorMode),
            "cursor_id" => Ok(AttributeName::CursorId),
            "highlights" => Ok(AttributeName::Highlights),
            "highlight_color" => Ok(AttributeName::HighlightColor),
            "highlight_mode" => Ok(AttributeName::HighlightMode),
            "image_reference" => Ok(AttributeName::ImageReference),
            "image_data" => Ok(AttributeName::ImageData),
            "svg_data" => Ok(AttributeName::SvgData),
            "svg_content" => Ok(AttributeName::SvgContent),
            "spacing" => Ok(AttributeName::Spacing),
            "scale" => Ok(AttributeName::Scale),
            "aspect_ratio" => Ok(AttributeName::AspectRatio),
            "cover" => Ok(AttributeName::ImageCover),
            "cache_key" => Ok(AttributeName::ImageCacheKey),
            "a11y_id" => Ok(AttributeName::A11yId),
            "a11y_focusable" => Ok(AttributeName::A11yFocusable),
            "a11y_auto_focus" => Ok(AttributeName::A11yAutoFocus),
            "a11y_name" => Ok(AttributeName::A11yName),
            "a11y_description" => Ok(AttributeName::A11yDescription),
            "a11y_value" => Ok(AttributeName::A11yValue),
            "a11y_access_key" => Ok(AttributeName::A11yAccessKey),
            "a11y_author_id" => Ok(AttributeName::A11yAuthorId),
            "a11y_keyboard_shortcut" => Ok(AttributeName::A11yKeyboardShortcut),
            "a11y_language" => Ok(AttributeName::A11yLanguage),
            "a11y_placeholder" => Ok(AttributeName::A11yPlaceholder),
            "a11y_role_description" => Ok(AttributeName::A11yRoleDescription),
            "a11y_state_description" => Ok(AttributeName::A11yStateDescription),
            "a11y_tooltip" => Ok(AttributeName::A11yTooltip),
            "a11y_url" => Ok(AttributeName::A11yUrl),
            "a11y_row_index_text" => Ok(AttributeName::A11yRowIndexText),
            "a11y_column_index_text" => Ok(AttributeName::A11yColumnIndexText),
            "a11y_scroll_x" => Ok(AttributeName::A11yScrollX),
            "a11y_scroll_x_min" => Ok(AttributeName::A11yScrollXMin),
            "a11y_scroll_x_max" => Ok(AttributeName::A11yScrollXMax),
            "a11y_scroll_y" => Ok(AttributeName::A11yScrollY),
            "a11y_scroll_y_min" => Ok(AttributeName::A11yScrollYMin),
            "a11y_scroll_y_max" => Ok(AttributeName::A11yScrollYMax),
            "a11y_numeric_value" => Ok(AttributeName::A11yNumericValue),
            "a11y_min_numeric_value" => Ok(AttributeName::A11yMinNumericValue),
            "a11y_max_numeric_value" => Ok(AttributeName::A11yMaxNumericValue),
            "a11y_numeric_value_step" => Ok(AttributeName::A11yNumericValueStep),
            "a11y_numeric_value_jump" => Ok(AttributeName::A11yNumericValueJump),
            "a11y_row_count" => Ok(AttributeName::A11yRowCount),
            "a11y_column_count" => Ok(AttributeName::A11yColumnCount),
            "a11y_row_index" => Ok(AttributeName::A11yRowIndex),
            "a11y_column_index" => Ok(AttributeName::A11yColumnIndex),
            "a11y_row_span" => Ok(AttributeName::A11yRowSpan),
            "a11y_column_span" => Ok(AttributeName::A11yColumnSpan),
            "a11y_level" => Ok(AttributeName::A11yLevel),
            "a11y_size_of_set" => Ok(AttributeName::A11ySizeOfSet),
            "a11y_position_in_set" => Ok(AttributeName::A11yPositionInSet),
            "a11y_color_value" => Ok(AttributeName::A11yColorValue),
            "a11y_expanded" => Ok(AttributeName::A11yExpanded),
            "a11y_selected" => Ok(AttributeName::A11ySelected),
            "a11y_hovered" => Ok(AttributeName::A11yHovered),
            "a11y_hidden" => Ok(AttributeName::A11yHidden),
            "a11y_linked" => Ok(AttributeName::A11yLinked),
            "a11y_multiselectable" => Ok(AttributeName::A11yMultiselectable),
            "a11y_required" => Ok(AttributeName::A11yRequired),
            "a11y_visited" => Ok(AttributeName::A11yVisited),
            "a11y_busy" => Ok(AttributeName::A11yBusy),
            "a11y_live_atomic" => Ok(AttributeName::A11yLiveAtomic),
            "a11y_modal" => Ok(AttributeName::A11yModal),
            "a11y_touch_transparent" => Ok(AttributeName::A11yTouchTransparent),
            "a11y_read_only" => Ok(AttributeName::A11yReadOnly),
            "a11y_disabled" => Ok(AttributeName::A11yDisabled),
            "a11y_is_spelling_error" => Ok(AttributeName::A11yIsSpellingError),
            "a11y_is_grammar_error" => Ok(AttributeName::A11yIsGrammarError),
            "a11y_is_search_match" => Ok(AttributeName::A11yIsSearchMatch),
            "a11y_is_suggestion" => Ok(AttributeName::A11yIsSuggestion),
            "a11y_role" => Ok(AttributeName::A11yRole),
            "a11y_invalid" => Ok(AttributeName::A11yInvalid),
            "a11y_toggled" => Ok(AttributeName::A11yToggled),
            "a11y_live" => Ok(AttributeName::A11yLive),
            "a11y_default_action_verb" => Ok(AttributeName::A11yDefaultActionVerb),
            "a11y_orientation" => Ok(AttributeName::A11yOrientation),
            "a11y_sort_direction" => Ok(AttributeName::A11ySortDirection),
            "a11y_current" => Ok(AttributeName::A11yCurrent),
            "a11y_auto_complete" => Ok(AttributeName::A11yAutoComplete),
            "a11y_has_popup" => Ok(AttributeName::A11yHasPopup),
            "a11y_list_style" => Ok(AttributeName::A11yListStyle),
            "a11y_vertical_offset" => Ok(AttributeName::A11yVerticalOffset),
            "a11y_member_of" => Ok(AttributeName::A11yMemberOf),
            _ => Err(format!("{attr} not supported.")),
        }
    }
}
