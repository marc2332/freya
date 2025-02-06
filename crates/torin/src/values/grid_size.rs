use crate::geometry::Length;

#[derive(PartialEq, Clone, Debug)]
pub enum GridSize {
    Inner,
    Pixels(Length),
    Stars(Length),
}

impl Default for GridSize {
    fn default() -> Self {
        Self::Inner
    }
}

impl GridSize {
    pub fn is_inner(&self) -> bool {
        matches!(self, Self::Inner)
    }

    pub fn is_weight(&self) -> bool {
        matches!(self, Self::Stars(_))
    }
}
