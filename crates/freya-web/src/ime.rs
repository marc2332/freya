use freya_core::{
    integration::*,
    prelude::{
        Code,
        Key,
        Modifiers,
    },
};
use torin::prelude::Area;

use crate::{
    emscripten::{
        run_script,
        run_script_string,
    },
    events::{
        ime_focused,
        set_ime_focused,
    },
};

const SETUP_SCRIPT: &str = r#"(function() {
    if (window.__freyaIme) return;

    var canvas = document.querySelector('#canvas');
    var input = document.createElement('input');
    input.id = 'freya-ime-input';
    input.autocomplete = 'off';
    input.autocapitalize = 'off';
    input.spellcheck = false;
    input.tabIndex = -1;
    input.style.position = 'fixed';
    input.style.opacity = '0';
    input.style.border = 'none';
    input.style.padding = '0';
    input.style.pointerEvents = 'none';
    input.style.caretColor = 'transparent';
    document.body.appendChild(input);

    var queue = [];
    var wanted = false;
    var composing = false;

    function commit() {
        if (input.value) queue.push('C' + input.value);
        input.value = '';
    }

    input.addEventListener('compositionstart', function() {
        composing = true;
    });
    input.addEventListener('compositionupdate', function(e) {
        queue.push('P' + (e.data || ''));
    });
    input.addEventListener('compositionend', function() {
        composing = false;
        queue.push('P');
        commit();
    });
    input.addEventListener('input', function() {
        if (!composing) commit();
    });
    input.addEventListener('paste', function(e) {
        var text = e.clipboardData && e.clipboardData.getData('text/plain');
        if (text) window.__freyaClipboardPaste = text;
        e.preventDefault();
    });
    canvas.addEventListener('mousedown', function(e) {
        if (wanted) {
            e.preventDefault();
            input.focus();
        }
    });

    window.__freyaIme = {
        take: function() {
            return queue.shift() || '';
        },
        focus: function(focused) {
            wanted = focused;
            if (focused) input.focus(); else input.blur();
        },
        area: function(left, top, width, height) {
            var rect = canvas.getBoundingClientRect();
            input.style.left = rect.left + left + 'px';
            input.style.top = rect.top + top + 'px';
            input.style.width = width + 'px';
            input.style.height = height + 'px';
        },
        copy: function(text) {
            var active = document.activeElement;
            input.value = text;
            input.focus();
            input.select();
            try { document.execCommand('copy'); } catch (e) {}
            input.value = '';
            if (active !== input) {
                input.blur();
                if (active && active.focus) active.focus();
            }
        },
    };
})();"#;

/// Hidden DOM input the browser routes IME, virtual keyboard and clipboard input through.
pub struct WebIme {
    area: Option<Area>,
}

impl WebIme {
    pub fn new() -> Self {
        run_script(SETUP_SCRIPT);

        Self { area: None }
    }

    /// Focuses and positions the input over the focused element, or blurs it if it takes no text.
    pub fn sync(&mut self, role: AccessibilityRole, area: Area, scale_factor: f64) {
        let focused = is_ime_role(role);
        if focused != ime_focused() {
            set_ime_focused(focused);
            run_script(&format!("window.__freyaIme.focus({focused});"));
        }

        if !focused {
            self.area = None;
            return;
        }

        if self.area != Some(area) {
            self.area = Some(area);
            run_script(&format!(
                "window.__freyaIme.area({}, {}, {}, {});",
                area.min_x() as f64 / scale_factor,
                area.min_y() as f64 / scale_factor,
                area.width() as f64 / scale_factor,
                area.height() as f64 / scale_factor,
            ));
        }
    }

    /// Pops the oldest queued text event, if any. Call in a loop to drain the queue.
    pub fn poll(&self) -> Option<PlatformEvent> {
        let text = run_script_string("window.__freyaIme.take();")?;
        let (tag, body) = text.split_at_checked(1)?;

        match tag {
            "C" => Some(PlatformEvent::Keyboard {
                name: KeyboardEventName::KeyDown,
                key: Key::Character(body.to_string()),
                code: Code::Unidentified,
                modifiers: Modifiers::empty(),
            }),
            "P" => Some(PlatformEvent::ImePreedit {
                name: ImeEventName::Preedit,
                text: body.to_string(),
                cursor: None,
            }),
            _ => None,
        }
    }
}
