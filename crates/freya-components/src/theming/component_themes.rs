use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    size::Size,
};

#[cfg(feature = "router")]
use crate::link::Link;
use crate::{
    accordion::Accordion,
    button::Button,
    checkbox::Checkbox,
    chip::Chip,
    define_theme,
    dropdown::Dropdown,
    floating_tab::FloatingTab,
    input::Input,
    loader::CircularLoader,
    menu::{
        MenuContainer,
        MenuItem,
    },
    popup::Popup,
    progressbar::ProgressBar,
    radio_item::RadioItem,
    resizable_container::ResizableHandle,
    scrollviews::ScrollBar,
    sidebar::{
        SideBar,
        SideBarItem,
    },
    slider::Slider,
    switch::Switch,
    table::Table,
    theming::themes::LIGHT_THEME,
    tooltip::Tooltip,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub name: &'static str,
    pub colors: ColorsSheet,
    pub button_layout: ButtonLayoutThemePreference,
    pub compact_button_layout: ButtonLayoutThemePreference,
    pub expanded_button_layout: ButtonLayoutThemePreference,
    pub button: ButtonColorsThemePreference,
    pub filled_button: ButtonColorsThemePreference,
    pub outline_button: ButtonColorsThemePreference,
    pub accordion: AccordionThemePreference,
    pub switch: SwitchThemePreference,
    pub scrollbar: ScrollBarThemePreference,
    pub progressbar: ProgressBarThemePreference,
    pub sidebar: SideBarThemePreference,
    pub sidebar_item: SideBarItemThemePreference,
    #[cfg(feature = "router")]
    pub link: LinkThemePreference,
    pub tooltip: TooltipThemePreference,
    pub circular_loader: CircularLoaderThemePreference,
    pub input: InputThemePreference,
    pub radio: RadioItemThemePreference,
    pub checkbox: CheckboxThemePreference,
    pub resizable_handle: ResizableHandleThemePreference,
    pub floating_tab: FloatingTabThemePreference,
    pub slider: SliderThemePreference,
    pub dropdown: DropdownThemePreference,
    pub popup: PopupThemePreference,
    pub table: TableThemePreference,
    pub chip: ChipThemePreference,
    pub menu_item: MenuItemThemePreference,
    pub menu_container: MenuContainerThemePreference,
}

