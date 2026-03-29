use freya::{
    material_design::{
        ButtonRippleExt,
        MenuItemRippleExt,
        Ripple,
    },
    prelude::*,
};

#[derive(PartialEq)]
pub struct WidgetsDemo;

impl Component for WidgetsDemo {
    fn render(&self) -> impl IntoElement {
        let mut theme = use_theme();
        let is_dark = *theme.read() == DARK_THEME;

        let mut slider_value = use_state(|| 50.0f64);

        let values = use_hook(|| {
            vec![
                "Rust".to_string(),
                "TypeScript".to_string(),
                "Python".to_string(),
            ]
        });
        let mut selected = use_state(|| 0usize);

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
                                        theme.set(LIGHT_THEME);
                                    } else {
                                        theme.set(DARK_THEME);
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
                    )),
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
