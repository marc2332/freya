use freya_core::parsing::Parse;
use freya_engine::prelude::*;

#[test]
fn parse_blend_mode() {
    let blend_mode = BlendMode::parse("color-dodge");
    assert_eq!(blend_mode, Ok(BlendMode::ColorDodge));
}

#[test]
fn parse_invalid_blend_modes() {
    let incorrect_name = BlendMode::parse("rust");
    assert!(incorrect_name.is_err());
}
