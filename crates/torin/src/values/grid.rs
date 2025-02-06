use crate::geometry::Length;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct GridPosition {
    pub column: usize,
    pub column_span: usize,
    pub row: usize,
    pub row_span: usize,
}

impl GridPosition {
    pub fn new(column: usize, column_span: usize, row: usize, row_span: usize) -> Self {
        Self {
            column,
            column_span,
            row,
            row_span,
        }
    }
}

impl Default for GridPosition {
    fn default() -> Self {
        Self::new(0, 1, 0, 1)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum GridSize {
    Inner,
    Pixels(Length),
    Weight(Length),
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
        matches!(self, Self::Weight(_))
    }
}
