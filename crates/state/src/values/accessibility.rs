pub use accesskit::{
    AriaCurrent as Current, AutoComplete, CustomAction, DefaultActionVerb, HasPopup,
    Invalid as InvalidReason, ListStyle, Live, NodeId, Orientation, Role, SortDirection,
    TextSelection, Toggled, VerticalOffset,
};
use freya_engine::prelude::Color;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct NumericValue {
    pub value: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub jump: Option<f64>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScrollValue {
    pub value: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct CellIndex {
    pub index: usize,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Invalid {
    pub reason: InvalidReason,
    pub error_message: Option<NodeId>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccessibilityOptions {
    pub role: Role,
    pub is_focusable: bool,

    // Vec<NodeId>
    pub details: Option<Vec<NodeId>>,
    pub controls: Option<Vec<NodeId>>,
    pub described_by: Option<Vec<NodeId>>,
    pub flow_to: Option<Vec<NodeId>>,
    pub named_by: Option<Vec<NodeId>>,
    pub owns: Option<Vec<NodeId>>,
    pub radio_group: Option<Vec<NodeId>>,

    // NodeId
    pub active_descendant: Option<NodeId>,
    pub in_page_link_target: Option<NodeId>,
    pub member_of: Option<NodeId>,
    pub next_on_line: Option<NodeId>,
    pub previous_on_line: Option<NodeId>,
    pub popup_for: Option<NodeId>,

    // String
    pub name: Option<String>,
    pub description: Option<String>,
    pub value: Option<String>,
    pub author_id: Option<String>,
    // pub class_name: Option<String>,
    // pub html_tag: Option<String>,
    // pub inner_html: Option<String>,
    pub keyboard_shortcuts: Option<String>,
    pub language: Option<String>,
    pub placeholder: Option<String>,
    pub role_description: Option<String>,
    pub state_description: Option<String>,
    pub tooltip: Option<String>,
    pub url: Option<String>,

    // char
    pub access_key: Option<char>,

    // usize
    pub row_count: Option<usize>,
    pub column_count: Option<usize>,
    pub row_span: Option<usize>,
    pub column_span: Option<usize>,
    pub level: Option<usize>,
    pub size_of_set: Option<usize>,
    pub position_in_set: Option<usize>,

    // bool
    pub is_disabled: Option<bool>,
    pub is_expanded: Option<bool>,
    pub is_selected: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_linked: Option<bool>,
    pub is_multiselectable: Option<bool>,
    pub is_required: Option<bool>,
    pub is_visited: Option<bool>,
    pub is_busy: Option<bool>,
    pub is_live_atomic: Option<bool>,
    pub is_modal: Option<bool>,
    pub is_touch_transparent: Option<bool>,
    pub is_read_only: Option<bool>,
    pub is_grammar_error: Option<bool>,
    pub is_spelling_error: Option<bool>,
    pub is_search_match: Option<bool>,
    pub is_suggestion: Option<bool>,

    // Color
    pub color_value: Option<Color>,

    // Other
    pub vertical_offset: Option<VerticalOffset>,
    pub numeric_value: Option<NumericValue>,
    pub scroll_x: Option<ScrollValue>,
    pub scroll_y: Option<ScrollValue>,
    pub row_index: Option<CellIndex>,
    pub column_index: Option<CellIndex>,
    pub has_popup: Option<HasPopup>,
    pub list_style: Option<ListStyle>,
    pub sort_direction: Option<SortDirection>,
    pub auto_complete: Option<AutoComplete>,
    pub orientation: Option<Orientation>,
    pub current: Option<Current>,
    pub default_action_verb: Option<DefaultActionVerb>,
    pub toggled: Option<Toggled>,
    pub live: Option<Live>,
    pub invalid: Option<Invalid>,
    pub custom_actions: Option<Vec<CustomAction>>,
    pub text_selection: Option<TextSelection>,
}
