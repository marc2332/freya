use freya_core::prelude::*;
#[cfg(feature = "titlebar")]
use torin::prelude::Length;
use torin::{
    gaps::Gaps,
    size::Size,
};

#[cfg(feature = "calendar")]
use crate::calendar::CalendarThemePreference;
#[cfg(feature = "router")]
use crate::link::LinkThemePreference;
#[cfg(feature = "markdown")]
use crate::markdown::MarkdownViewerThemePreference;
#[cfg(feature = "titlebar")]
use crate::titlebar::TitlebarButtonThemePreference;
use crate::{
    accordion::AccordionThemePreference,
    button::{
        ButtonColorsThemePreference,
        ButtonLayoutThemePreference,
    },
    card::{
        CardColorsThemePreference,
        CardLayoutThemePreference,
    },
    checkbox::CheckboxThemePreference,
    chip::ChipThemePreference,
    color_picker::ColorPickerThemePreference,
    floating_tab::FloatingTabThemePreference,
    input::{
        InputColorsThemePreference,
        InputLayoutThemePreference,
    },
    loader::CircularLoaderThemePreference,
    menu::{
        MenuContainerThemePreference,
        MenuItemThemePreference,
    },
    popup::PopupThemePreference,
    progressbar::ProgressBarThemePreference,
    radio_item::RadioItemThemePreference,
    resizable_container::ResizableHandleThemePreference,
    scrollviews::ScrollBarThemePreference,
    segmented_button::{
        ButtonSegmentThemePreference,
        SegmentedButtonThemePreference,
    },
    select::SelectThemePreference,
    sidebar::SideBarItemThemePreference,
    slider::SliderThemePreference,
    switch::{
        SwitchColorsThemePreference,
        SwitchLayoutThemePreference,
    },
    table::TableThemePreference,
    theming::{
        component_themes::{
            ColorsSheet,
            Theme,
        },
        macros::Preference,
    },
    tooltip::TooltipThemePreference,
};

pub const LIGHT_COLORS: ColorsSheet = ColorsSheet {
    // Brand & Accent
    primary: Color::from_rgb(103, 80, 164),
    secondary: Color::from_rgb(202, 193, 227),
    tertiary: Color::from_rgb(79, 61, 130),

    // Status
    success: Color::from_rgb(76, 175, 80),
    warning: Color::from_rgb(255, 193, 7),
    error: Color::from_rgb(244, 67, 54),
    info: Color::from_rgb(33, 150, 243),

    // Surfaces
    background: Color::from_rgb(250, 250, 250),
    surface_primary: Color::from_rgb(210, 210, 210),
    surface_secondary: Color::from_rgb(225, 225, 225),
    surface_tertiary: Color::from_rgb(245, 245, 245),
    surface_inverse: Color::from_rgb(125, 125, 125),
    surface_inverse_secondary: Color::from_rgb(110, 110, 110),
    surface_inverse_tertiary: Color::from_rgb(90, 90, 90),

    // Borders
    border: Color::from_rgb(210, 210, 210),
    border_focus: Color::from_rgb(180, 180, 180),
    border_disabled: Color::from_rgb(210, 210, 210),

    // Text
    text_primary: Color::from_rgb(10, 10, 10),
    text_secondary: Color::from_rgb(100, 100, 100),
    text_placeholder: Color::from_rgb(150, 150, 150),
    text_inverse: Color::WHITE,
    text_highlight: Color::from_rgb(38, 89, 170),

    // States
    hover: Color::from_rgb(235, 235, 235),
    focus: Color::from_rgb(225, 225, 255),
    active: Color::from_rgb(200, 200, 200),
    disabled: Color::from_rgb(210, 210, 210),

    // Utility
    overlay: Color::from_af32rgb(0.5, 0, 0, 0),
    shadow: Color::from_af32rgb(0.2, 0, 0, 0),
};

