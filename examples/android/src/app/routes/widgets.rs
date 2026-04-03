use std::{
    collections::HashSet,
    fmt,
};

use freya::{
    material_design::{
        ButtonRippleExt,
        MenuItemRippleExt,
        Ripple,
        TileRippleExt,
    },
    prelude::*,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Feature {
    WiFi,
    Bluetooth,
    Location,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Feature::WiFi => write!(f, "Wi-Fi"),
            Feature::Bluetooth => write!(f, "Bluetooth"),
            Feature::Location => write!(f, "Location"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum TextSize {
    Small,
    Medium,
    Large,
}

impl fmt::Display for TextSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextSize::Small => write!(f, "Small"),
            TextSize::Medium => write!(f, "Medium"),
            TextSize::Large => write!(f, "Large"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Tag {
    Design,
    Mobile,
    OpenSource,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tag::Design => write!(f, "Design"),
            Tag::Mobile => write!(f, "Mobile"),
            Tag::OpenSource => write!(f, "Open Source"),
        }
    }
}

#[derive(PartialEq)]
pub struct WidgetsDemo;

impl Component for WidgetsDemo {
    fn render(&self) -> impl IntoElement {
        let mut theme = use_theme();
        let is_dark = theme.read().name == "dark";

        let input_text = use_state(String::new);
        let mut slider_value = use_state(|| 50.0f64);

        let values = use_hook(|| {
            vec![
                "Rust".to_string(),
                "TypeScript".to_string(),
                "Python".to_string(),
            ]
        });
        let mut selected = use_state(|| 0);

        let mut enabled_features = use_state(|| HashSet::from([Feature::WiFi, Feature::Bluetooth]));
        let mut text_size = use_state(|| TextSize::Medium);

        let mut selected_tags = use_state(HashSet::<Tag>::new);
        let mut color = use_state(|| Color::from_hsv(0.0, 1.0, 1.0));

        ScrollView::new()
            .width(Size::fill())
            .height(Size::fill())
            .child(
                rect()
                    .width(Size::fill())
                    .padding(16.)
                    .spacing(20.)
                    .child(
                        rect().spacing(8.).child("Dark Theme").child(
                            Switch::new()
                                .expanded()
                                .toggled(is_dark)
                                .on_toggle(move |_| {
                                    if is_dark {
                                        theme.set(light_theme());
                                    } else {
                                        theme.set(dark_theme());
                                    }
                                }),
                        ),
                    )
                    .child(
                        rect()
                            .spacing(8.)
                            .child(format!("Slider: {}%", slider_value().floor()))
                            .child(
                                Slider::new(move |v| slider_value.set(v))
                                    .value(slider_value())
                                    .size(Size::fill()),
                            )
                            .child(ProgressBar::new(slider_value().floor() as f32)),
                    )
                    .child(
                        rect().spacing(8.).child("Language").child(
                            Select::new()
                                .selected_item(values[selected()].to_string())
                                .children(values.iter().enumerate().map(|(i, val)| {
                                    MenuItem::new()
                                        .selected(selected() == i)
                                        .on_press(move |_| selected.set(i))
                                        .ripple()
                                        .child(val.to_string())
                                        .into()
                                })),
                        ),
                    )
                    .child(
                        rect()
                            .width(Size::fill())
                            .spacing(8.)
                            .child("Text Input")
                            .child(
                                Input::new(input_text)
                                    .expanded()
                                    .width(Size::fill())
                                    .flat()
                                    .placeholder("Type something..."),
                            )
                            .child(format!("Value: {}", input_text.read())),
                    )
                    .child(
                        rect()
                            .width(Size::fill())
                            .spacing(4.)
                            .child("Features")
                            .children([Feature::WiFi, Feature::Bluetooth, Feature::Location].map(
                                |feature| {
                                    let is_checked = enabled_features.read().contains(&feature);
                                    Tile::new()
                                        .on_select(move |_| {
                                            if enabled_features.read().contains(&feature) {
                                                enabled_features.write().remove(&feature);
                                            } else {
                                                enabled_features.write().insert(feature);
                                            }
                                        })
                                        .ripple()
                                        .leading(Checkbox::new().selected(is_checked))
                                        .child(
                                            label().text(feature.to_string()).width(Size::fill()),
                                        )
                                        .into()
                                },
                            )),
                    )
                    .child(
                        rect()
                            .width(Size::fill())
                            .spacing(4.)
                            .child("Text Size")
                            .children([TextSize::Small, TextSize::Medium, TextSize::Large].map(
                                |size| {
                                    Tile::new()
                                        .on_select(move |_| text_size.set(size))
                                        .ripple()
                                        .leading(RadioItem::new().selected(text_size() == size))
                                        .child(label().text(size.to_string()).width(Size::fill()))
                                        .into()
                                },
                            )),
                    )
                    .child(Button::new().expanded().ripple().child("Ripple Button"))
                    .child(ripple_card(
                        (230, 230, 240),
                        (30, 30, 30),
                        None::<Color>,
                        "Tap for ripple",
                    ))
                    .child(ripple_card(
                        (255, 240, 240),
                        (30, 30, 30),
                        Some((255, 80, 80)),
                        "Red ripple",
                    ))
                    .child(
                        rect().spacing(8.).child("Card").child(
                            Card::new()
                                .width(Size::fill())
                                .child("This is a card surface with elevated styling."),
                        ),
                    )
                    .child(
                        rect().spacing(8.).child("Accordion").child(
                            Accordion::new().header("Click to expand").child(
                                "Accordion content goes here. You can put any elements inside.",
                            ),
                        ),
                    )
                    .child(
                        rect().spacing(8.).child("Tags").child(
                            rect()
                                .direction(Direction::Horizontal)
                                .spacing(8.)
                                .children([Tag::Design, Tag::Mobile, Tag::OpenSource].map(|tag| {
                                    let is_selected = selected_tags.read().contains(&tag);
                                    Chip::new()
                                        .selected(is_selected)
                                        .on_press(move |_| {
                                            if selected_tags.read().contains(&tag) {
                                                selected_tags.write().remove(&tag);
                                            } else {
                                                selected_tags.write().insert(tag);
                                            }
                                        })
                                        .child(tag.to_string())
                                        .into()
                                })),
                        ),
                    )
                    .child(
                        rect()
                            .spacing(8.)
                            .child("Loader")
                            .child(CircularLoader::new()),
                    )
                    .child(
                        rect()
                            .spacing(8.)
                            .child("Color Picker")
                            .child(ColorPicker::new(move |c| color.set(c)).value(color())),
                    )
                    .child(
                        rect().spacing(8.).child("Remote Image").child(
                            ImageViewer::new("https://picsum.photos/500/1000")
                                .width(Size::fill())
                                .aspect_ratio(AspectRatio::Max),
                        ),
                    ),
            )
    }
}

fn ripple_card(
    bg: impl Into<Color>,
    fg: impl Into<Color>,
    ripple_color: Option<impl Into<Color>>,
    text: &str,
) -> impl IntoElement {
    let mut ripple = Ripple::new();

    if let Some(color) = ripple_color {
        ripple = ripple.color(color);
    }

    ripple.child(
        rect()
            .width(Size::fill())
            .height(Size::px(80.))
            .center()
            .background(bg)
            .corner_radius(12.)
            .color(fg)
            .child(text),
    )
}
