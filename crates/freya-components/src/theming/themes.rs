use std::time::Duration;

use freya_core::prelude::*;
#[cfg(feature = "titlebar")]
use torin::prelude::Length;
use torin::{
    gaps::Gaps,
    size::Size,
};

#[cfg(feature = "router")]
use crate::link::LinkThemePreference;
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
    combobox::ComboBoxThemePreference,
    corner_radius::{
        CornerRadiusRole,
        CornerRadiusScale,
    },
    density::{
        Density,
        PaddingRole,
    },
    floating_tab::FloatingTabThemePreference,
    icons::{
        IconSize,
        IconTheme,
        IconThemePreference,
    },
    input::{
        InputColorsThemePreference,
        InputLayoutThemePreference,
    },
    loader::CircularLoaderThemePreference,
    menu::{
        MenuContainerThemePreference,
        MenuItemThemePreference,
    },
    popup::{
        PopupButtonsThemePreference,
        PopupContentThemePreference,
        PopupThemePreference,
        PopupTitleThemePreference,
    },
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
    skeleton::{
        SkeletonAnimation,
        SkeletonThemePreference,
    },
    slider::SliderThemePreference,
    switch::{
        SwitchColorsThemePreference,
        SwitchLayoutThemePreference,
    },
    table::TableThemePreference,
    theming::{
        component_themes::{
            AnyMap,
            Theme,
        },
        macros::Preference,
    },
    tile::TileThemePreference,
    tooltip::TooltipThemePreference,
    typography::{
        TextRole,
        TypographyScale,
    },
};

/// Map of color names used by freya's built-in themes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorRole {
    Primary,
    Secondary,
    Tertiary,
    Success,
    Warning,
    Error,
    Info,
    Background,
    SurfacePrimary,
    SurfaceSecondary,
    SurfaceTertiary,
    SurfaceInverse,
    SurfaceInverseSecondary,
    SurfaceInverseTertiary,
    Border,
    BorderFocus,
    BorderDisabled,
    TextPrimary,
    TextSecondary,
    TextPlaceholder,
    TextInverse,
    TextHighlight,
    Focus,
    Active,
    Disabled,
    Overlay,
    Shadow,
}

impl ColorRole {
    pub const fn key(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Tertiary => "tertiary",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Info => "info",
            Self::Background => "background",
            Self::SurfacePrimary => "surface_primary",
            Self::SurfaceSecondary => "surface_secondary",
            Self::SurfaceTertiary => "surface_tertiary",
            Self::SurfaceInverse => "surface_inverse",
            Self::SurfaceInverseSecondary => "surface_inverse_secondary",
            Self::SurfaceInverseTertiary => "surface_inverse_tertiary",
            Self::Border => "border",
            Self::BorderFocus => "border_focus",
            Self::BorderDisabled => "border_disabled",
            Self::TextPrimary => "text_primary",
            Self::TextSecondary => "text_secondary",
            Self::TextPlaceholder => "text_placeholder",
            Self::TextInverse => "text_inverse",
            Self::TextHighlight => "text_highlight",
            Self::Focus => "focus",
            Self::Active => "active",
            Self::Disabled => "disabled",
            Self::Overlay => "overlay",
            Self::Shadow => "shadow",
        }
    }
}