pub const DARK_COLORS: ColorsSheet = ColorsSheet {
    // Brand & Accent
    primary: Color::from_rgb(103, 80, 164),
    secondary: Color::from_rgb(202, 193, 227),
    tertiary: Color::from_rgb(79, 61, 130),

    // Status
    success: Color::from_rgb(129, 199, 132),
    warning: Color::from_rgb(255, 213, 79),
    error: Color::from_rgb(229, 115, 115),
    info: Color::from_rgb(100, 181, 246),

    // Surfaces
    background: Color::from_rgb(20, 20, 20),
    surface_primary: Color::from_rgb(60, 60, 60),
    surface_secondary: Color::from_rgb(45, 45, 45),
    surface_tertiary: Color::from_rgb(25, 25, 25),
    surface_inverse: Color::from_rgb(135, 135, 135),
    surface_inverse_secondary: Color::from_rgb(150, 150, 150),
    surface_inverse_tertiary: Color::from_rgb(170, 170, 170),

    // Borders
    border: Color::from_rgb(60, 60, 60),
    border_focus: Color::from_rgb(110, 110, 110),
    border_disabled: Color::from_rgb(80, 80, 80),

    // Text
    text_primary: Color::from_rgb(250, 250, 250),
    text_secondary: Color::from_rgb(210, 210, 210),
    text_placeholder: Color::from_rgb(150, 150, 150),
    text_inverse: Color::WHITE,
    text_highlight: Color::from_rgb(96, 145, 224),

    // States
    hover: Color::from_rgb(80, 80, 80),
    focus: Color::from_rgb(100, 100, 120),
    active: Color::from_rgb(70, 70, 70),
    disabled: Color::from_rgb(50, 50, 50),

    // Utility
    overlay: Color::from_af32rgb(0.2, 255, 255, 255),
    shadow: Color::from_af32rgb(0.6, 0, 0, 0),
};

