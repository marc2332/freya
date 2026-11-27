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
    let mut density_slider = use_state(|| 50.);
    let mut radius_slider = use_state(|| 40.);
    let mut icon_slider = use_state(|| (10. - 6.) * 5.);

    let is_dark = theme.read().name == "dark";
    let font_size = 10. + font_slider() / 5.;
    let density_percent = 50. + density_slider();
    let base_radius = radius_slider() / 5.;
    let icon_size = (6. + icon_slider() / 5.) as f32;

    rect()
        .theme_background()
        .theme_color()
        .expanded()
        .content(Content::flex())
        .padding(20.)
        .spacing(16.)
        .child(
            rect()
                .width(Size::fill())
                .horizontal()
                .main_align(Alignment::space_between())
                .cross_align(Alignment::center())
                .child(
                    rect()
                        .spacing(4.)
                        .child(label().title().text("Theme scales"))
                        .child(label().body().text("Change the values")),
                )
                .child(
                    rect()
                        .horizontal()
                        .cross_align(Alignment::center())
                        .spacing(12.)
                        .child("Dark theme")
                        .child(Switch::new(WritableSetter::new(is_dark, move |is_dark| {
                            let mut next_theme = if is_dark { dark_theme() } else { light_theme() };
                            TypographyScale {
                                base_size: font_size as f32,
                                ..TypographyScale::default()
                            }
                            .apply_to(&mut next_theme.variables);
                            Density {
                                factor: (density_percent / 100.) as f32,
                            }
                            .apply_to(&mut next_theme.variables);
                            CornerRadiusScale {
                                base_radius: base_radius as f32,
                                ..CornerRadiusScale::default()
                            }
                            .apply_to(&mut next_theme.variables);
                            IconSize(icon_size).apply_to(&mut next_theme.variables);
                            theme.set(next_theme);
                        })))
                        .child(
                            Button::new()
                                .flat()
                                .on_press(move |_| {
                                    font_slider.set(30.);
                                    density_slider.set(50.);
                                    radius_slider.set(40.);
                                    icon_slider.set((10. - 6.) * 5.);
                                    theme.set(if is_dark { dark_theme() } else { light_theme() });
                                })
                                .child("Reset values"),
                        ),
                ),
        )
        .child(
            rect()
                .horizontal()
                .content(Content::flex())
                .width(Size::fill())
                .spacing(16.)
                .child(
                    rect()
                        .width(Size::flex(1.))
                        .child(format!("Font size: {font_size:.0} px"))
                        .child(
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
                        ),
                )
                .child(
                    rect()
                        .width(Size::flex(1.))
                        .child(format!("Density: {density_percent:.0}%"))
                        .child(
                            Slider::new(move |value| {
                                density_slider.set_if_modified_and_then(value, || {
                                    Density {
                                        factor: (0.5 + value / 100.) as f32,
                                    }
                                    .apply_to(&mut theme.write().variables);
                                });
                            })
                            .value(density_slider())
                            .step(1.),
                        ),
                )
                .child(
                    rect()
                        .width(Size::flex(1.))
                        .child(format!("Corner radius: {base_radius:.0} px"))
                        .child(
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
                        ),
                )
                .child(
                    rect()
                        .width(Size::flex(1.))
                        .child(format!("Icon size: {icon_size:.0} px"))
                        .child(
                            Slider::new(move |value| {
                                icon_slider.set_if_modified_and_then(value, || {
                                    IconSize((6. + value / 5.) as f32)
                                        .apply_to(&mut theme.write().variables);
                                });
                            })
                            .value(icon_slider())
                            .step(5.),
                        ),
                ),
        )
        .child(
            ScrollView::new()
                .width(Size::fill())
                .height(Size::flex(1.))
                .child(Preview::default()),
        )
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
                        .spacing(16.)
                        .child(
                            rect()
                                .spacing(4.)
                                .child(label().title().text("Some title"))
                                .child(label().body().text("Lorem ipsum dolor sit amet."))
                                .child(label().caption().text("Consectetur adipiscing elit.")),
                        )
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
