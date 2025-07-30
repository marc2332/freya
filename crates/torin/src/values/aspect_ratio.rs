#[derive(Default, Clone, Debug, PartialEq)]
pub enum AspectRatio {
    #[default]
    None,
    Fit,
    Fill,
}
