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
    external_link: ExternalLinkTheme {
        highlight_color: cow_borrowed!("rgb(43,106,208)"),
    },
    dropdown: DropdownTheme {
        dropdown_background: cow_borrowed!("rgb(25, 25, 25)"),
        background_button: cow_borrowed!("rgb(35, 35, 35)"),
        hover_background: cow_borrowed!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
        border_fill: cow_borrowed!("rgb(80, 80, 80)"),
        arrow_fill: cow_borrowed!("rgb(40, 40, 40)"),
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
        secondary_color: cow_borrowed!("rgb(255, 255, 255)"),
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
    arrow_icon: ArrowIconTheme {
        width: LIGHT_THEME.arrow_icon.width,
        height: LIGHT_THEME.arrow_icon.height,
        margin: LIGHT_THEME.arrow_icon.margin,
    },
};