pub const LIGHT_COLORS: &[(ColorRole, Color)] = &[
    // Brand & Accent
    (ColorRole::Primary, Color::from_rgb(103, 80, 164)),
    (ColorRole::Secondary, Color::from_rgb(202, 193, 227)),
    (ColorRole::Tertiary, Color::from_rgb(79, 61, 130)),
    // Status
    (ColorRole::Success, Color::from_rgb(76, 175, 80)),
    (ColorRole::Warning, Color::from_rgb(255, 193, 7)),
    (ColorRole::Error, Color::from_rgb(244, 67, 54)),
    (ColorRole::Info, Color::from_rgb(33, 150, 243)),
    // Surfaces
    (ColorRole::Background, Color::from_rgb(250, 250, 250)),
    (ColorRole::SurfacePrimary, Color::from_rgb(210, 210, 210)),
    (ColorRole::SurfaceSecondary, Color::from_rgb(225, 225, 225)),
    (ColorRole::SurfaceTertiary, Color::from_rgb(245, 245, 245)),
    (ColorRole::SurfaceInverse, Color::from_rgb(125, 125, 125)),
    (
        ColorRole::SurfaceInverseSecondary,
        Color::from_rgb(110, 110, 110),
    ),
    (
        ColorRole::SurfaceInverseTertiary,
        Color::from_rgb(90, 90, 90),
    ),
    // Borders
    (ColorRole::Border, Color::from_rgb(210, 210, 210)),
    (ColorRole::BorderFocus, Color::from_rgb(180, 180, 180)),
    (ColorRole::BorderDisabled, Color::from_rgb(210, 210, 210)),
    // Text
    (ColorRole::TextPrimary, Color::from_rgb(10, 10, 10)),
    (ColorRole::TextSecondary, Color::from_rgb(100, 100, 100)),
    (ColorRole::TextPlaceholder, Color::from_rgb(150, 150, 150)),
    (ColorRole::TextInverse, Color::WHITE),
    (ColorRole::TextHighlight, Color::from_rgb(38, 89, 170)),
    // States
    (ColorRole::Focus, Color::from_rgb(225, 225, 255)),
    (ColorRole::Active, Color::from_rgb(200, 200, 200)),
    (ColorRole::Disabled, Color::from_rgb(210, 210, 210)),
    // Utility
    (ColorRole::Overlay, Color::from_af32rgb(0.5, 0, 0, 0)),
    (ColorRole::Shadow, Color::from_af32rgb(0.2, 0, 0, 0)),
];

pub const DARK_COLORS: &[(ColorRole, Color)] = &[
    // Brand & Accent
    (ColorRole::Primary, Color::from_rgb(103, 80, 164)),
    (ColorRole::Secondary, Color::from_rgb(202, 193, 227)),
    (ColorRole::Tertiary, Color::from_rgb(79, 61, 130)),
    // Status
    (ColorRole::Success, Color::from_rgb(129, 199, 132)),
    (ColorRole::Warning, Color::from_rgb(255, 213, 79)),
    (ColorRole::Error, Color::from_rgb(229, 115, 115)),
    (ColorRole::Info, Color::from_rgb(100, 181, 246)),
    // Surfaces
    (ColorRole::Background, Color::from_rgb(20, 20, 20)),
    (ColorRole::SurfacePrimary, Color::from_rgb(60, 60, 60)),
    (ColorRole::SurfaceSecondary, Color::from_rgb(45, 45, 45)),
    (ColorRole::SurfaceTertiary, Color::from_rgb(25, 25, 25)),
    (ColorRole::SurfaceInverse, Color::from_rgb(135, 135, 135)),
    (
        ColorRole::SurfaceInverseSecondary,
        Color::from_rgb(150, 150, 150),
    ),
    (
        ColorRole::SurfaceInverseTertiary,
        Color::from_rgb(170, 170, 170),
    ),
    // Borders
    (ColorRole::Border, Color::from_rgb(60, 60, 60)),
    (ColorRole::BorderFocus, Color::from_rgb(110, 110, 110)),
    (ColorRole::BorderDisabled, Color::from_rgb(80, 80, 80)),
    // Text
    (ColorRole::TextPrimary, Color::from_rgb(250, 250, 250)),
    (ColorRole::TextSecondary, Color::from_rgb(210, 210, 210)),
    (ColorRole::TextPlaceholder, Color::from_rgb(150, 150, 150)),
    (ColorRole::TextInverse, Color::WHITE),
    (ColorRole::TextHighlight, Color::from_rgb(96, 145, 224)),
    // States
    (ColorRole::Focus, Color::from_rgb(100, 100, 120)),
    (ColorRole::Active, Color::from_rgb(70, 70, 70)),
    (ColorRole::Disabled, Color::from_rgb(50, 50, 50)),
    // Utility
    (ColorRole::Overlay, Color::from_af32rgb(0.2, 255, 255, 255)),
    (ColorRole::Shadow, Color::from_af32rgb(0.6, 0, 0, 0)),
];

