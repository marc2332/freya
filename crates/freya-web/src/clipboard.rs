use std::{
    cell::{
        Cell,
        RefCell,
    },
    collections::HashMap,
    mem::take,
    rc::Rc,
};

use freya_clipboard::prelude::{
    ClipboardError,
    ClipboardFuture,
    ClipboardProvider,
    DataFormat,
    DataValue,
    TransferData,
};
use futures_channel::oneshot;
use serde::Deserialize;

use crate::emscripten::{
    run_script,
    run_script_string,
};

const SETUP_SCRIPT: &str = r#"(function() {
    var results = [];
    function finish(id, success, text, bytes) {
        results.push({ id: id, success: success, text: text, bytes: bytes });
    }
    window.__freyaClipboard = {
        read: function(id, type) {
            if (type === 'text/plain') {
                if (typeof window.__freyaClipboardPaste === 'string') {
                    var text = window.__freyaClipboardPaste;
                    delete window.__freyaClipboardPaste;
                    finish(id, true, text);
                    return;
                }
                if (navigator.clipboard && navigator.clipboard.readText) {
                    navigator.clipboard.readText().then(function(text) {
                        finish(id, true, text);
                    }, function() { finish(id, false); });
                    return;
                }
            }
            if (!navigator.clipboard || !navigator.clipboard.read) {
                finish(id, false);
                return;
            }
            navigator.clipboard.read().then(function(items) {
                var item = items.find(function(item) { return item.types.includes(type); });
                if (!item) throw new Error('Clipboard format unavailable');
                return item.getType(type);
            }).then(function(blob) {
                if (type === 'text/plain' || type === 'text/html' || type === 'application/rtf') {
                    return blob.text().then(function(text) { finish(id, true, text); });
                }
                return blob.arrayBuffer().then(function(buffer) {
                    finish(id, true, null, Array.from(new Uint8Array(buffer)));
                });
            }).catch(function() { finish(id, false); });
        },
        write: function(id, entries) {
            try {
                if (entries.length === 1 && entries[0].type === 'text/plain') {
                    var entry = entries[0];
                    var text = entry.text !== undefined ? entry.text :
                        new TextDecoder('utf-8', { fatal: true }).decode(new Uint8Array(entry.bytes));
                    if (navigator.clipboard && navigator.clipboard.writeText) {
                        navigator.clipboard.writeText(text).then(function() {
                            finish(id, true);
                        }, function() { finish(id, false); });
                        return;
                    }
                    var active = document.activeElement;
                    var input = document.createElement('textarea');
                    input.value = text;
                    input.style.position = 'fixed';
                    input.style.opacity = '0';
                    document.body.appendChild(input);
                    input.focus();
                    input.select();
                    var success = false;
                    try { success = document.execCommand('copy'); } catch (error) {}
                    input.remove();
                    if (active && active.focus) active.focus();
                    finish(id, success);
                    return;
                }
                if (!navigator.clipboard || !navigator.clipboard.write || !window.ClipboardItem) {
                    finish(id, false);
                    return;
                }
                var data = {};
                entries.forEach(function(entry) {
                    var value = entry.text !== undefined ? entry.text : new Uint8Array(entry.bytes);
                    data[entry.type] = new Blob([value], { type: entry.type });
                });
                navigator.clipboard.write([new ClipboardItem(data)]).then(function() {
                    finish(id, true);
                }, function() { finish(id, false); });
            } catch (error) { finish(id, false); }
        },
        take: function() {
            return results.length ? JSON.stringify(results.shift()) : '';
        },
    };
})();"#;

enum PendingClipboard {
    Read(oneshot::Sender<Result<DataValue, ClipboardError>>),
    Write(oneshot::Sender<Result<(), ClipboardError>>),
}

impl PendingClipboard {
    fn complete(self, success: bool, text: Option<String>, bytes: Option<Vec<u8>>) {
        match self {
            Self::Read(response) => {
                let result = match (success, text, bytes) {
                    (true, Some(text), None) => Ok(DataValue::Text(text)),
                    (true, None, Some(bytes)) => Ok(DataValue::Bytes(bytes)),
                    _ => Err(ClipboardError::FailedToRead),
                };
                let _ = response.send(result);
            }
            Self::Write(response) => {
                let result = if success {
                    Ok(())
                } else {
                    Err(ClipboardError::FailedToSet)
                };
                let _ = response.send(result);
            }
        }
    }
}

#[derive(Default)]
struct WebClipboardState {
    next_request: Cell<u32>,
    pending: RefCell<HashMap<u32, PendingClipboard>>,
}

#[derive(Deserialize)]
struct ClipboardResult {
    id: u32,
    success: bool,
    text: Option<String>,
    bytes: Option<Vec<u8>>,
}

#[derive(Clone)]
pub(crate) struct WebClipboard(Rc<WebClipboardState>);

impl WebClipboard {
    pub(crate) fn new() -> Self {
        run_script(SETUP_SCRIPT);
        Self(Rc::default())
    }

    pub(crate) fn poll(&self) {
        while let Some(result) = run_script_string("window.__freyaClipboard.take();") {
            if result.is_empty() {
                break;
            }
            let Ok(result) = serde_json::from_str::<ClipboardResult>(&result) else {
                let pending = take(&mut *self.0.pending.borrow_mut());
                for pending in pending.into_values() {
                    pending.complete(false, None, None);
                }
                break;
            };
            let pending = self.0.pending.borrow_mut().remove(&result.id);
            if let Some(pending) = pending {
                pending.complete(result.success, result.text, result.bytes);
            }
        }
    }
}

impl ClipboardProvider for WebClipboard {
    fn get(&self, format: DataFormat) -> ClipboardFuture<DataValue> {
        if format == DataFormat::Files {
            return Box::pin(async { Err(ClipboardError::UnsupportedFormat) });
        }
        let (response, receiver) = oneshot::channel();
        let id = self.0.next_request.get();
        self.0.next_request.set(id.wrapping_add(1));
        self.0
            .pending
            .borrow_mut()
            .insert(id, PendingClipboard::Read(response));
        run_script(&format!(
            "window.__freyaClipboard.read({id}, '{}');",
            format.mime_type()
        ));
        Box::pin(async move { receiver.await.map_err(|_| ClipboardError::FailedToRead)? })
    }

    fn set(&self, data: TransferData) -> ClipboardFuture<()> {
        if data.formats.is_empty() {
            return Box::pin(async { Err(ClipboardError::FailedToSet) });
        }
        let mut entries = Vec::new();
        for (format, value) in data.formats {
            if format == DataFormat::Files {
                return Box::pin(async { Err(ClipboardError::UnsupportedFormat) });
            }
            let entry = match value {
                DataValue::Text(text) => {
                    serde_json::json!({ "type": format.mime_type(), "text": text })
                }
                DataValue::Bytes(bytes) => {
                    serde_json::json!({ "type": format.mime_type(), "bytes": bytes })
                }
                DataValue::Files(_) => return Box::pin(async { Err(ClipboardError::FailedToSet) }),
            };
            entries.push(entry);
        }
        let Ok(contents) = serde_json::to_string(&entries) else {
            return Box::pin(async { Err(ClipboardError::FailedToSet) });
        };
        let (response, receiver) = oneshot::channel();
        let id = self.0.next_request.get();
        self.0.next_request.set(id.wrapping_add(1));
        self.0
            .pending
            .borrow_mut()
            .insert(id, PendingClipboard::Write(response));
        run_script(&format!("window.__freyaClipboard.write({id}, {contents});"));
        Box::pin(async move { receiver.await.map_err(|_| ClipboardError::FailedToSet)? })
    }
}
