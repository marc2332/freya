use freya::{
    i18n::*,
    prelude::*,
};

#[derive(PartialEq)]
pub struct I18nShowcase;

impl Component for I18nShowcase {
    fn render(&self) -> impl IntoElement {
        let mut i18n = I18n::get();
        let mut count = use_state(|| 0);

        rect()
            .spacing(20.)
            .child(super::heading("Internationalization", "Multiple languages"))
            .child(
                rect()
                    .horizontal()
                    .spacing(8.)
                    .child(
                        Button::new()
                            .on_press(move |_| i18n.set_language(langid!("en-US")))
                            .child("English"),
                    )
                    .child(
                        Button::new()
                            .on_press(move |_| i18n.set_language(langid!("es-ES")))
                            .child("Español"),
                    ),
            )
            .child(rect().font_size(22.).child(t!("greeting")))
            .child(
                rect()
                    .horizontal()
                    .spacing(12.)
                    .cross_align(Alignment::center())
                    .child(
                        Button::new()
                            .on_press(move |_| *count.write() += 1)
                            .child("+1"),
                    )
                    .child(t!("counter", count: count().to_string())),
            )
    }
}
