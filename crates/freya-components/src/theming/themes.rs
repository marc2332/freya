use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    size::Size,
};

#[cfg(feature = "router")]
use crate::theming::component_themes::LinkThemePreference;
use crate::theming::{
    component_themes::{
        AccordionThemePreference,
        ButtonColorsThemePreference,
        ButtonLayoutThemePreference,
        CheckboxThemePreference,
        ChipThemePreference,
        CircularLoaderThemePreference,
        ColorsSheet,
        DropdownItemThemePreference,
        DropdownThemePreference,
        FloatingTabThemePreference,
        InputThemePreference,
        PopupThemePreference,
        ProgressBarThemePreference,
        RadioItemThemePreference,
        ResizableHandleThemePreference,
        ScrollBarThemePreference,
        SideBarItemThemePreference,
        SideBarThemePreference,
        SliderThemePreference,
        SwitchThemePreference,
        TableThemePreference,
        Theme,
        TooltipThemePreference,
    },
    macros::Preference,
};

pub(crate) const BASE_THEME: Theme = Theme {
    name: "base",
    colors: ColorsSheet {
        // Brand & Accent
        primary: Color::TRANSPARENT,
        secondary: Color::TRANSPARENT,
        tertiary: Color::TRANSPARENT,

        // Status
        success: Color::TRANSPARENT,
        warning: Color::TRANSPARENT,
        error: Color::TRANSPARENT,
        info: Color::TRANSPARENT,

        // Surfaces
        background: Color::TRANSPARENT,
        surface_primary: Color::TRANSPARENT,
        surface_secondary: Color::TRANSPARENT,
        surface_tertiary: Color::TRANSPARENT,
        surface_inverse: Color::TRANSPARENT,
        surface_inverse_secondary: Color::TRANSPARENT,
        surface_inverse_tertiary: Color::TRANSPARENT,

        // Borders
        border: Color::TRANSPARENT,
        border_focus: Color::TRANSPARENT,
        border_disabled: Color::TRANSPARENT,

        // Text
        text_primary: Color::TRANSPARENT,
        text_secondary: Color::TRANSPARENT,
        text_placeholder: Color::TRANSPARENT,
        text_inverse: Color::TRANSPARENT,
        text_highlight: Color::TRANSPARENT,

        // States
        hover: Color::TRANSPARENT,
        focus: Color::TRANSPARENT,
        active: Color::TRANSPARENT,
        disabled: Color::TRANSPARENT,

        // Utility
        overlay: Color::TRANSPARENT,
        shadow: Color::TRANSPARENT,
    },
    button_layout: ButtonLayoutThemePreference {
        padding: Preference::Specific(Gaps::new(6., 12., 6., 12.)),
        margin: Preference::Specific(Gaps::new_all(0.)),
        corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
        width: Preference::Specific(Size::Inner),
        height: Preference::Specific(Size::Inner),
    },
    compact_button_layout: ButtonLayoutThemePreference {
        padding: Preference::Specific(Gaps::new(3., 6., 3., 6.)),
        margin: Preference::Specific(Gaps::new_all(0.)),
        corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
        width: Preference::Specific(Size::Inner),
        height: Preference::Specific(Size::Inner),
    },
    expanded_button_layout: ButtonLayoutThemePreference {
        padding: Preference::Specific(Gaps::new(10., 16., 10., 16.)),
        margin: Preference::Specific(Gaps::new_all(0.)),
        corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
        width: Preference::Specific(Size::Inner),
        height: Preference::Specific(Size::Inner),
    },
    button: ButtonColorsThemePreference {
        background: Preference::Reference("surface_tertiary"),
        hover_background: Preference::Reference("hover"),
        border_fill: Preference::Reference("border"),
        focus_border_fill: Preference::Reference("border_focus"),
        color: Preference::Reference("text_primary"),
    },
    filled_button: ButtonColorsThemePreference {
        background: Preference::Reference("primary"),
        hover_background: Preference::Reference("tertiary"),
        border_fill: Preference::Specific(Color::TRANSPARENT),
        focus_border_fill: Preference::Reference("secondary"),
        color: Preference::Reference("text_inverse"),
    },
    outline_button: ButtonColorsThemePreference {
        background: Preference::Reference("surface_tertiary"),
        hover_background: Preference::Reference("hover"),
        border_fill: Preference::Reference("border"),
        focus_border_fill: Preference::Reference("secondary"),
        color: Preference::Reference("primary"),
    },
    accordion: AccordionThemePreference {
        color: Preference::Reference("text_primary"),
        background: Preference::Reference("surface_tertiary"),
        border_fill: Preference::Reference("border"),
    },
    switch: SwitchThemePreference {
        margin: Preference::Specific(Gaps::new_all(0.)),
        background: Preference::Reference("surface_secondary"),
        thumb_background: Preference::Reference("surface_inverse"),
        toggled_background: Preference::Reference("secondary"),
        toggled_thumb_background: Preference::Reference("primary"),
        focus_border_fill: Preference::Reference("border_focus"),
    },
    scrollbar: ScrollBarThemePreference {
        background: Preference::Reference("surface_secondary"),
        thumb_background: Preference::Reference("surface_inverse"),
        hover_thumb_background: Preference::Reference("surface_inverse_secondary"),
        active_thumb_background: Preference::Reference("surface_inverse_tertiary"),
        size: Preference::Specific(15.),
    },
    progressbar: ProgressBarThemePreference {
        color: Preference::Reference("text_inverse"),
        background: Preference::Reference("surface_primary"),
        progress_background: Preference::Reference("primary"),
        height: Preference::Specific(20.),
    },
    sidebar: SideBarThemePreference {
        color: Preference::Reference("text_primary"),
        background: Preference::Reference("surface_tertiary"),
        padding: Preference::Specific(Gaps::new_all(8.)),
        spacing: Preference::Specific(4.),
    },
    sidebar_item: SideBarItemThemePreference {
        color: Preference::Reference("text_primary"),
        background: Preference::Reference("surface_tertiary"),
        hover_background: Preference::Reference("hover"),
        corner_radius: Preference::Specific(CornerRadius::new_all(99.)),
        margin: Preference::Specific(Gaps::new_all(0.)),
        padding: Preference::Specific(Gaps::new(8., 12., 8., 12.)),
    },
    #[cfg(feature = "router")]
    link: LinkThemePreference {
        color: Preference::Reference("text_highlight"),
    },
    tooltip: TooltipThemePreference {
        background: Preference::Reference("surface_tertiary"),
        color: Preference::Reference("text_primary"),
        border_fill: Preference::Reference("surface_primary"),
    },
    circular_loader: CircularLoaderThemePreference {
        primary_color: Preference::Reference("surface_primary"),
        inversed_color: Preference::Reference("surface_inverse"),
    },
    input: InputThemePreference {
        background: Preference::Reference("surface_tertiary"),
        hover_background: Preference::Reference("background"),
        color: Preference::Reference("text_primary"),
        placeholder_color: Preference::Reference("text_secondary"),
        border_fill: Preference::Reference("border"),
        focus_border_fill: Preference::Reference("border_focus"),
        corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
        inner_margin: Preference::Specific(Gaps::new(8., 8., 8., 8.)),
    },
    radio: RadioItemThemePreference {
        unselected_fill: Preference::Reference("surface_inverse_tertiary"),
        selected_fill: Preference::Reference("primary"),
        border_fill: Preference::Reference("surface_primary"),
    },
    checkbox: CheckboxThemePreference {
        unselected_fill: Preference::Reference("surface_inverse_tertiary"),
        selected_fill: Preference::Reference("primary"),
        selected_icon_fill: Preference::Reference("secondary"),
        border_fill: Preference::Reference("surface_primary"),
    },
    resizable_handle: ResizableHandleThemePreference {
        background: Preference::Reference("surface_secondary"),
        hover_background: Preference::Reference("surface_primary"),
    },
    floating_tab: FloatingTabThemePreference {
        background: Preference::Specific(Color::TRANSPARENT),
        hover_background: Preference::Reference("surface_secondary"),
        color: Preference::Reference("text_primary"),
        padding: Preference::Specific(Gaps::new(6., 12., 6., 12.)),
        width: Preference::Specific(Size::Inner),
        height: Preference::Specific(Size::Inner),
    },
    slider: SliderThemePreference {
        background: Preference::Reference("surface_primary"),
        thumb_background: Preference::Reference("secondary"),
        thumb_inner_background: Preference::Reference("primary"),
        border_fill: Preference::Reference("surface_primary"),
    },
    dropdown: DropdownThemePreference {
        width: Preference::Specific(Size::Inner),
        margin: Preference::Specific(Gaps::new_all(0.)),
        dropdown_background: Preference::Reference("background"),
        background_button: Preference::Reference("surface_tertiary"),
        hover_background: Preference::Reference("background"),
        color: Preference::Reference("text_primary"),
        border_fill: Preference::Reference("border"),
        focus_border_fill: Preference::Reference("border_focus"),
        arrow_fill: Preference::Reference("text_primary"),
    },
    dropdown_item: DropdownItemThemePreference {
        background: Preference::Reference("background"),
        select_background: Preference::Reference("surface_secondary"),
        hover_background: Preference::Reference("surface_secondary"),
        border_fill: Preference::Specific(Color::TRANSPARENT),
        select_border_fill: Preference::Reference("border_focus"),
        color: Preference::Reference("text_primary"),
    },
    popup: PopupThemePreference {
        background: Preference::Reference("background"),
        color: Preference::Reference("text_primary"),
    },
    table: TableThemePreference {
        background: Preference::Reference("background"),
        arrow_fill: Preference::Reference("text_primary"),
        row_background: Preference::Specific(Color::TRANSPARENT),
        hover_row_background: Preference::Reference("surface_secondary"),
        divider_fill: Preference::Reference("surface_primary"),
        corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
        color: Preference::Reference("text_primary"),
    },
    chip: ChipThemePreference {
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
};

pub const LIGHT_THEME: Theme = Theme {
    name: "light",
    colors: ColorsSheet {
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
    },
    ..BASE_THEME
};

pub const DARK_THEME: Theme = Theme {
    name: "dark",
    colors: ColorsSheet {
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
        text_inverse: Color::BLACK,
        text_highlight: Color::from_rgb(96, 145, 224),

        // States
        hover: Color::from_rgb(80, 80, 80),
        focus: Color::from_rgb(100, 100, 120),
        active: Color::from_rgb(70, 70, 70),
        disabled: Color::from_rgb(50, 50, 50),

        // Utility
        overlay: Color::from_af32rgb(0.2, 255, 255, 255),
        shadow: Color::from_af32rgb(0.6, 0, 0, 0),
    },
    ..BASE_THEME
};
