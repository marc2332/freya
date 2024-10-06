use super::base::BASE_THEME;
use crate::{
    cow_borrowed,
    theming::*,
};

pub const DARK_THEME: Theme = Theme {
    name: "dark",
    colors: ColorsSheet {
        primary: cow_borrowed!("rgb(103, 80, 164)"),
        secondary: cow_borrowed!("rgb(202, 193, 227)"),
        tertiary: cow_borrowed!("rgb(79, 61, 130)"),
        surface: cow_borrowed!("rgb(60, 60, 60)"),
        secondary_surface: cow_borrowed!("rgb(45, 45, 45)"),
        neutral_surface: cow_borrowed!("rgb(25, 25, 25)"),
        focused_surface: cow_borrowed!("rgb(15, 15, 15)"),
        opposite_surface: cow_borrowed!("rgb(125, 125, 125)"),
        secondary_opposite_surface: cow_borrowed!("rgb(150, 150, 150)"),
        tertiary_opposite_surface: cow_borrowed!("rgb(170, 170, 170)"),
        background: cow_borrowed!("rgb(20, 20, 20)"),
        focused_border: cow_borrowed!("rgb(110, 110, 110)"),
        solid: cow_borrowed!("rgb(240, 240, 240)"),
        color: cow_borrowed!("rgb(250, 250, 250)"),
        primary_color: cow_borrowed!("white"),
        placeholder_color: cow_borrowed!("rgb(210, 210, 210)"),
        highlight_color: cow_borrowed!("rgb(96, 145, 224)"),
    },
    ..BASE_THEME
};

pub const LIGHT_THEME: Theme = Theme {
    name: "light",
    colors: ColorsSheet {
        primary: cow_borrowed!("rgb(103, 80, 164)"),
        secondary: cow_borrowed!("rgb(202, 193, 227)"),
        tertiary: cow_borrowed!("rgb(79, 61, 130)"),
        surface: cow_borrowed!("rgb(210, 210, 210)"),
        secondary_surface: cow_borrowed!("rgb(225, 225, 225)"),
        neutral_surface: cow_borrowed!("rgb(245, 245, 245)"),
        focused_surface: cow_borrowed!("rgb(235, 235, 235)"),
        opposite_surface: cow_borrowed!("rgb(125, 125, 125)"),
        secondary_opposite_surface: cow_borrowed!("rgb(110, 110, 125)"),
        tertiary_opposite_surface: cow_borrowed!("rgb(90, 90, 90)"),
        background: cow_borrowed!("rgb(250, 250, 250)"),
        solid: cow_borrowed!("rgb(35, 35, 35)"),
        focused_border: cow_borrowed!("rgb(180, 180, 180)"),
        color: cow_borrowed!("rgb(10, 10, 10)"),
        primary_color: cow_borrowed!("white"),
        placeholder_color: cow_borrowed!("rgb(100, 100, 100)"),
        highlight_color: cow_borrowed!("rgb(38, 89, 170)"),
    },
    ..BASE_THEME
};

pub const BANANA_THEME: Theme = Theme {
    name: "banana",
    colors: ColorsSheet {
        primary: cow_borrowed!("rgb(240, 200, 50)"),
        secondary: cow_borrowed!("rgb(255, 250, 160)"),
        tertiary: cow_borrowed!("rgb(186, 153, 37)"),
        surface: cow_borrowed!("rgb(240, 229, 189)"),
        secondary_surface: cow_borrowed!("rgb(250, 240, 210)"),
        neutral_surface: cow_borrowed!("rgb(255, 245, 220)"),
        focused_surface: cow_borrowed!("rgb(255, 238, 170)"),
        opposite_surface: cow_borrowed!("rgb(139, 69, 19)"),
        secondary_opposite_surface: cow_borrowed!("rgb(120, 80, 20)"),
        tertiary_opposite_surface: cow_borrowed!("rgb(90, 60, 10)"),
        background: cow_borrowed!("rgb(255, 255, 224)"),
        solid: cow_borrowed!("rgb(110, 70, 10)"),
        focused_border: cow_borrowed!("rgb(255, 239, 151)"),
        color: cow_borrowed!("rgb(85, 60, 5)"),
        primary_color: cow_borrowed!("rgb(69, 49, 7)"),
        placeholder_color: cow_borrowed!("rgb(56, 44, 5)"),
        highlight_color: cow_borrowed!("rgb(143, 114, 6)"),
    },
    ..BASE_THEME
};
