use freya_core::prelude::Color;

#[derive(Clone, PartialEq)]
pub struct EditorTheme {
    pub background: Color,
    pub gutter_selected: Color,
    pub gutter_unselected: Color,
    pub line_selected_background: Color,
    pub cursor: Color,
    pub highlight: Color,
    pub text: Color,
    pub whitespace: Color,
}

#[derive(Clone, PartialEq)]
pub struct SyntaxTheme {
    pub text: Color,
    pub whitespace: Color,
    pub attribute: Color,
    pub boolean: Color,
    pub comment: Color,
    pub constant: Color,
    pub constructor: Color,
    pub escape: Color,
    pub function: Color,
    pub function_macro: Color,
    pub function_method: Color,
    pub keyword: Color,
    pub label: Color,
    pub module: Color,
    pub number: Color,
    pub operator: Color,
    pub property: Color,
    pub punctuation: Color,
    pub punctuation_bracket: Color,
    pub punctuation_delimiter: Color,
    pub punctuation_special: Color,
    pub string: Color,
    pub string_escape: Color,
    pub string_special: Color,
    pub tag: Color,
    pub text_literal: Color,
    pub text_reference: Color,
    pub text_title: Color,
    pub text_uri: Color,
    pub text_emphasis: Color,
    pub type_: Color,
    pub variable: Color,
    pub variable_builtin: Color,
    pub variable_parameter: Color,
}

impl Default for SyntaxTheme {
    fn default() -> Self {
        DEFAULT_SYNTAX_THEME
    }
}

impl Default for EditorTheme {
    fn default() -> Self {
        DEFAULT_EDITOR_THEME
    }
}

pub const DEFAULT_EDITOR_THEME: EditorTheme = EditorTheme {
    background: Color::from_rgb(29, 32, 33),
    gutter_selected: Color::from_rgb(235, 235, 235),
    gutter_unselected: Color::from_rgb(135, 135, 135),
    line_selected_background: Color::from_rgb(55, 55, 55),
    cursor: Color::WHITE,
    highlight: Color::from_rgb(80, 80, 80),
    text: Color::WHITE,
    whitespace: Color::from_af32rgb(0.2, 223, 191, 142),
};

pub const DEFAULT_SYNTAX_THEME: SyntaxTheme = SyntaxTheme {
    text: Color::from_rgb(235, 219, 178),
    whitespace: Color::from_af32rgb(0.2, 223, 191, 142),
    attribute: Color::from_rgb(131, 165, 152),
    boolean: Color::from_rgb(211, 134, 155),
    comment: Color::from_rgb(146, 131, 116),
    constant: Color::from_rgb(211, 134, 155),
    constructor: Color::from_rgb(250, 189, 47),
    escape: Color::from_rgb(254, 128, 25),
    function: Color::from_rgb(152, 192, 124),
    function_macro: Color::from_rgb(131, 165, 152),
    function_method: Color::from_rgb(152, 192, 124),
    keyword: Color::from_rgb(251, 73, 52),
    label: Color::from_rgb(211, 134, 155),
    module: Color::from_rgb(250, 189, 47),
    number: Color::from_rgb(211, 134, 155),
    operator: Color::from_rgb(104, 157, 96),
    property: Color::from_rgb(152, 192, 124),
    punctuation: Color::from_rgb(104, 157, 96),
    punctuation_bracket: Color::from_rgb(254, 128, 25),
    punctuation_delimiter: Color::from_rgb(104, 157, 96),
    punctuation_special: Color::from_rgb(131, 165, 152),
    string: Color::from_rgb(184, 187, 38),
    string_escape: Color::from_rgb(254, 128, 25),
    string_special: Color::from_rgb(184, 187, 38),
    tag: Color::from_rgb(131, 165, 152),
    text_literal: Color::from_rgb(235, 219, 178),
    text_reference: Color::from_rgb(131, 165, 152),
    text_title: Color::from_rgb(250, 189, 47),
    text_uri: Color::from_rgb(104, 157, 96),
    text_emphasis: Color::from_rgb(235, 219, 178),
    type_: Color::from_rgb(250, 189, 47),
    variable: Color::from_rgb(235, 219, 178),
    variable_builtin: Color::from_rgb(211, 134, 155),
    variable_parameter: Color::from_rgb(235, 219, 178),
};
