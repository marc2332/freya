#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("Theme scales")
                .with_size(880., 680.)
                .with_min_size(760., 580.),
        ),
    )
}

fn app() -> impl IntoElement {
    let mut theme = use_init_theme(light_theme);
    let mut font_slider = use_state(|| 30.);
    let mut padding_slider = use_state(|| 50.);
    let mut radius_slider = use_state(|| 40.);
    let (is_dark, icon_size) = {
        let theme = theme.read();
        (theme.name == "dark", *theme.get_var::<f32>(ICON_SIZE_VAR))
    };
    let font_size = 10. + font_slider() / 5.;
    let padding_percent = 50. + padding_slider();
    let base_radius = radius_slider() / 5.;

    rect()
        .theme_background()
        .theme_color()
        .expanded()
        .content(Content::flex())
        .padding(20.)
        .spacing(16.)
        .child(label().title().text("Theme scales"))
        .child(label().body().text("Change the values"))
        .child(
            rect()
                .horizontal()
                .content(Content::flex())
                .width(Size::fill())
                .height(Size::flex(1.))
                .spacing(16.)
                .child(
                    ScrollView::new()
                        .width(Size::flex(1.))
                        .height(Size::fill())
                        .child(Preview::default()),
                )
                .child(
                    ScrollView::new()
                        .width(Size::px(288.))
                        .height(Size::fill())
                        .child(
                            Card::new().outline().width(Size::fill()).child(
                                rect()
                                    .width(Size::fill())
                                    .spacing(12.)
                                    .child(label().subtitle().text("Theme controls"))
                                    .child(
                                        rect()
                                            .horizontal()
                                            .cross_align(Alignment::center())
                                            .spacing(8.)
                                            .child("Dark theme")
                                            .child(Switch::new(WritableSetter::new(
                                                is_dark,
                                                move |is_dark| {
                                                    let mut next_theme = if is_dark {
                                                        dark_theme()
                                                    } else {
                                                        light_theme()
                                                    };
                                                    TypographyScale {
                                                        base_size: font_size as f32,
                                                        ..TypographyScale::default()
                                                    }
                                                    .apply_to(&mut next_theme.variables);
                                                    PaddingScale {
                                                        factor: (padding_percent / 100.) as f32,
                                                    }
                                                    .apply_to(&mut next_theme.variables);
                                                    CornerRadiusScale {
                                                        base_radius: base_radius as f32,
                                                        ..CornerRadiusScale::default()
                                                    }
                                                    .apply_to(&mut next_theme.variables);
                                                    next_theme.set_var(ICON_SIZE_VAR, icon_size);
                                                    theme.set(next_theme);
                                                },
                                            ))),
                                    )
                                    .child(control(
                                        format!("Base font size: {font_size:.0} px"),
                                        Slider::new(move |value| {
                                            font_slider.set_if_modified_and_then(value, || {
                                                TypographyScale {
                                                    base_size: (10. + value / 5.) as f32,
                                                    ..TypographyScale::default()
                                                }
                                                .apply_to(&mut theme.write().variables);
                                            });
                                        })
                                        .value(font_slider())
                                        .step(5.),
                                    ))
                                    .child(control(
                                        format!("Component padding: {padding_percent:.0}%"),
                                        Slider::new(move |value| {
                                            padding_slider.set_if_modified_and_then(value, || {
                                                PaddingScale {
                                                    factor: (0.5 + value / 100.) as f32,
                                                }
                                                .apply_to(&mut theme.write().variables);
                                            });
                                        })
                                        .value(padding_slider())
                                        .step(1.),
                                    ))
                                    .child(control(
                                        format!("Base corner radius: {base_radius:.0} px"),
                                        Slider::new(move |value| {
                                            radius_slider.set_if_modified_and_then(value, || {
                                                CornerRadiusScale {
                                                    base_radius: (value / 5.) as f32,
                                                    ..CornerRadiusScale::default()
                                                }
                                                .apply_to(&mut theme.write().variables);
                                            });
                                        })
                                        .value(radius_slider())
                                        .step(5.),
                                    ))
                                    .child(control(
                                        format!("Icon size: {icon_size:.0} px"),
                                        Slider::new(move |value| {
                                            theme
                                                .write()
                                                .set_var(ICON_SIZE_VAR, 6. + value as f32 / 5.);
                                        })
                                        .value(f64::from(icon_size - 6.) * 5.)
                                        .step(5.),
                                    ))
                                    .child(
                                        Button::new()
                                            .flat()
                                            .on_press(move |_| {
                                                font_slider.set(30.);
                                                padding_slider.set(50.);
                                                radius_slider.set(40.);
                                                theme.set(if is_dark {
                                                    dark_theme()
                                                } else {
                                                    light_theme()
                                                });
                                            })
                                            .child("Reset values"),
                                    ),
                            ),
                        ),
                ),
        )
}

fn control(text: String, slider: Slider) -> impl IntoElement {
    rect().width(Size::fill()).child(text).child(slider)
}

#[derive(Default, PartialEq)]
struct Preview {
    key: DiffKey,
}

impl KeyExt for Preview {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for Preview {
    fn render(&self) -> impl IntoElement {
        let name = use_state(String::new);
        let mut selected = use_state(|| "Rust");
        let enabled = use_state(|| true);

        rect()
            .width(Size::fill())
            .spacing(12.)
            .child(label().subtitle().text("Live preview"))
            .child(
                Card::new().outline().width(Size::fill()).child(
                    rect()
                        .width(Size::fill())
                        .spacing(8.)
                        .child(label().title().text("Some title"))
                        .child(label().body().text("Lorem ipsum dolor sit amet."))
                        .child(label().caption().text("Captions use a smaller type role.")),
                ),
            )
            .child(
                Card::new().outline().width(Size::fill()).child(
                    rect()
                        .width(Size::fill())
                        .spacing(12.)
                        .child(label().overline().text("COMPONENTS"))
                        .child(
                            Input::new(name)
                                .width(Size::fill())
                                .placeholder("Project name"),
                        )
                        .child(
                            rect()
                                .horizontal()
                                .cross_align(Alignment::center())
                                .spacing(8.)
                                .child(Button::new().filled().child("Primary"))
                                .child(Button::new().outline().child("Outline"))
                                .child(Select::new().selected_item(selected()).children(
                                    ["Rust", "Turbofish", "Crabs"].into_iter().map(|option| {
                                        MenuItem::new()
                                            .selected(selected() == option)
                                            .on_press(move |_| selected.set(option))
                                            .child(option)
                                    }),
                                )),
                        )
                        .child(
                            rect()
                                .horizontal()
                                .cross_align(Alignment::center())
                                .spacing(12.)
                                .child(Switch::new(enabled))
                                .child("Enabled")
                                .child(ArrowIcon::new())
                                .child(TickIcon::new()),
                        ),
                ),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone()
    }
}
