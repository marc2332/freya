use tree_sitter_highlight::*;
use tree_sitter_javascript::*;

fn main() {
    let mut highlight_names = &mut [
        "attribute",
        "constant",
        "function.builtin",
        "function",
        "keyword",
        "operator",
        "property",
        "punctuation",
        "punctuation.bracket",
        "punctuation.delimiter",
        "string",
        "string.special",
        "tag",
        "type",
        "type.builtin",
        "variable",
        "variable.builtin",
        "variable.parameter",
    ];

    let mut highlighter = Highlighter::new();
    let javascript_language = unsafe { tree_sitter_javascript::language() };

    let mut javascript_config = HighlightConfiguration::new(
        tree_sitter_javascript::language(),
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::INJECTION_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    )
    .unwrap();
    javascript_config.configure(highlight_names);

    let highlights = highlighter
        .highlight(&javascript_config, b"const x = new Y();", None, |_| None)
        .unwrap();

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                eprintln!("source: {}-{}", start, end);
            }
            HighlightEvent::HighlightStart(s) => {
                eprintln!("highlight style started: {:?}", highlight_names[s.0]);
            }
            HighlightEvent::HighlightEnd => {
                eprintln!("highlight style ended");
            }
        }
    }
}
