use crate::cow_borrowed;
use crate::theming::*;

pub const DARK_THEME: Theme = Theme {
    name: "dark",
    body: BodyTheme {
        background: cow_borrowed!("rgb(25, 25, 25)"),
        color: cow_borrowed!("white"),
        padding: LIGHT_THEME.body.padding,
    },
    slider: SliderTheme {
        background: cow_borrowed!("rgb(60, 60, 60)"),
        thumb_background: cow_borrowed!("rgb(60, 60, 60)"),
        thumb_inner_background: cow_borrowed!("rgb(255, 95, 0)"),
        border_fill: cow_borrowed!("rgb(110, 110, 110)"),
    },
    button: ButtonTheme {
        background: cow_borrowed!("rgb(35, 35, 35)"),
        hover_background: cow_borrowed!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
        border_fill: cow_borrowed!("rgb(80, 80, 80)"),
        focus_border_fill: cow_borrowed!("rgb(110, 110, 110)"),
        shadow: cow_borrowed!("0 4 5 0 rgb(0, 0, 0, 0.1)"),
        padding: LIGHT_THEME.button.padding,
        margin: LIGHT_THEME.button.margin,
        corner_radius: LIGHT_THEME.button.corner_radius,
        width: LIGHT_THEME.button.width,
        height: LIGHT_THEME.button.height,
    },
    input: InputTheme {
        background: cow_borrowed!("rgb(35, 35, 35)"),
        hover_background: cow_borrowed!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
        border_fill: cow_borrowed!("rgb(80, 80, 80)"),
        width: LIGHT_THEME.input.width,
        margin: LIGHT_THEME.input.margin,
        corner_radius: LIGHT_THEME.input.corner_radius,
    },
    switch: SwitchTheme {
        background: cow_borrowed!("rgb(60, 60, 60)"),
        thumb_background: cow_borrowed!("rgb(200, 200, 200)"),
        enabled_background: cow_borrowed!("rgb(255, 95, 0)"),
        enabled_thumb_background: cow_borrowed!("rgb(234, 221, 255)"),
        focus_border_fill: cow_borrowed!("rgb(110, 110, 110)"),
        enabled_focus_border_fill: cow_borrowed!("rgb(170, 170, 170)"),
    },
    scroll_bar: ScrollBarTheme {
        background: cow_borrowed!("rgb(35, 35, 35)"),
        thumb_background: cow_borrowed!("rgb(100, 100, 100)"),
        hover_thumb_background: cow_borrowed!("rgb(120, 120, 120)"),
        active_thumb_background: cow_borrowed!("rgb(140, 140, 140)"),
        size: LIGHT_THEME.scroll_bar.size,
    },
    scroll_view: ScrollViewTheme {
        height: LIGHT_THEME.scroll_view.height,
        width: LIGHT_THEME.scroll_view.width,
        padding: LIGHT_THEME.scroll_view.padding,
    },
    tooltip: TooltipTheme {
        background: cow_borrowed!("rgb(35,35,35)"),
        color: cow_borrowed!("rgb(240,240,240)"),
        border_fill: cow_borrowed!("rgb(80, 80, 80)"),
    },
    dropdown: DropdownTheme {
        dropdown_background: cow_borrowed!("rgb(25, 25, 25)"),
        background_button: cow_borrowed!("rgb(35, 35, 35)"),
        hover_background: cow_borrowed!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
        border_fill: cow_borrowed!("rgb(80, 80, 80)"),
        arrow_fill: cow_borrowed!("rgb(150, 150, 150)"),
    },
    dropdown_item: DropdownItemTheme {
        background: cow_borrowed!("rgb(35, 35, 35)"),
        select_background: cow_borrowed!("rgb(80, 80, 80)"),
        hover_background: cow_borrowed!("rgb(55, 55, 55)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
    },
    accordion: AccordionTheme {
        color: cow_borrowed!("white"),
        background: cow_borrowed!("rgb(60, 60, 60)"),
        border_fill: cow_borrowed!("rgb(80, 80, 80)"),
    },
    loader: LoaderTheme {
        primary_color: cow_borrowed!("rgb(150, 150, 150)"),
    },
    link: LinkTheme {
        highlight_color: cow_borrowed!("rgb(43,106,208)"),
    },
    progress_bar: ProgressBarTheme {
        color: cow_borrowed!("white"),
        background: cow_borrowed!("rgb(60, 60, 60)"),
        progress_background: cow_borrowed!("rgb(255, 95, 0)"),
        width: LIGHT_THEME.progress_bar.width,
        height: LIGHT_THEME.progress_bar.height,
    },
    table: TableTheme {
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
        background: cow_borrowed!("rgb(25, 25, 25)"),
        arrow_fill: cow_borrowed!("rgb(150, 150, 150)"),
        row_background: cow_borrowed!("transparent"),
        alternate_row_background: cow_borrowed!("rgb(50, 50, 50)"),
        divider_fill: cow_borrowed!("rgb(100, 100, 100)"),
        height: LIGHT_THEME.table.height,
        corner_radius: LIGHT_THEME.table.corner_radius,
        shadow: LIGHT_THEME.table.shadow,
    },
    canvas: CanvasTheme {
        width: LIGHT_THEME.canvas.width,
        height: LIGHT_THEME.canvas.height,
        background: cow_borrowed!("white"),
    },
    graph: GraphTheme {
        width: LIGHT_THEME.graph.width,
        height: LIGHT_THEME.graph.height,
    },
    network_image: NetworkImageTheme {
        width: LIGHT_THEME.network_image.width,
        height: LIGHT_THEME.network_image.height,
    },
    icon: IconTheme {
        width: LIGHT_THEME.icon.width,
        height: LIGHT_THEME.icon.height,
        margin: LIGHT_THEME.icon.margin,
    },
    sidebar: SidebarTheme {
        background: cow_borrowed!("rgb(20, 20, 20)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
    },
    sidebar_item: SidebarItemTheme {
        background: cow_borrowed!("transparent"),
        hover_background: cow_borrowed!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
    },
    tile: TileTheme {
        padding: LIGHT_THEME.tile.padding,
    },
    radio: RadioTheme {
        unselected_fill: cow_borrowed!("rgb(245, 245, 245)"),
        selected_fill: cow_borrowed!("rgbrgb(103, 80, 164)"),
    },
    checkbox: CheckboxTheme {
        unselected_fill: cow_borrowed!("rgb(245, 245, 245)"),
        selected_fill: cow_borrowed!("rgb(103, 80, 164)"),
        selected_icon_fill: cow_borrowed!("white"),
    },
    menu_item: MenuItemTheme {
        hover_background: cow_borrowed!("rgb(45, 45, 45)"),
        corner_radius: LIGHT_THEME.menu_item.corner_radius,
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
    },
    menu_container: MenuContainerTheme {
        background: cow_borrowed!("rgb(35, 35, 35)"),
        padding: LIGHT_THEME.menu_container.padding,
        shadow: LIGHT_THEME.menu_container.shadow,
    },
    snackbar: SnackBarTheme {
        background: cow_borrowed!("rgb(35, 35, 35)"),
        color: cow_borrowed!("white"),
    },
    popup: PopupTheme {
        background: cow_borrowed!("rgb(25, 25, 25)"),
        color: cow_borrowed!("white"),
        cross_fill: cow_borrowed!("rgb(150, 150, 150)"),
        width: LIGHT_THEME.popup.width,
        height: LIGHT_THEME.popup.height,
    },
};
