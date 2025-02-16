use super::grid::GridSize;

#[derive(PartialEq, Clone, Debug, Default)]
pub enum Content {
    #[default]
    Normal,
    Fit,
    Flex,
    Grid {
        columns: Vec<GridSize>,
        rows: Vec<GridSize>,
    },
}

impl Content {
    pub fn is_same_type(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Normal, Self::Normal)
                | (Self::Fit, Self::Fit)
                | (Self::Flex, Self::Flex)
                | (Self::Grid { .. }, Self::Grid { .. })
        )
    }

    pub fn new_grid() -> Self {
        Self::Grid {
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn is_fit(&self) -> bool {
        self == &Self::Fit
    }

    pub fn is_flex(&self) -> bool {
        self == &Self::Flex
    }

    pub fn is_grid(&self) -> bool {
        matches!(self, Self::Grid { .. })
    }

    pub fn set_columns(&mut self, values: Vec<GridSize>) {
        if !self.is_grid() {
            *self = Self::new_grid();
        }

        if let Self::Grid { rows, .. } = self {
            *rows = values;
        }
    }

    pub fn set_rows(&mut self, values: Vec<GridSize>) {
        if !self.is_grid() {
            *self = Self::new_grid();
        }

        if let Self::Grid { rows, .. } = self {
            *rows = values;
        }
    }
}

impl Content {
    pub fn pretty(&self) -> String {
        match self {
            Self::Normal => "normal",
            Self::Fit => "fit",
            Self::Flex => "flex",
            Self::Grid { .. } => "grid",
        }
        .to_owned()
    }
}