fn register_base_component_themes(theme: &mut Theme) {
    theme.set(
        "button_layout",
        ButtonLayoutThemePreference {
            padding: Preference::Specific(Gaps::new(6., 12., 6., 12.)),
            margin: Preference::Specific(Gaps::new_all(0.)),
            corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
        },
    );
    theme.set(
        "compact_button_layout",
        ButtonLayoutThemePreference {
            padding: Preference::Specific(Gaps::new(3., 6., 3., 6.)),
            margin: Preference::Specific(Gaps::new_all(0.)),
            corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
        },
    );
    theme.set(
        "expanded_button_layout",
        ButtonLayoutThemePreference {
            padding: Preference::Specific(Gaps::new(10., 16., 10., 16.)),
            margin: Preference::Specific(Gaps::new_all(0.)),
            corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
        },
    );
    theme.set(
        "button",
        ButtonColorsThemePreference {
            background: Preference::Reference("surface_tertiary"),
            hover_background: Preference::Reference("hover"),
            border_fill: Preference::Reference("border"),
            focus_border_fill: Preference::Reference("border_focus"),
            color: Preference::Reference("text_primary"),
        },
    );
    theme.set(
        "filled_button",
        ButtonColorsThemePreference {
            background: Preference::Reference("primary"),
            hover_background: Preference::Reference("tertiary"),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            focus_border_fill: Preference::Reference("secondary"),
            color: Preference::Reference("text_inverse"),
        },
    );
    theme.set(
        "outline_button",
        ButtonColorsThemePreference {
            background: Preference::Reference("surface_tertiary"),
            hover_background: Preference::Reference("hover"),
            border_fill: Preference::Reference("border"),
            focus_border_fill: Preference::Reference("secondary"),
            color: Preference::Reference("primary"),
        },
    );
    theme.set(
        "flat_button",
        ButtonColorsThemePreference {
            background: Preference::Specific(Color::TRANSPARENT),
            hover_background: Preference::Reference("surface_tertiary"),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            focus_border_fill: Preference::Reference("border"),
            color: Preference::Reference("text_primary"),
        },
    );
    theme.set(
        "card_layout",
        CardLayoutThemePreference {
            padding: Preference::Specific(Gaps::new(16., 16., 16., 16.)),
            corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
        },
    );
    theme.set(
        "compact_card_layout",
        CardLayoutThemePreference {
            padding: Preference::Specific(Gaps::new(8., 12., 8., 12.)),
            corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
        },
    );
    theme.set(
        "filled_card",
        CardColorsThemePreference {
            background: Preference::Reference("primary"),
            hover_background: Preference::Reference("tertiary"),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            color: Preference::Reference("text_inverse"),
            shadow: Preference::Reference("shadow"),
        },
    );
    theme.set(
        "outline_card",
        CardColorsThemePreference {
            background: Preference::Reference("surface_tertiary"),
            hover_background: Preference::Reference("hover"),
            border_fill: Preference::Reference("border"),
            color: Preference::Reference("text_primary"),
            shadow: Preference::Reference("shadow"),
        },
    );
    theme.set(
        "accordion",
        AccordionThemePreference {
            color: Preference::Reference("text_primary"),
            background: Preference::Reference("surface_tertiary"),
            border_fill: Preference::Reference("border"),
        },
    );
    theme.set(
        "switch",
        SwitchColorsThemePreference {
            background: Preference::Reference("surface_secondary"),
            thumb_background: Preference::Reference("surface_inverse"),
            toggled_background: Preference::Reference("secondary"),
            toggled_thumb_background: Preference::Reference("primary"),
            focus_border_fill: Preference::Reference("border_focus"),
        },
    );
    theme.set(
        "switch_layout",
        SwitchLayoutThemePreference {
            margin: Preference::Specific(Gaps::new_all(0.)),
            width: Preference::Specific(48.),
            height: Preference::Specific(28.),
            padding: Preference::Specific(4.),
            thumb_size: Preference::Specific(16.),
            toggled_thumb_size: Preference::Specific(20.),
            thumb_offset: Preference::Specific(2.),
            toggled_thumb_offset: Preference::Specific(20.),
        },
    );
    theme.set(
        "expanded_switch_layout",
        SwitchLayoutThemePreference {
            margin: Preference::Specific(Gaps::new_all(0.)),
            width: Preference::Specific(56.),
            height: Preference::Specific(32.),
            padding: Preference::Specific(4.),
            thumb_size: Preference::Specific(18.),
            toggled_thumb_size: Preference::Specific(22.),
            thumb_offset: Preference::Specific(2.),
            toggled_thumb_offset: Preference::Specific(26.),
        },
    );
    theme.set(
        "scrollbar",
        ScrollBarThemePreference {
            background: Preference::Reference("surface_primary"),
            thumb_background: Preference::Reference("surface_inverse"),
            hover_thumb_background: Preference::Reference("surface_inverse_secondary"),
            active_thumb_background: Preference::Reference("surface_inverse_tertiary"),
            size: Preference::Specific(15.),
        },
    );
    theme.set(
        "progressbar",
        ProgressBarThemePreference {
            color: Preference::Reference("text_inverse"),
            background: Preference::Reference("surface_primary"),
            progress_background: Preference::Reference("primary"),
            height: Preference::Specific(20.),
        },
    );
    theme.set(
        "sidebar_item",
        SideBarItemThemePreference {
            color: Preference::Reference("text_primary"),
            background: Preference::Reference("surface_tertiary"),
            active_background: Preference::Reference("surface_secondary"),
            hover_background: Preference::Reference("hover"),
            corner_radius: Preference::Specific(CornerRadius::new_all(12.)),
            margin: Preference::Specific(Gaps::new_all(0.)),
            padding: Preference::Specific(Gaps::new(8., 12., 8., 12.)),
        },
    );
    #[cfg(feature = "router")]
    theme.set(
        "link",
        LinkThemePreference {
            color: Preference::Reference("text_highlight"),
        },
    );
    theme.set(
        "tooltip",
        TooltipThemePreference {
            background: Preference::Reference("surface_tertiary"),
            color: Preference::Reference("text_primary"),
            border_fill: Preference::Reference("surface_primary"),
            font_size: Preference::Specific(14.),
        },
    );
    theme.set(
        "circular_loader",
        CircularLoaderThemePreference {
            primary_color: Preference::Reference("surface_primary"),
            inversed_color: Preference::Reference("surface_inverse"),
        },
    );
    theme.set(
        "input_layout",
        InputLayoutThemePreference {
            corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
            inner_margin: Preference::Specific(Gaps::new(8., 8., 8., 8.)),
        },
    );
    theme.set(
        "compact_input_layout",
        InputLayoutThemePreference {
            corner_radius: Preference::Specific(CornerRadius::new_all(4.)),
            inner_margin: Preference::Specific(Gaps::new(4., 6., 4., 6.)),
        },
    );
    theme.set(
        "expanded_input_layout",
        InputLayoutThemePreference {
            corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
            inner_margin: Preference::Specific(Gaps::new(12., 12., 12., 12.)),
        },
    );
    theme.set(
        "input",
        InputColorsThemePreference {
            background: Preference::Reference("surface_tertiary"),
            hover_background: Preference::Reference("background"),
            color: Preference::Reference("text_primary"),
            placeholder_color: Preference::Reference("text_secondary"),
            border_fill: Preference::Reference("border"),
            focus_border_fill: Preference::Reference("border_focus"),
        },
    );
    theme.set(
        "filled_input",
        InputColorsThemePreference {
            background: Preference::Reference("primary"),
            hover_background: Preference::Reference("tertiary"),
            color: Preference::Reference("text_inverse"),
            placeholder_color: Preference::Reference("text_inverse"),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            focus_border_fill: Preference::Reference("secondary"),
        },
    );
    theme.set(
        "flat_input",
        InputColorsThemePreference {
            background: Preference::Specific(Color::TRANSPARENT),
            hover_background: Preference::Reference("surface_tertiary"),
            color: Preference::Reference("text_primary"),
            placeholder_color: Preference::Reference("text_secondary"),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            focus_border_fill: Preference::Reference("border"),
        },
    );
    theme.set(
        "radio",
        RadioItemThemePreference {
            unselected_fill: Preference::Reference("surface_inverse_tertiary"),
            selected_fill: Preference::Reference("primary"),
            border_fill: Preference::Reference("surface_primary"),
        },
    );
    theme.set(
        "checkbox",
        CheckboxThemePreference {
            unselected_fill: Preference::Reference("surface_inverse_tertiary"),
            selected_fill: Preference::Reference("primary"),
            selected_icon_fill: Preference::Reference("secondary"),
            border_fill: Preference::Reference("surface_primary"),
        },
    );
    theme.set(
        "resizable_handle",
        ResizableHandleThemePreference {
            background: Preference::Reference("surface_secondary"),
            hover_background: Preference::Reference("surface_primary"),
            corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
        },
    );
    theme.set(
        "floating_tab",
        FloatingTabThemePreference {
            background: Preference::Specific(Color::TRANSPARENT),
            hover_background: Preference::Reference("surface_secondary"),
            color: Preference::Reference("text_primary"),
            padding: Preference::Specific(Gaps::new(6., 12., 6., 12.)),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
            corner_radius: Preference::Specific(CornerRadius::new_all(99.)),
        },
    );
    theme.set(
        "slider",
        SliderThemePreference {
            background: Preference::Reference("surface_primary"),
            thumb_background: Preference::Reference("secondary"),
            thumb_inner_background: Preference::Reference("primary"),
            border_fill: Preference::Reference("surface_primary"),
        },
    );
    theme.set(
        "color_picker",
        ColorPickerThemePreference {
            background: Preference::Reference("surface_tertiary"),
            border_fill: Preference::Reference("border"),
            color: Preference::Reference("text_primary"),
        },
    );
    theme.set(
        "select",
        SelectThemePreference {
            width: Preference::Specific(Size::Inner),
            margin: Preference::Specific(Gaps::new_all(0.)),
            select_background: Preference::Reference("background"),
            background_button: Preference::Reference("surface_tertiary"),
            hover_background: Preference::Reference("hover"),
            color: Preference::Reference("text_primary"),
            border_fill: Preference::Reference("border"),
            focus_border_fill: Preference::Reference("border_focus"),
            arrow_fill: Preference::Reference("text_primary"),
        },
    );
    theme.set(
        "popup",
        PopupThemePreference {
            background: Preference::Reference("background"),
            color: Preference::Reference("text_primary"),
        },
    );
    theme.set(
        "table",
        TableThemePreference {
            background: Preference::Reference("background"),
            arrow_fill: Preference::Reference("text_primary"),
            row_background: Preference::Specific(Color::TRANSPARENT),
            hover_row_background: Preference::Reference("surface_secondary"),
            divider_fill: Preference::Reference("surface_primary"),
            corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
            color: Preference::Reference("text_primary"),
        },
    );
    #[cfg(feature = "markdown")]
    theme.set(
        "markdown_viewer",
        MarkdownViewerThemePreference {
            color: Preference::Reference("text_primary"),
            background_code: Preference::Reference("surface_tertiary"),
            color_code: Preference::Reference("text_primary"),
            background_blockquote: Preference::Reference("surface_tertiary"),
            border_blockquote: Preference::Reference("surface_primary"),
            background_divider: Preference::Reference("border"),
            heading_h1: Preference::Specific(32.0),
            heading_h2: Preference::Specific(28.0),
            heading_h3: Preference::Specific(24.0),
            heading_h4: Preference::Specific(20.0),
            heading_h5: Preference::Specific(18.0),
            heading_h6: Preference::Specific(16.0),
            paragraph_size: Preference::Specific(16.0),
            code_font_size: Preference::Specific(14.0),
            table_font_size: Preference::Specific(14.0),
        },
    );
    theme.set(
        "chip",
        ChipThemePreference {
            background: Preference::Reference("background"),
            hover_background: Preference::Reference("tertiary"),
            selected_background: Preference::Reference("primary"),
            border_fill: Preference::Reference("border"),
            hover_border_fill: Preference::Reference("tertiary"),
            selected_border_fill: Preference::Reference("primary"),
            focus_border_fill: Preference::Reference("secondary"),
            padding: Preference::Specific(Gaps::new(8., 14., 8., 14.)),
            margin: Preference::Specific(0.),
            corner_radius: Preference::Specific(CornerRadius::new_all(99.)),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
            color: Preference::Reference("text_primary"),
            hover_color: Preference::Reference("text_inverse"),
            selected_color: Preference::Reference("text_inverse"),
            selected_icon_fill: Preference::Reference("secondary"),
            hover_icon_fill: Preference::Reference("secondary"),
        },
    );
    theme.set(
        "menu_item",
        MenuItemThemePreference {
            background: Preference::Specific(Color::TRANSPARENT),
            hover_background: Preference::Reference("surface_secondary"),
            select_background: Preference::Reference("surface_secondary"),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            select_border_fill: Preference::Reference("border_focus"),
            corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
            color: Preference::Reference("text_primary"),
        },
    );
    theme.set(
        "menu_container",
        MenuContainerThemePreference {
            background: Preference::Reference("background"),
            padding: Preference::Specific(Gaps::new_all(4.)),
            shadow: Preference::Reference("shadow"),
            border_fill: Preference::Reference("surface_primary"),
            corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
        },
    );
    theme.set(
        "button_segment",
        ButtonSegmentThemePreference {
            background: Preference::Reference("surface_tertiary"),
            hover_background: Preference::Reference("hover"),
            disabled_background: Preference::Reference("disabled"),
            selected_background: Preference::Reference("hover"),
            focus_background: Preference::Reference("surface_secondary"),
            padding: Preference::Specific(Gaps::new(8., 16., 8., 16.)),
            selected_padding: Preference::Specific(Gaps::new(8., 12., 8., 12.)),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
            color: Preference::Reference("text_primary"),
            selected_icon_fill: Preference::Reference("primary"),
        },
    );
    theme.set(
        "segmented_button",
        SegmentedButtonThemePreference {
            background: Preference::Reference("surface_tertiary"),
            border_fill: Preference::Reference("border"),
            corner_radius: Preference::Specific(CornerRadius::new_all(99.)),
        },
    );
    #[cfg(feature = "calendar")]
    theme.set(
        "calendar",
        CalendarThemePreference {
            background: Preference::Reference("surface_tertiary"),
            day_background: Preference::Specific(Color::TRANSPARENT),
            day_hover_background: Preference::Reference("hover"),
            day_selected_background: Preference::Reference("surface_primary"),
            color: Preference::Reference("text_primary"),
            day_other_month_color: Preference::Reference("text_placeholder"),
            header_color: Preference::Reference("text_primary"),
            corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
            padding: Preference::Specific(Gaps::new_all(12.)),
            day_corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
            nav_button_hover_background: Preference::Reference("hover"),
        },
    );
    #[cfg(feature = "titlebar")]
    theme.set(
        "titlebar_button",
        TitlebarButtonThemePreference {
            background: Preference::Specific(Color::TRANSPARENT),
            hover_background: Preference::Reference("hover"),
            corner_radius: Preference::Specific(CornerRadius::new_all(0.0)),
            width: Preference::Specific(Size::Pixels(Length::new(46.0))),
            height: Preference::Specific(Size::Fill),
        },
    );
}

/// Light theme with all built-in component themes registered.
pub fn light_theme() -> Theme {
    let mut theme = Theme::new("light", LIGHT_COLORS);
    register_base_component_themes(&mut theme);
    theme
}

/// Dark theme with all built-in component themes registered.
pub fn dark_theme() -> Theme {
    let mut theme = Theme::new("dark", DARK_COLORS);
    register_base_component_themes(&mut theme);
    theme
}