fn register_base_component_themes(theme: &mut Theme) {
    theme.set_theme(
        "button_layout",
        ButtonLayoutThemePreference {
            font_size: Preference::Reference(TextRole::Control.key()),
            padding: PaddingRole::COMPACT_WIDE.into(),
            margin: Preference::Specific(Gaps::new_all(0.)),
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
        },
    );
    theme.set_theme(
        "compact_button_layout",
        ButtonLayoutThemePreference {
            font_size: Preference::Reference(TextRole::Control.key()),
            padding: PaddingRole::TIGHT_WIDE.into(),
            margin: Preference::Specific(Gaps::new_all(0.)),
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
        },
    );
    theme.set_theme(
        "expanded_button_layout",
        ButtonLayoutThemePreference {
            font_size: Preference::Reference(TextRole::Control.key()),
            padding: PaddingRole::RELAXED_WIDE.into(),
            margin: Preference::Specific(Gaps::new_all(0.)),
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
        },
    );
    theme.set_theme(
        "button",
        ButtonColorsThemePreference {
            background: Preference::Reference(ColorRole::SurfaceTertiary.key()),
            hover_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            border_fill: Preference::Reference(ColorRole::Border.key()),
            focus_border_fill: Preference::Reference(ColorRole::BorderFocus.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
        },
    );
    theme.set_theme(
        "filled_button",
        ButtonColorsThemePreference {
            background: Preference::Reference(ColorRole::Primary.key()),
            hover_background: Preference::Reference(ColorRole::Tertiary.key()),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            focus_border_fill: Preference::Reference(ColorRole::Secondary.key()),
            color: Preference::Reference(ColorRole::TextInverse.key()),
        },
    );
    theme.set_theme(
        "outline_button",
        ButtonColorsThemePreference {
            background: Preference::Reference(ColorRole::SurfaceTertiary.key()),
            hover_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            border_fill: Preference::Reference(ColorRole::Border.key()),
            focus_border_fill: Preference::Reference(ColorRole::Secondary.key()),
            color: Preference::Reference(ColorRole::Primary.key()),
        },
    );
    theme.set_theme(
        "flat_button",
        ButtonColorsThemePreference {
            background: Preference::Specific(Color::TRANSPARENT),
            hover_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            focus_border_fill: Preference::Reference(ColorRole::Border.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
        },
    );
    theme.set_theme(
        "card_layout",
        CardLayoutThemePreference {
            padding: PaddingRole::SPACIOUS.into(),
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
        },
    );
    theme.set_theme(
        "compact_card_layout",
        CardLayoutThemePreference {
            padding: PaddingRole::STANDARD_WIDE.into(),
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
        },
    );
    theme.set_theme(
        "filled_card",
        CardColorsThemePreference {
            background: Preference::Reference(ColorRole::Primary.key()),
            hover_background: Preference::Reference(ColorRole::Tertiary.key()),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            color: Preference::Reference(ColorRole::TextInverse.key()),
            shadow: Preference::Reference(ColorRole::Shadow.key()),
        },
    );
    theme.set_theme(
        "outline_card",
        CardColorsThemePreference {
            background: Preference::Reference(ColorRole::SurfaceTertiary.key()),
            hover_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            border_fill: Preference::Reference(ColorRole::Border.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            shadow: Preference::Reference(ColorRole::Shadow.key()),
        },
    );
    theme.set_theme(
        "accordion",
        AccordionThemePreference {
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            background: Preference::Reference(ColorRole::SurfaceTertiary.key()),
            border_fill: Preference::Reference(ColorRole::Border.key()),
            padding: PaddingRole::STANDARD.into(),
        },
    );
    theme.set_theme(
        "switch",
        SwitchColorsThemePreference {
            background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            thumb_background: Preference::Reference(ColorRole::SurfaceInverse.key()),
            toggled_background: Preference::Reference(ColorRole::Secondary.key()),
            toggled_thumb_background: Preference::Reference(ColorRole::Primary.key()),
            focus_border_fill: Preference::Reference(ColorRole::BorderFocus.key()),
        },
    );
    theme.set_theme(
        "switch_layout",
        SwitchLayoutThemePreference {
            margin: Preference::Specific(Gaps::new_all(0.)),
            width: Preference::Specific(48.),
            height: Preference::Specific(28.),
            padding: Preference::Specific(4.),
            density: Preference::Reference(Density::key()),
            thumb_size: Preference::Specific(16.),
            toggled_thumb_size: Preference::Specific(20.),
            pressed_thumb_size_offset: Preference::Specific(4.),
            thumb_offset: Preference::Specific(2.),
            toggled_thumb_offset: Preference::Specific(20.),
        },
    );
    theme.set_theme(
        "expanded_switch_layout",
        SwitchLayoutThemePreference {
            margin: Preference::Specific(Gaps::new_all(0.)),
            width: Preference::Specific(56.),
            height: Preference::Specific(32.),
            padding: Preference::Specific(4.),
            density: Preference::Reference(Density::key()),
            thumb_size: Preference::Specific(18.),
            toggled_thumb_size: Preference::Specific(22.),
            pressed_thumb_size_offset: Preference::Specific(4.),
            thumb_offset: Preference::Specific(2.),
            toggled_thumb_offset: Preference::Specific(26.),
        },
    );
    theme.set_theme(
        "scrollbar",
        ScrollBarThemePreference {
            background: Preference::Reference(ColorRole::Border.key()),
            thumb_background: Preference::Reference(ColorRole::SurfaceInverse.key()),
            hover_thumb_background: Preference::Reference(ColorRole::SurfaceInverseSecondary.key()),
            active_thumb_background: Preference::Reference(ColorRole::SurfaceInverseTertiary.key()),
            thumb_cross_size: Preference::Specific(5.),
            expanded_thumb_cross_size: Preference::Specific(8.),
            opacity: Preference::Specific(0.),
            expanded_opacity: Preference::Specific(220.),
            cross_gap: Preference::Specific(0.),
            expanded_cross_gap: Preference::Specific(3.),
        },
    );
    theme.set_theme(
        "progressbar",
        ProgressBarThemePreference {
            color: Preference::Reference(ColorRole::TextInverse.key()),
            background: Preference::Reference(ColorRole::SurfacePrimary.key()),
            progress_background: Preference::Reference(ColorRole::Primary.key()),
            height: Preference::Specific(20.),
            font_size: Preference::Reference(TextRole::Caption.key()),
        },
    );
    theme.set_theme(
        "sidebar_item",
        SideBarItemThemePreference {
            font_size: Preference::Reference(TextRole::Control.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            background: Preference::Reference(ColorRole::SurfaceTertiary.key()),
            active_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            hover_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            focus_border_fill: Preference::Reference(ColorRole::BorderFocus.key()),
            corner_radius: Preference::Reference(CornerRadiusRole::Xl.key()),
            margin: Preference::Specific(Gaps::new_all(0.)),
            padding: PaddingRole::STANDARD_WIDE.into(),
        },
    );
    #[cfg(feature = "router")]
    theme.set_theme(
        "link",
        LinkThemePreference {
            color: Preference::Reference(ColorRole::TextHighlight.key()),
        },
    );
    theme.set_theme(
        "tooltip",
        TooltipThemePreference {
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
            background: Preference::Reference(ColorRole::SurfaceTertiary.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            border_fill: Preference::Reference(ColorRole::SurfacePrimary.key()),
            font_size: Preference::Reference(TextRole::Control.key()),
            padding: PaddingRole::TIGHT_WIDE.into(),
        },
    );
    theme.set_theme(
        "tile",
        TileThemePreference {
            padding: PaddingRole::STANDARD.into(),
            leading_padding: PaddingRole::TIGHT.into(),
        },
    );
    theme.set_theme(
        "circular_loader",
        CircularLoaderThemePreference {
            primary_color: Preference::Reference(ColorRole::SurfacePrimary.key()),
        },
    );
    theme.set_theme(
        "input_layout",
        InputLayoutThemePreference {
            font_size: Preference::Reference(TextRole::Control.key()),
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
            padding: PaddingRole::STANDARD.into(),
        },
    );
    theme.set_theme(
        "compact_input_layout",
        InputLayoutThemePreference {
            font_size: Preference::Reference(TextRole::Control.key()),
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
            padding: PaddingRole::TIGHT_WIDE.into(),
        },
    );
    theme.set_theme(
        "expanded_input_layout",
        InputLayoutThemePreference {
            font_size: Preference::Reference(TextRole::Control.key()),
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
            padding: PaddingRole::RELAXED.into(),
        },
    );
    theme.set_theme(
        "input",
        InputColorsThemePreference {
            background: Preference::Reference(ColorRole::SurfaceTertiary.key()),
            focus_background: Preference::Reference(ColorRole::Background.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            placeholder_color: Preference::Reference(ColorRole::TextSecondary.key()),
            border_fill: Preference::Reference(ColorRole::Border.key()),
            focus_border_fill: Preference::Reference(ColorRole::BorderFocus.key()),
        },
    );
    theme.set_theme(
        "filled_input",
        InputColorsThemePreference {
            background: Preference::Reference(ColorRole::Primary.key()),
            focus_background: Preference::Reference(ColorRole::Tertiary.key()),
            color: Preference::Reference(ColorRole::TextInverse.key()),
            placeholder_color: Preference::Reference(ColorRole::TextInverse.key()),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            focus_border_fill: Preference::Reference(ColorRole::Secondary.key()),
        },
    );
    theme.set_theme(
        "flat_input",
        InputColorsThemePreference {
            background: Preference::Specific(Color::TRANSPARENT),
            focus_background: Preference::Reference(ColorRole::SurfaceTertiary.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            placeholder_color: Preference::Reference(ColorRole::TextSecondary.key()),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            focus_border_fill: Preference::Reference(ColorRole::Border.key()),
        },
    );
    theme.set_theme(
        "radio",
        RadioItemThemePreference {
            padding: PaddingRole::TIGHT.into(),
            unselected_fill: Preference::Reference(ColorRole::SurfaceInverseTertiary.key()),
            selected_fill: Preference::Reference(ColorRole::Primary.key()),
            border_fill: Preference::Reference(ColorRole::SurfacePrimary.key()),
        },
    );
    theme.set_theme(
        "checkbox",
        CheckboxThemePreference {
            padding: PaddingRole::TIGHT.into(),
            unselected_fill: Preference::Reference(ColorRole::SurfaceInverseTertiary.key()),
            selected_fill: Preference::Reference(ColorRole::Primary.key()),
            selected_icon_fill: Preference::Reference(ColorRole::Secondary.key()),
            border_fill: Preference::Reference(ColorRole::SurfacePrimary.key()),
        },
    );
    theme.set_theme(
        "resizable_handle",
        ResizableHandleThemePreference {
            background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            hover_background: Preference::Reference(ColorRole::SurfacePrimary.key()),
            corner_radius: Preference::Reference(CornerRadiusRole::Sm.key()),
        },
    );
    theme.set_theme(
        "floating_tab",
        FloatingTabThemePreference {
            font_size: Preference::Reference(TextRole::Control.key()),
            background: Preference::Specific(Color::TRANSPARENT),
            hover_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            padding: PaddingRole::COMPACT_WIDE.into(),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
            corner_radius: CornerRadius::new_all(99.).into(),
        },
    );
    theme.set_theme(
        "slider",
        SliderThemePreference {
            background: Preference::Reference(ColorRole::SurfacePrimary.key()),
            thumb_background: Preference::Reference(ColorRole::Secondary.key()),
            thumb_inner_background: Preference::Reference(ColorRole::Primary.key()),
            border_fill: Preference::Reference(ColorRole::SurfacePrimary.key()),
        },
    );
    theme.set_theme(
        "color_picker",
        ColorPickerThemePreference {
            padding: PaddingRole::STANDARD.into(),
            background: Preference::Reference(ColorRole::SurfaceTertiary.key()),
            border_fill: Preference::Reference(ColorRole::Border.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
        },
    );
    theme.set_theme(
        "select",
        SelectThemePreference {
            corner_radius: Preference::Reference(CornerRadiusRole::Lg.key()),
            list_corner_radius: Preference::Reference(CornerRadiusRole::Xl.key()),
            font_size: Preference::Reference(TextRole::Control.key()),
            padding: PaddingRole::COMPACT_WIDE.into(),
            options_padding: PaddingRole::COMPACT.into(),
            width: Preference::Specific(Size::Inner),
            margin: Preference::Specific(Gaps::new_all(0.)),
            select_background: Preference::Reference(ColorRole::Background.key()),
            background_button: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            hover_background: Preference::Reference(ColorRole::SurfacePrimary.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            border_fill: Preference::Reference(ColorRole::SurfacePrimary.key()),
            focus_border_fill: Preference::Reference(ColorRole::BorderFocus.key()),
            arrow_fill: Preference::Reference(ColorRole::TextPrimary.key()),
        },
    );
    theme.set_theme(
        "combobox",
        ComboBoxThemePreference {
            corner_radius: Preference::Reference(CornerRadiusRole::Lg.key()),
            list_corner_radius: Preference::Reference(CornerRadiusRole::Xl.key()),
            font_size: Preference::Reference(TextRole::Control.key()),
            width: Preference::Specific(Size::px(260.)),
            margin: Preference::Specific(Gaps::new_all(0.)),
            padding: Preference::Specific(Gaps::new_all(8.)),
            list_background: Preference::Reference(ColorRole::Background.key()),
            background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            focus_background: Preference::Reference(ColorRole::Background.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            placeholder_color: Preference::Reference(ColorRole::TextSecondary.key()),
            border_fill: Preference::Reference(ColorRole::SurfacePrimary.key()),
            focus_border_fill: Preference::Reference(ColorRole::BorderFocus.key()),
            arrow_fill: Preference::Reference(ColorRole::TextPrimary.key()),
        },
    );
    theme.set_theme(
        "popup_title",
        PopupTitleThemePreference {
            padding: PaddingRole::STANDARD.into(),
            font_size: Preference::Reference(TextRole::Subtitle.key()),
        },
    );
    theme.set_theme(
        "popup_content",
        PopupContentThemePreference {
            padding: PaddingRole::STANDARD.into(),
            font_size: Preference::Reference(TextRole::Content.key()),
        },
    );
    theme.set_theme(
        "popup_buttons",
        PopupButtonsThemePreference {
            padding: PaddingRole::STANDARD.into(),
        },
    );
    theme.set_theme(
        "popup",
        PopupThemePreference {
            corner_radius: Preference::Reference(CornerRadiusRole::Xl.key()),
            background: Preference::Reference(ColorRole::Background.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            width: Preference::Specific(Size::px(500.)),
            height: Preference::Specific(Size::auto()),
            min_width: Preference::Specific(Size::px(100.)),
            min_height: Preference::Specific(Size::auto()),
            max_width: Preference::Specific(Size::window_percent(90.)),
            max_height: Preference::Specific(Size::window_percent(90.)),
            padding: PaddingRole::STANDARD.into(),
            spacing: Preference::Specific(4.),
        },
    );
    theme.set_theme(
        "icon",
        IconThemePreference {
            fill: Preference::Reference(ColorRole::TextPrimary.key()),
            size: Preference::Reference(IconTheme::size_key()),
        },
    );
    theme.set_theme(
        "table",
        TableThemePreference {
            background: Preference::Reference(ColorRole::Background.key()),
            row_background: Preference::Specific(Color::TRANSPARENT),
            hover_row_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            divider_fill: Preference::Reference(ColorRole::SurfacePrimary.key()),
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
            cell_padding: PaddingRole::STANDARD_WIDE.into(),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
        },
    );
    theme.set_theme(
        "chip",
        ChipThemePreference {
            font_size: Preference::Reference(TextRole::Control.key()),
            background: Preference::Reference(ColorRole::Background.key()),
            hover_background: Preference::Reference(ColorRole::Tertiary.key()),
            selected_background: Preference::Reference(ColorRole::Primary.key()),
            border_fill: Preference::Reference(ColorRole::Border.key()),
            hover_border_fill: Preference::Reference(ColorRole::Tertiary.key()),
            selected_border_fill: Preference::Reference(ColorRole::Primary.key()),
            focus_border_fill: Preference::Reference(ColorRole::Secondary.key()),
            padding: PaddingRole::STANDARD_WIDE.into(),
            margin: Preference::Specific(0.),
            corner_radius: CornerRadius::new_all(99.).into(),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::Inner),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            hover_color: Preference::Reference(ColorRole::TextInverse.key()),
            selected_color: Preference::Reference(ColorRole::TextInverse.key()),
            selected_icon_fill: Preference::Reference(ColorRole::Secondary.key()),
            hover_icon_fill: Preference::Reference(ColorRole::Secondary.key()),
        },
    );
    theme.set_theme(
        "menu_item",
        MenuItemThemePreference {
            font_size: Preference::Reference(TextRole::Control.key()),
            padding: PaddingRole::COMPACT_WIDE.into(),
            background: Preference::Specific(Color::TRANSPARENT),
            hover_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            select_background: Preference::Reference(ColorRole::SurfacePrimary.key()),
            border_fill: Preference::Specific(Color::TRANSPARENT),
            focus_border_fill: Preference::Reference(ColorRole::BorderFocus.key()),
            corner_radius: Preference::Reference(CornerRadiusRole::Lg.key()),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
            select_color: Preference::Reference(ColorRole::TextPrimary.key()),
        },
    );
    theme.set_theme(
        "menu_container",
        MenuContainerThemePreference {
            background: Preference::Reference(ColorRole::Background.key()),
            padding: PaddingRole::COMPACT.into(),
            shadow: Preference::Specific(Color::from((0, 0, 0, 0.25))),
            border_fill: Preference::Reference(ColorRole::SurfacePrimary.key()),
            corner_radius: Preference::Reference(CornerRadiusRole::Xl.key()),
        },
    );
    theme.set_theme(
        "button_segment",
        ButtonSegmentThemePreference {
            corner_radius: Preference::Reference(CornerRadiusRole::Md.key()),
            font_size: Preference::Reference(TextRole::Control.key()),
            background: Preference::Specific(Color::TRANSPARENT),
            hover_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            disabled_background: Preference::Reference(ColorRole::Disabled.key()),
            selected_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            focus_background: Preference::Reference(ColorRole::SurfacePrimary.key()),
            padding: PaddingRole::COMPACT_WIDE.into(),
            selected_padding: PaddingRole::COMPACT_WIDE.into(),
            width: Preference::Specific(Size::Inner),
            height: Preference::Specific(Size::FillMinimum),
            color: Preference::Reference(ColorRole::TextPrimary.key()),
        },
    );
    theme.set_theme(
        "segmented_button",
        SegmentedButtonThemePreference {
            padding: PaddingRole::TIGHT.into(),
            background: Preference::Reference(ColorRole::Background.key()),
            border_fill: Preference::Reference(ColorRole::Border.key()),
            corner_radius: Preference::Reference(CornerRadiusRole::Xl.key()),
        },
    );
    #[cfg(feature = "titlebar")]
    theme.set_theme(
        "titlebar_button",
        TitlebarButtonThemePreference {
            background: Preference::Specific(Color::TRANSPARENT),
            hover_background: Preference::Reference(ColorRole::SurfaceSecondary.key()),
            corner_radius: CornerRadius::new_all(0.).into(),
            width: Preference::Specific(Size::Pixels(Length::new(46.0))),
            height: Preference::Specific(Size::Fill),
        },
    );
    theme.set_theme(
        "skeleton",
        SkeletonThemePreference {
            background: Preference::Reference(ColorRole::SurfacePrimary.key()),
            shimmer_color: Preference::Specific(Color::WHITE.with_a(160)),
            duration: Preference::Specific(Duration::from_millis(1000)),
            animation: Preference::Specific(SkeletonAnimation::Pulse),
            corner_radius: Preference::Reference(CornerRadiusRole::Xs.key()),
            shimmer_from: Preference::Specific(-320.),
            shimmer_to: Preference::Specific(960.),
            shimmer_width: Preference::Specific(300.),
        },
    );
}

/// Light theme with all built-in component themes registered.
///
/// The primary color tracks the OS-level [`AccentColor`] when available. The
/// secondary and tertiary colors are derived from it (lightened / darkened).
pub fn light_theme() -> Theme {
    build_theme("light", LIGHT_COLORS)
}

/// Dark theme with all built-in component themes registered.
///
/// The primary color tracks the OS-level [`AccentColor`] when available. The
/// secondary and tertiary colors are derived from it (lightened / darkened).
pub fn dark_theme() -> Theme {
    build_theme("dark", DARK_COLORS)
}

fn build_theme(name: &'static str, colors: &[(ColorRole, Color)]) -> Theme {
    let mut variables = AnyMap::default();
    for &(role, color) in colors {
        variables.set(role.key(), color);
    }
    TypographyScale::default().apply_to(&mut variables);
    CornerRadiusScale::default().apply_to(&mut variables);
    Density::default().apply_to(&mut variables);
    IconSize::default().apply_to(&mut variables);
    if let Some(primary) = current_accent_color() {
        variables.set(ColorRole::Primary.key(), primary);
        variables.set(
            ColorRole::Secondary.key(),
            Color::lerp(primary, Color::WHITE, 0.65),
        );
        variables.set(
            ColorRole::Tertiary.key(),
            Color::lerp(primary, Color::BLACK, 0.23),
        );
    }
    let mut theme = Theme::new(name, variables);
    register_base_component_themes(&mut theme);
    theme
}

fn current_accent_color() -> Option<Color> {
    let window_id: CurrentWindowId = try_consume_root_context()?;
    let platform = Platform::get().try_window(window_id.0)?;
    let accent = platform.accent_color.read().0?;
    let [r, g, b, _] = accent.to_u8_array();
    Some(Color::from_rgb(r, g, b))
}
