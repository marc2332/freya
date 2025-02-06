use super::grid_size::GridSize;

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
    pub fn is_fit(&self) -> bool {
        self == &Self::Fit
    }

    pub fn is_flex(&self) -> bool {
        self == &Self::Flex
    }

    pub fn is_grid(&self) -> bool {
        matches!(self, Self::Grid { .. })
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
