use freya_core::prelude::consume_root_context;
use freya_engine::prelude::*;
use ropey::Rope;
use tree_sitter::InputEdit;

use crate::{
    languages::LanguageId,
    syntax::*,
};

pub struct EditorMetrics {
    pub(crate) syntax_blocks: SyntaxBlocks,
    pub(crate) longest_width: f32,
    pub(crate) highlighter: SyntaxHighlighter,
}

impl EditorMetrics {
    pub fn new() -> Self {
        Self {
            syntax_blocks: SyntaxBlocks::default(),
            longest_width: 0.0,
            highlighter: SyntaxHighlighter::new(),
        }
    }

    pub fn measure_longest_line(&mut self, font_size: f32, rope: &Rope) {
        // We assume the font used is monospaced.

        // Calculate character width by measuring a reference character
        let font_collection = consume_root_context::<FontCollection>();
        let mut paragraph_style = ParagraphStyle::default();
        let mut text_style = TextStyle::default();
        text_style.set_font_size(font_size);
        text_style.set_font_families(&["Jetbrains Mono"]);
        paragraph_style.set_text_style(&text_style);
        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

        // Measure a single character to get the monospace character width
        paragraph_builder.add_text("W");
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(f32::MAX);
        let char_width = paragraph.longest_line();

        // Find the line with the maximum character count
        let max_chars = rope.lines().map(|line| line.len_chars()).max().unwrap_or(0);

        self.longest_width = max_chars as f32 * char_width;
    }

    pub fn run_parser(&mut self, rope: &Rope, language_id: LanguageId, edit: Option<InputEdit>) {
        self.highlighter.set_language(language_id);
        self.highlighter.parse(rope, &mut self.syntax_blocks, edit);
    }
}
