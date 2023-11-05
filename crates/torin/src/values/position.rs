#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub enum Position {
    #[default]
    Stacked,

    Absolute,
}
