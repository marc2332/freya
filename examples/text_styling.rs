#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(720., 900.)
                .with_title("Text styling"),
        ),
    )
}

fn app() -> impl IntoElement {
    ScrollView::new().expanded().child(
        rect()
            .padding(24.)
            .spacing(28.)
            .child(section(
                "Font size",
                rect()
                    .horizontal()
                    .spacing(16.)
                    .child(label().text("12").font_size(12.))
                    .child(label().text("16").font_size(16.))
                    .child(label().text("24").font_size(24.))
                    .child(label().text("32").font_size(32.)),
            ))
            .child(section(
                "Font weight",
                rect()
                    .horizontal()
                    .spacing(16.)
                    .child(
                        label()
                            .text("Thin")
                            .font_size(18.)
                            .font_weight(FontWeight::THIN),
                    )
                    .child(
                        label()
                            .text("Normal")
                            .font_size(18.)
                            .font_weight(FontWeight::NORMAL),
                    )
                    .child(
                        label()
                            .text("Medium")
                            .font_size(18.)
                            .font_weight(FontWeight::MEDIUM),
                    )
                    .child(
                        label()
                            .text("Bold")
                            .font_size(18.)
                            .font_weight(FontWeight::BOLD),
                    )
                    .child(
                        label()
                            .text("Black")
                            .font_size(18.)
                            .font_weight(FontWeight::BLACK),
                    ),
            ))
            .child(section(
                "Font slant",
                rect()
                    .horizontal()
                    .spacing(16.)
                    .child(
                        label()
                            .text("Upright")
                            .font_size(18.)
                            .font_slant(FontSlant::Upright),
                    )
                    .child(
                        label()
                            .text("Italic")
                            .font_size(18.)
                            .font_slant(FontSlant::Italic),
                    )
                    .child(
                        label()
                            .text("Oblique")
                            .font_size(18.)
                            .font_slant(FontSlant::Oblique),
                    ),
            ))
            .child(section(
                "Font width",
                rect()
                    .horizontal()
                    .spacing(16.)
                    .child(
                        label()
                            .text("Condensed")
                            .font_size(18.)
                            .font_width(FontWidth::CONDENSED),
                    )
                    .child(
                        label()
                            .text("Normal")
                            .font_size(18.)
                            .font_width(FontWidth::NORMAL),
                    )
                    .child(
                        label()
                            .text("Expanded")
                            .font_size(18.)
                            .font_width(FontWidth::EXPANDED),
                    ),
            ))
            .child(section(
                "Letter spacing",
                rect()
                    .vertical()
                    .spacing(8.)
                    .child(
                        label()
                            .text("Tight letter spacing (-1.5)")
                            .font_size(20.)
                            .letter_spacing(-1.5),
                    )
                    .child(label().text("Normal letter spacing (0)").font_size(20.))
                    .child(
                        label()
                            .text("Wide letter spacing (4)")
                            .font_size(20.)
                            .letter_spacing(4.),
                    )
                    .child(
                        label()
                            .text("Very wide letter spacing (10)")
                            .font_size(20.)
                            .letter_spacing(10.),
                    ),
            ))
            .child(section(
                "Text decoration",
                rect()
                    .horizontal()
                    .spacing(16.)
                    .child(
                        label()
                            .text("Underline")
                            .font_size(18.)
                            .text_decoration(TextDecoration::Underline),
                    )
                    .child(
                        label()
                            .text("Overline")
                            .font_size(18.)
                            .text_decoration(TextDecoration::Overline),
                    )
                    .child(
                        label()
                            .text("Line-through")
                            .font_size(18.)
                            .text_decoration(TextDecoration::LineThrough),
                    ),
            ))
            .child(section(
                "Color",
                rect()
                    .horizontal()
                    .spacing(16.)
                    .child(
                        label()
                            .text("Solid")
                            .font_size(24.)
                            .font_weight(FontWeight::BOLD)
                            .color((88, 101, 242)),
                    )
                    .child(
                        label()
                            .text("Gradient")
                            .font_size(24.)
                            .font_weight(FontWeight::BOLD)
                            .color(
                                LinearGradient::new()
                                    .angle(90.)
                                    .stop(((255, 100, 50), 0.))
                                    .stop(((100, 0, 255), 100.)),
                            ),
                    ),
            ))
            .child(section(
                "Text shadow",
                label()
                    .text("Shadowed text")
                    .font_size(28.)
                    .font_weight(FontWeight::BOLD)
                    .text_shadow(TextShadow::new(Color::from_rgb(0, 0, 0), (3., 3.), 4.)),
            ))
            .child(section(
                "Text align",
                rect()
                    .vertical()
                    .spacing(8.)
                    .child(align_box("Left", TextAlign::Left))
                    .child(align_box("Center", TextAlign::Center))
                    .child(align_box("Right", TextAlign::Right))
                    .child(align_box("Justify", TextAlign::Justify)),
            ))
            .child(section(
                "Text overflow",
                rect()
                    .horizontal()
                    .spacing(16.)
                    .child(overflow_box("Clip", TextOverflow::Clip))
                    .child(overflow_box("Ellipsis", TextOverflow::Ellipsis))
                    .child(overflow_box(
                        "Custom",
                        TextOverflow::Custom(" (more)".to_string()),
                    )),
            ))
            .child(section(
                "Line height",
                rect()
                    .horizontal()
                    .spacing(24.)
                    .child(
                        label()
                            .text("The quick brown fox jumps over the lazy dog")
                            .font_size(16.)
                            .width(Size::px(180.)),
                    )
                    .child(
                        label()
                            .text("The quick brown fox jumps over the lazy dog")
                            .font_size(16.)
                            .width(Size::px(180.))
                            .line_height(2.2),
                    ),
            )),
    )
}

fn section(title: &'static str, content: impl IntoElement) -> impl IntoElement {
    rect()
        .spacing(10.)
        .child(
            label()
                .text(title)
                .font_size(14.)
                .font_weight(FontWeight::BOLD)
                .color((150, 150, 160)),
        )
        .child(content)
}

fn align_box(caption: &'static str, text_align: TextAlign) -> impl IntoElement {
    rect()
        .background((245, 245, 248))
        .corner_radius(6.)
        .padding(10.)
        .width(Size::px(320.))
        .child(
            label()
                .text(format!(
                    "{caption}: the quick brown fox jumps over the lazy dog"
                ))
                .font_size(15.)
                .text_align(text_align),
        )
}

fn overflow_box(caption: &'static str, text_overflow: TextOverflow) -> impl IntoElement {
    rect()
        .vertical()
        .spacing(4.)
        .width(Size::px(160.))
        .child(label().text(caption).font_size(12.).color((150, 150, 160)))
        .child(
            rect()
                .width(Size::px(160.))
                .background((245, 245, 248))
                .corner_radius(6.)
                .padding(8.)
                .overflow(Overflow::Clip)
                .child(
                    label()
                        .text("This sentence is definitely too long to fit in this box")
                        .font_size(15.)
                        .max_lines(1)
                        .text_overflow(text_overflow),
                ),
        )
}
