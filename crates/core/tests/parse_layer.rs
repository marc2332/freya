use freya_core::{
    parsing::Parse,
    values::LayerMode,
};
use freya_engine::prelude::*;

#[test]
fn parse_overlay() {
    let layer = LayerMode::parse("overlay");
    assert_eq!(layer, Ok(LayerMode::Overlay));
}

#[test]
fn parse_inherited() {
    let layer = LayerMode::parse("rust");
    assert_eq!(layer, Ok(LayerMode::Inherited));
}

#[test]
fn parse_relative() {
    let layer = LayerMode::parse("-128");
    assert_eq!(layer, Ok(LayerMode::Relative(-128)));
    let layer = LayerMode::parse("256");
    assert_eq!(layer, Ok(LayerMode::Relative(256)));
}
