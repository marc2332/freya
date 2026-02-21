pub mod constants;
pub mod editor_data;
pub mod editor_line;
pub mod editor_ui;
pub mod languages;
pub mod metrics;
pub mod syntax;

pub mod prelude {
    pub use ropey::Rope;

    pub use crate::{
        constants::{
            BASE_FONT_SIZE,
            MAX_FONT_SIZE,
        },
        editor_data::{
            CodeEditorData,
            CodeEditorMetadata,
        },
        editor_line::EditorLineUI,
        editor_ui::CodeEditor,
        languages::LanguageId,
        metrics::EditorMetrics,
        syntax::{
            InputEditExt,
            RopeChunkIter,
            RopeTextProvider,
            SyntaxBlocks,
            SyntaxHighlighter,
            SyntaxLine,
            TextNode,
        },
    };
}
