#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug, Default, Copy)]
pub enum Order {
    /// Lay children out in their natural order. This is the default.
    #[default]
    Forward,
    /// Lay children out in reverse order along the direction axis.
    Backward,
}

impl Order {
    /// Use a [`Forward`](Order::Forward) order.
    pub fn forward() -> Order {
        Order::Forward
    }

    /// Use a [`Backward`](Order::Backward) order.
    pub fn backward() -> Order {
        Order::Backward
    }

    /// Whether children are laid out in reverse order.
    pub fn is_reverse(&self) -> bool {
        matches!(self, Self::Backward)
    }

    pub fn pretty(&self) -> String {
        match self {
            Self::Forward => "forward".to_string(),
            Self::Backward => "backward".to_string(),
        }
    }
}
