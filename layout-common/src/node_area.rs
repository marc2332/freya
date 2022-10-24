/// An area starting at point `x` and `y` with a certain `width` and `height`.
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct NodeArea {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
