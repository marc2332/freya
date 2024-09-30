use crate::{
    cow_borrowed,
    theming::*,
};

pub(crate) const BASE_THEME: Theme = Theme {
    name: "base",
    colors: ColorsSheet {
        primary: cow_borrowed!(""),
        secondary: cow_borrowed!(""),
        tertiary: cow_borrowed!(""),
        surface: cow_borrowed!(""),
        secondary_surface: cow_borrowed!(""),
        neutral_surface: cow_borrowed!(""),
        focused_surface: cow_borrowed!(""),
        opposite_surface: cow_borrowed!(""),
        secondary_opposite_surface: cow_borrowed!(""),
        tertiary_opposite_surface: cow_borrowed!(""),
        background: cow_borrowed!(""),
        focused_border: cow_borrowed!(""),
        solid: cow_borrowed!(""),
        color: cow_borrowed!(""),
        placeholder_color: cow_borrowed!(""),
        highlight_color: cow_borrowed!(""),
    },
    body: BodyTheme {
        background: cow_borrowed!("key(background)"),
        color: cow_borrowed!("key(color)"),
        padding: cow_borrowed!("none"),
    },
    slider: SliderTheme {
        background: cow_borrowed!("key(surface)"),
        thumb_background: cow_borrowed!("key(secondary)"),
        thumb_inner_background: cow_borrowed!("key(primary)"),
        border_fill: cow_borrowed!("key(surface)"),
    },
    button: ButtonTheme {
        background: cow_borrowed!("key(neutral_surface)"),
        hover_background: cow_borrowed!("key(focused_surface)"),
        font_theme: FontTheme {
            color: cow_borrowed!("key(color)"),
        },
        border_fill: cow_borrowed!("key(surface)"),
        focus_border_fill: cow_borrowed!("key(focused_border)"),
        shadow: cow_borrowed!("0 4 5 0 rgb(0, 0, 0, 0.1)"),
        padding: cow_borrowed!("8 12"),
        margin: cow_borrowed!("0"),
        corner_radius: cow_borrowed!("8"),
        width: cow_borrowed!("auto"),
        height: cow_borrowed!("auto"),
    },
    input: InputTheme {
        background: cow_borrowed!("key(neutral_surface)"),
        hover_background: cow_borrowed!("key(focused_surface)"),
        font_theme: FontTheme {
            color: cow_borrowed!("key(color)"),
        },
        placeholder_font_theme: FontTheme {
            color: cow_borrowed!("key(placeholder_color)"),
        },
        border_fill: cow_borrowed!("key(surface)"),
        width: cow_borrowed!("150"),
        margin: cow_borrowed!("0"),
        corner_radius: cow_borrowed!("10"),
        shadow: cow_borrowed!("0 4 5 0 rgb(0, 0, 0, 0.1)"),
    },
    switch: SwitchTheme {
        margin: cow_borrowed!("0"),
        background: cow_borrowed!("key(secondary_surface)"),
        thumb_background: cow_borrowed!("key(opposite_surface)"),
        enabled_background: cow_borrowed!("key(secondary)"),
        enabled_thumb_background: cow_borrowed!("key(primary)"),
        focus_border_fill: cow_borrowed!("key(focused_border)"),
        enabled_focus_border_fill: cow_borrowed!("key(focused_border)"),
    },
    scroll_bar: ScrollBarTheme {
        background: cow_borrowed!("key(secondary_surface)"),
        thumb_background: cow_borrowed!("key(opposite_surface)"),
        hover_thumb_background: cow_borrowed!("key(secondary_opposite_surface)"),
        active_thumb_background: cow_borrowed!("key(tertiary_opposite_surface)"),
        size: cow_borrowed!("15"),
    },
    tooltip: TooltipTheme {
        background: cow_borrowed!("key(neutral_surface)"),
        color: cow_borrowed!("key(color)"),
        border_fill: cow_borrowed!("key(surface)"),
    },
    dropdown: DropdownTheme {
        width: cow_borrowed!("auto"),
        margin: cow_borrowed!("0"),
        dropdown_background: cow_borrowed!("key(background)"),
        background_button: cow_borrowed!("key(neutral_surface)"),
        hover_background: cow_borrowed!("key(focused_surface)"),
        font_theme: FontTheme {
            color: cow_borrowed!("key(color)"),
        },
        border_fill: cow_borrowed!("key(surface)"),
        arrow_fill: cow_borrowed!("key(solid)"),
    },
    dropdown_item: DropdownItemTheme {
        background: cow_borrowed!("key(background)"),
        select_background: cow_borrowed!("key(neutral_surface)"),
        hover_background: cow_borrowed!("key(focused_surface)"),
        font_theme: FontTheme {
            color: cow_borrowed!("key(color)"),
        },
    },
    accordion: AccordionTheme {
        color: cow_borrowed!("key(color)"),
        background: cow_borrowed!("key(neutral_surface)"),
        border_fill: cow_borrowed!("key(surface)"),
    },
    loader: LoaderTheme {
        primary_color: cow_borrowed!("key(tertiary_opposite_surface)"),
    },
    link: LinkTheme {
        highlight_color: cow_borrowed!("key(highlight_color)"),
    },
    progress_bar: ProgressBarTheme {
        color: cow_borrowed!("white"),
        background: cow_borrowed!("key(surface)"),
        progress_background: cow_borrowed!("key(primary)"),
        width: cow_borrowed!("fill"),
        height: cow_borrowed!("20"),
    },
    table: TableTheme {
        font_theme: FontTheme {
            color: cow_borrowed!("key(color)"),
        },
        background: cow_borrowed!("key(background)"),
        arrow_fill: cow_borrowed!("key(solid)"),
        row_background: cow_borrowed!("transparent"),
        alternate_row_background: cow_borrowed!("key(neutral_surface)"),
        divider_fill: cow_borrowed!("key(secondary_surface)"),
        height: cow_borrowed!("auto"),
        corner_radius: cow_borrowed!("6"),
        shadow: cow_borrowed!("0 2 15 5 rgb(35, 35, 35, 70)"),
    },
    canvas: CanvasTheme {
        width: cow_borrowed!("300"),
        height: cow_borrowed!("150"),
        background: cow_borrowed!("white"),
    },
    graph: GraphTheme {
        width: cow_borrowed!("100%"),
        height: cow_borrowed!("100%"),
    },
    network_image: NetworkImageTheme {
        width: cow_borrowed!("100%"),
        height: cow_borrowed!("100%"),
    },
    icon: IconTheme {
        width: cow_borrowed!("10"),
        height: cow_borrowed!("10"),
        margin: cow_borrowed!("0"),
    },
    sidebar: SidebarTheme {
        spacing: cow_borrowed!("4"),
        background: cow_borrowed!("key(neutral_surface)"),
        font_theme: FontTheme {
            color: cow_borrowed!("key(color)"),
        },
    },
    sidebar_item: SidebarItemTheme {
        margin: cow_borrowed!("0"),
        background: cow_borrowed!("transparent"),
        hover_background: cow_borrowed!("key(focused_surface)"),
        font_theme: FontTheme {
            color: cow_borrowed!("key(color)"),
        },
    },
    tile: TileTheme {
        padding: cow_borrowed!("4 6"),
    },
    radio: RadioTheme {
        unselected_fill: cow_borrowed!("key(solid)"),
        selected_fill: cow_borrowed!("key(primary)"),
        border_fill: cow_borrowed!("key(surface)"),
    },
    checkbox: CheckboxTheme {
        unselected_fill: cow_borrowed!("key(solid)"),
        selected_fill: cow_borrowed!("key(primary)"),
        selected_icon_fill: cow_borrowed!("key(secondary)"),
        border_fill: cow_borrowed!("key(surface)"),
    },
    menu_item: MenuItemTheme {
        hover_background: cow_borrowed!("key(focused_surface)"),
        corner_radius: cow_borrowed!("8"),
        font_theme: FontTheme {
            color: cow_borrowed!("key(color)"),
        },
    },
    menu_container: MenuContainerTheme {
        background: cow_borrowed!("key(neutral_surface)"),
        padding: cow_borrowed!("4"),
        shadow: cow_borrowed!("0 2 5 2 rgb(0, 0, 0, 0.1)"),
    },
    snackbar: SnackBarTheme {
        background: cow_borrowed!("key(focused_surface)"),
        color: cow_borrowed!("key(color)"),
    },
    popup: PopupTheme {
        background: cow_borrowed!("key(background)"),
        color: cow_borrowed!("key(color)"),
        cross_fill: cow_borrowed!("key(solid)"),
        width: cow_borrowed!("350"),
        height: cow_borrowed!("200"),
    },
    tab: TabTheme {
        background: cow_borrowed!("key(neutral_surface)"),
        hover_background: cow_borrowed!("key(focused_surface)"),
        font_theme: FontTheme {
            color: cow_borrowed!("key(color)"),
        },
        border_fill: cow_borrowed!("none"),
        focus_border_fill: cow_borrowed!("key(focused_border)"),
        padding: cow_borrowed!("8 16"),
        width: cow_borrowed!("auto"),
        height: cow_borrowed!("auto"),
    },
    bottom_tab: BottomTabTheme {
        background: cow_borrowed!("transparent"),
        hover_background: cow_borrowed!("key(secondary_surface)"),
        font_theme: FontTheme {
            color: cow_borrowed!("key(color)"),
        },
        padding: cow_borrowed!("8 10"),
        width: cow_borrowed!("auto"),
        height: cow_borrowed!("auto"),
    },
};
