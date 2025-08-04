use freya_core::{
    parsing::Parse,
    values::{
        Color,
        TextShadow,
    },
};

#[test]
fn parse_text_shadow() {
    let text_shadow = TextShadow::parse("1 5 12 rgb(255, 0, 0)");
    assert_eq!(
        text_shadow,
        Ok(TextShadow {
            color: Color::RED,
            offset: (1.0, 5.0),
            blur_sigma: 6.0
        })
    );
}
