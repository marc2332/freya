use std::error::Error;

use freya_clipboard::copypasta::ClipboardProvider;

use crate::emscripten::{
    js_string_literal,
    run_script,
    run_script_string,
};

/// Clipboard backed by the hidden input HTML input.
pub struct WebClipboard;

impl ClipboardProvider for WebClipboard {
    fn get_contents(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        run_script_string("window.__freyaClipboardPaste || '';")
            .filter(|text| !text.is_empty())
            .ok_or_else(|| "no clipboard content pasted yet".into())
    }

    fn set_contents(&mut self, contents: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        run_script(&format!(
            "window.__freyaIme.copy({});",
            js_string_literal(&contents)
        ));

        Ok(())
    }
}
