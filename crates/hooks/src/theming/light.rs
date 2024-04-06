use crate::cow_borrowed;
use crate::theming::*;

pub const LIGHT_THEME: Theme = Theme {
    name: "light",
    body: BodyTheme {
        background: cow_borrowed!("white"),
        color: cow_borrowed!("black"),
        padding: cow_borrowed!("none"),
    },
    slider: SliderTheme {
        background: cow_borrowed!("rgb(210, 210, 210)"),
        thumb_background: cow_borrowed!("rgb(210, 210, 210)"),
        thumb_inner_background: cow_borrowed!("rgb(103, 80, 164)"),
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
    },
    button: ButtonTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        hover_background: cow_borrowed!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
        focus_border_fill: cow_borrowed!("rgb(180, 180, 180)"),
        shadow: cow_borrowed!("0 4 5 0 rgb(0, 0, 0, 0.1)"),
        padding: cow_borrowed!("8 16"),
        margin: cow_borrowed!("4"),
        corner_radius: cow_borrowed!("8"),
        width: cow_borrowed!("auto"),
        height: cow_borrowed!("auto"),
    },
    input: InputTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        hover_background: cow_borrowed!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
        width: cow_borrowed!("150"),
        margin: cow_borrowed!("4"),
        corner_radius: cow_borrowed!("10"),
    },
    switch: SwitchTheme {
        background: cow_borrowed!("rgb(121, 116, 126)"),
        thumb_background: cow_borrowed!("rgb(231, 224, 236)"),
        enabled_background: cow_borrowed!("rgb(103, 80, 164)"),
        enabled_thumb_background: cow_borrowed!("rgb(234, 221, 255)"),
        focus_border_fill: cow_borrowed!("rgb(180, 180, 180)"),
        enabled_focus_border_fill: cow_borrowed!("rgb(180, 180, 180)"),
    },
    scroll_bar: ScrollBarTheme {
        background: cow_borrowed!("rgb(225, 225, 225)"),
        thumb_background: cow_borrowed!("rgb(135, 135, 135)"),
        hover_thumb_background: cow_borrowed!("rgb(115, 115, 115)"),
        active_thumb_background: cow_borrowed!("rgb(95, 95, 95)"),
        size: cow_borrowed!("15"),
    },
    scroll_view: ScrollViewTheme {
        height: cow_borrowed!("fill"),
        width: cow_borrowed!("fill"),
        padding: cow_borrowed!("0"),
    },
    tooltip: TooltipTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        color: cow_borrowed!("rgb(25,25,25)"),
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
    },
    dropdown: DropdownTheme {
        dropdown_background: cow_borrowed!("white"),
        background_button: cow_borrowed!("rgb(245, 245, 245)"),
        hover_background: cow_borrowed!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
        arrow_fill: cow_borrowed!("rgb(40, 40, 40)"),
    },
    dropdown_item: DropdownItemTheme {
        background: cow_borrowed!("white"),
        select_background: cow_borrowed!("rgb(240, 240, 240)"),
        hover_background: cow_borrowed!("rgb(220, 220, 220)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
    },
    accordion: AccordionTheme {
        color: cow_borrowed!("black"),
        background: cow_borrowed!("rgb(245, 245, 245)"),
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
    },
    loader: LoaderTheme {
        primary_color: cow_borrowed!("rgb(50, 50, 50)"),
    },
    link: LinkTheme {
        highlight_color: cow_borrowed!("rgb(43,106,208)"),
    },
    progress_bar: ProgressBarTheme {
        color: cow_borrowed!("white"),
        background: cow_borrowed!("rgb(210, 210, 210)"),
        progress_background: cow_borrowed!("rgb(103, 80, 164)"),
        width: cow_borrowed!("fill"),
        height: cow_borrowed!("20"),
    },
    table: TableTheme {
        font_theme: FontTheme {
            color: cow_borrowed!("black"),
        },
        background: cow_borrowed!("white"),
        arrow_fill: cow_borrowed!("rgb(40, 40, 40)"),
        row_background: cow_borrowed!("transparent"),
        alternate_row_background: cow_borrowed!("rgb(240, 240, 240)"),
        divider_fill: cow_borrowed!("rgb(200, 200, 200)"),
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
        margin: cow_borrowed!("none"),
    },
    sidebar: SidebarTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
    },
    sidebar_item: SidebarItemTheme {
        background: cow_borrowed!("transparent"),
        hover_background: cow_borrowed!("rgb(230, 230, 230)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
    },
    tile: TileTheme {
        padding: cow_borrowed!("4 6"),
    },
    radio: RadioTheme {
        unselected_fill: cow_borrowed!("rgb(35, 35, 35)"),
        selected_fill: cow_borrowed!("rgb(103, 80, 164)"),
    },
    checkbox: CheckboxTheme {
        unselected_fill: cow_borrowed!("rgb(80, 80, 80)"),
        selected_fill: cow_borrowed!("rgb(103, 80, 164)"),
        selected_icon_fill: cow_borrowed!("white"),
    },
    menu_item: MenuItemTheme {
        hover_background: cow_borrowed!("rgb(235, 235, 235)"),
        corner_radius: cow_borrowed!("8"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
    },
    menu_container: MenuContainerTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        padding: cow_borrowed!("4"),
        shadow: cow_borrowed!("0 2 5 2 rgb(0, 0, 0, 0.1)"),
    },
    snackbar: SnackBarTheme {
        background: cow_borrowed!("rgb(235, 235, 235)"),
        color: cow_borrowed!("rgb(103, 80, 164)"),
    },
    popup: PopupTheme {
        background: cow_borrowed!("white"),
        color: cow_borrowed!("black"),
        cross_fill: cow_borrowed!("rgb(40, 40, 40)"),
        width: cow_borrowed!("350"),
        height: cow_borrowed!("200"),
    },
};
