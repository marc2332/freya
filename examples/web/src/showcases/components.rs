use freya::{
    icons::lucide,
    prelude::*,
};

use crate::showcases::heading;

const RUST_LOGO: &[u8] = include_bytes!("../rust_logo.png");

#[derive(Clone, Copy, PartialEq)]
enum Density {
    Compact,
    Cozy,
    Comfortable,
}

impl Density {
    fn label(&self) -> &'static str {
        match self {
            Self::Compact => "Compact",
            Self::Cozy => "Cozy",
            Self::Comfortable => "Comfortable",
        }
    }
}

#[derive(PartialEq)]
pub struct ComponentsShowcase;

impl Component for ComponentsShowcase {
    fn render(&self) -> impl IntoElement {
        let mut slider_value = use_state(|| 45.0f64);
        let mut is_on = use_state(|| true);
        let mut clicks = use_state(|| 0);
        let mut selected = use_state(|| 0usize);
        let mut density = use_state(|| Density::Cozy);
        let mut features = use_state(|| vec!["Cheese", "Basil"]);
        let input_text = use_state(String::new);
        let mut show_popup = use_state(|| false);
        let mut theme = use_theme();

        let coffees = ["Espresso", "Flat white", "Cortado"];
        let icon_color = theme.read().colors.text_primary;
        let is_dark = theme.read().name == "dark";

        ScrollView::new().child(
            rect()
                .spacing(20.)
                .child(heading("Components", "Components gallery"))
                .child(
                    rect()
                        .horizontal()
                        .spacing(8.)
                        .cross_align(Alignment::center())
                        .child(
                            Button::new()
                                .on_press(move |_| *clicks.write() += 1)
                                .child("Press me"),
                        )
                        .child(Button::new().filled().child("Filled"))
                        .child(Button::new().outline().child("Outline"))
                        .child(format!("{} clicks", clicks())),
                )
                .child(
                    rect()
                        .horizontal()
                        .spacing(14.)
                        .cross_align(Alignment::center())
                        .children(
                            [
                                lucide::house as fn() -> Bytes,
                                lucide::heart,
                                lucide::star,
                                lucide::settings,
                                lucide::search,
                            ]
                            .map(|icon| {
                                SvgViewer::new(icon())
                                    .stroke(icon_color)
                                    .width(Size::px(22.))
                                    .height(Size::px(22.))
                            }),
                        ),
                )
                .child(
                    rect()
                        .spacing(8.)
                        .child(format!("Slider: {}%", slider_value().floor()))
                        .child(
                            Slider::new(move |value| slider_value.set(value))
                                .value(slider_value())
                                .size(Size::fill()),
                        )
                        .child(ProgressBar::new(slider_value().floor() as f32)),
                )
                .child(
                    rect()
                        .horizontal()
                        .spacing(12.)
                        .cross_align(Alignment::center())
                        .child(
                            Switch::new()
                                .toggled(is_on())
                                .on_toggle(move |_| is_on.toggle()),
                        )
                        .child(if is_on() { "Switch on" } else { "Switch off" })
                        .child(Switch::new().toggled(is_dark).on_toggle(move |_| {
                            theme.set(if is_dark { light_theme() } else { dark_theme() })
                        }))
                        .child("Dark mode"),
                )
                .child(rect().spacing(4.).child("Toppings").children(
                    ["Cheese", "Olives", "Basil"].map(|feature| {
                        let is_checked = features.read().contains(&feature);
                        Tile::new()
                            .on_select(move |_| {
                                if is_checked {
                                    features.write().retain(|item| *item != feature);
                                } else {
                                    features.write().push(feature);
                                }
                            })
                            .leading(Checkbox::new().selected(is_checked))
                            .child(label().text(feature).width(Size::fill()))
                    }),
                ))
                .child(rect().spacing(4.).child("Spacing").children(
                    [Density::Compact, Density::Cozy, Density::Comfortable].map(|value| {
                        Tile::new()
                            .on_select(move |_| density.set(value))
                            .leading(RadioItem::new().selected(density() == value))
                            .child(label().text(value.label()).width(Size::fill()))
                    }),
                ))
                .child(rect().spacing(8.).child("Coffee").child(
                    Select::new().selected_item(coffees[selected()]).children(
                        coffees.iter().enumerate().map(|(index, name)| {
                            MenuItem::new()
                                .selected(selected() == index)
                                .on_press(move |_| selected.set(index))
                                .child(*name)
                        }),
                    ),
                ))
                .child(
                    rect()
                        .spacing(8.)
                        .width(Size::fill())
                        .child("Your name")
                        .child(
                            Input::new(input_text)
                                .width(Size::fill())
                                .placeholder("Type it here"),
                        )
                        .child(if input_text.read().is_empty() {
                            "We have not met yet".to_string()
                        } else {
                            format!("Nice to meet you, {}", input_text.read())
                        }),
                )
                .child(
                    rect()
                        .spacing(8.)
                        .child("A small window")
                        .child(
                            Button::new()
                                .on_press(move |_| show_popup.toggle())
                                .child("Open it"),
                        )
                        .child(
                            Popup::new()
                                .on_close_request(move |_| show_popup.set(false))
                                .maybe(show_popup(), |popup| {
                                    popup
                                        .child(PopupTitle::new("Hello there".to_string()))
                                        .child(
                                            PopupContent::new()
                                                .child("Nothing important here, just saying hi."),
                                        )
                                        .child(
                                            PopupButtons::new().child(
                                                Button::new()
                                                    .on_press(move |_| show_popup.set(false))
                                                    .expanded()
                                                    .filled()
                                                    .child("Close"),
                                            ),
                                        )
                                }),
                        ),
                )
                .child(
                    rect().spacing(8.).child("Image").child(
                        ImageViewer::new(("rust-logo", RUST_LOGO))
                            .width(Size::px(120.))
                            .height(Size::px(120.)),
                    ),
                ),
        )
    }
}
