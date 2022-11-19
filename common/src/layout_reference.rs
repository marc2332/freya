/// Area and some extra info of a given node on a given time
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeReferenceLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub inner_height: f32,
    pub inner_width: f32,
}
