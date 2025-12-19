/// Strategy focusing an Accessibility Node.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AccessibilityFocusMovement {
    InsideGroup,
    OutsideGroup,
}

/// Strategy focusing an Accessibility Node.
#[derive(PartialEq, Debug, Clone)]
pub enum AccessibilityFocusStrategy {
    Forward(AccessibilityFocusMovement),
    Backward(AccessibilityFocusMovement),
    Node(accesskit::NodeId),
}

impl AccessibilityFocusStrategy {
    pub fn mode(&self) -> Option<AccessibilityFocusMovement> {
        match self {
            Self::Forward(mode) => Some(*mode),
            Self::Backward(mode) => Some(*mode),
            _ => None,
        }
    }
}