impl Default for Theme {
    fn default() -> Self {
        LIGHT_THEME
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorsSheet {
    // Brand & Accent
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,

    // Status / Semantic colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // Surfaces / Backgrounds
    pub background: Color,
    pub surface_primary: Color,
    pub surface_secondary: Color,
    pub surface_tertiary: Color,
    pub surface_inverse: Color,
    pub surface_inverse_secondary: Color,
    pub surface_inverse_tertiary: Color,

    // Borders
    pub border: Color,
    pub border_focus: Color,
    pub border_disabled: Color,

    // Text / Content
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_placeholder: Color,
    pub text_inverse: Color,
    pub text_highlight: Color,

    // States / Interaction
    pub hover: Color,
    pub focus: Color,
    pub active: Color,
    pub disabled: Color,

    // Utility
    pub overlay: Color,
    pub shadow: Color,
}

define_theme! {
    for = Button;
    theme_field = theme_layout;

    %[component]
    pub ButtonLayout {
        %[fields]
        margin: Gaps,
        corner_radius: CornerRadius,
        width: Size,
        height: Size,
        padding: Gaps,
    }
}

define_theme! {
    for = Button;
    theme_field = theme_colors;

    %[component]
    pub ButtonColors {
        %[fields]
        background: Color,
        hover_background: Color,
        border_fill: Color,
        focus_border_fill: Color,
        color: Color,
    }
}

define_theme! {
    %[component]
    pub Accordion {
        %[fields]
        color: Color,
        background: Color,
        border_fill: Color,
    }
}

define_theme! {
    %[component]
    pub Switch {
        %[fields]
        margin: Gaps,
        background: Color,
        thumb_background: Color,
        toggled_background: Color,
        toggled_thumb_background: Color,
        focus_border_fill: Color,
    }
}

define_theme! {
    %[component]
    pub ScrollBar {
        %[fields]
        background: Color,
        thumb_background: Color,
        hover_thumb_background: Color,
        active_thumb_background: Color,
        size: f32,
    }
}

define_theme! {
    %[component]
    pub ProgressBar {
        %[fields]
        color: Color,
        background: Color,
        progress_background: Color,
        height: f32,
    }
}

define_theme! {
    %[component]
    pub SideBar {
       %[fields]
        color: Color,
        background: Color,
        padding: Gaps,
        spacing: f32,
    }
}

define_theme! {
    %[component]
    pub SideBarItem {
        %[fields]
        color: Color,
        background: Color,
        hover_background: Color,
        active_background: Color,
        corner_radius: CornerRadius,
        margin: Gaps,
        padding: Gaps,
    }
}

#[cfg(feature = "router")]
define_theme! {
    %[component]
    pub Link {
        %[fields]
        color: Color,
    }
}

define_theme! {
    %[component]
    pub Tooltip {
        %[fields]
        color: Color,
        background: Color,
        border_fill: Color,
    }
}

define_theme! {
    %[component]
    pub CircularLoader {
        %[fields]
        primary_color: Color,
        inversed_color: Color,
    }
}

define_theme! {
    %[component]
    pub Input {
        %[fields]
        background: Color,
        hover_background: Color,
        border_fill: Color,
        focus_border_fill: Color,
        corner_radius: CornerRadius,
        inner_margin: Gaps,
        color: Color,
        placeholder_color: Color,
    }
}

define_theme! {
    %[component]
    pub RadioItem {
        %[fields]
        unselected_fill: Color,
        selected_fill: Color,
        border_fill: Color,
    }
}

define_theme! {
    %[component]
    pub Checkbox {
        %[fields]
        unselected_fill: Color,
        selected_fill: Color,
        selected_icon_fill: Color,
        border_fill: Color,
    }
}

define_theme! {
    %[component]
    pub ResizableHandle {
        %[fields]
        background: Color,
        hover_background: Color,
        corner_radius: CornerRadius,
    }
}

define_theme! {
    %[component]
    pub FloatingTab {
        %[fields]
        background: Color,
        hover_background: Color,
        width: Size,
        height: Size,
        padding: Gaps,
        color: Color,
    }
}

define_theme! {
    %[component]
    pub Slider {
        %[fields]
        background: Color,
        thumb_background: Color,
        thumb_inner_background: Color,
        border_fill: Color,
    }
}

define_theme! {
    %[component]
    pub Dropdown {
        %[fields]
        width: Size,
        margin: Gaps,
        dropdown_background: Color,
        background_button: Color,
        hover_background: Color,
        border_fill: Color,
        focus_border_fill: Color,
        arrow_fill: Color,
        color: Color,
    }
}

define_theme! {
    %[component]
    pub Popup {
        %[fields]
        background: Color,
        color: Color,
    }
}

define_theme! {
    %[component]
    pub Table {
        %[fields]
        background: Color,
        arrow_fill: Color,
        hover_row_background: Color,
        row_background: Color,
        divider_fill: Color,
        corner_radius: CornerRadius,
        color: Color,
    }
}

define_theme! {
    %[component]
    pub Chip {
        %[fields]
        background: Color,
        hover_background: Color,
        selected_background: Color,
        border_fill: Color,
        selected_border_fill: Color,
        hover_border_fill: Color,
        focus_border_fill: Color,
        margin: f32,
        corner_radius: CornerRadius,
        width: Size,
        height: Size,
        padding: Gaps,
        color: Color,
        hover_color: Color,
        selected_color: Color,
        selected_icon_fill: Color,
        hover_icon_fill: Color,
    }
}

define_theme! {
    %[component]
    pub MenuContainer {
        %[fields]
        background: Color,
        padding: Gaps,
        shadow: Color,
        border_fill: Color,
        corner_radius: CornerRadius,
    }
}

define_theme! {
    %[component]
    pub MenuItem {
       %[fields]
        background: Color,
        hover_background: Color,
        select_background: Color,
        border_fill: Color,
        select_border_fill: Color,
        corner_radius: CornerRadius,
        color: Color,
    }
}
