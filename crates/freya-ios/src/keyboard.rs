use std::cell::Cell;

use freya_core::prelude::*;
use freya_winit::{
    integration::is_ime_role,
    plugins::PluginHandle,
};
use objc2::{
    MainThreadMarker,
    rc::Retained,
};
use objc2_ui_kit::{
    UIKeyboardType,
    UIReturnKeyType,
    UIView,
};
use raw_window_handle::{
    HandleError,
    HasWindowHandle,
    RawWindowHandle,
};
use winit::window::Window;

use crate::text_field::{
    TextField,
    TextInputSink,
};

/// Why the soft keyboard could not be attached to a window.
#[derive(Debug)]
pub enum KeyboardAttachError {
    /// UIKit views can only be created on the main thread.
    NotMainThread,
    /// The window handle could not be read.
    WindowHandle(HandleError),
    /// The window is not backed by a UIKit view.
    NotUiKit,
}

impl From<HandleError> for KeyboardAttachError {
    fn from(error: HandleError) -> Self {
        Self::WindowHandle(error)
    }
}

/// Keyboard appearance for an accessibility role.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct KeyboardTraits {
    pub(crate) keyboard_type: UIKeyboardType,
    pub(crate) return_key: UIReturnKeyType,
    pub(crate) secure: bool,
}

impl Default for KeyboardTraits {
    fn default() -> Self {
        Self::from(AccessibilityRole::TextInput)
    }
}

impl From<AccessibilityRole> for KeyboardTraits {
    fn from(role: AccessibilityRole) -> Self {
        let keyboard_type = match role {
            AccessibilityRole::EmailInput => UIKeyboardType::EmailAddress,
            AccessibilityRole::NumberInput => UIKeyboardType::DecimalPad,
            AccessibilityRole::PhoneNumberInput => UIKeyboardType::PhonePad,
            AccessibilityRole::UrlInput => UIKeyboardType::URL,
            AccessibilityRole::SearchInput => UIKeyboardType::WebSearch,
            _ => UIKeyboardType::Default,
        };
        let return_key = match role {
            AccessibilityRole::SearchInput => UIReturnKeyType::Search,
            _ => UIReturnKeyType::Default,
        };
        Self {
            keyboard_type,
            return_key,
            secure: role == AccessibilityRole::PasswordInput,
        }
    }
}

/// Shows and hides the iOS soft keyboard for the focused node of a window.
pub(crate) struct SoftKeyboard {
    view: Retained<UIView>,
    text_field: Retained<TextField>,
    focused: Cell<Option<AccessibilityId>>,
}

impl SoftKeyboard {
    pub(crate) fn attach(
        window: &Window,
        handle: PluginHandle,
    ) -> Result<Self, KeyboardAttachError> {
        let mtm = MainThreadMarker::new().ok_or(KeyboardAttachError::NotMainThread)?;
        let RawWindowHandle::UiKit(uikit) = window.window_handle()?.as_raw() else {
            return Err(KeyboardAttachError::NotUiKit);
        };
        // SAFETY: winit guarantees `ui_view` points to the live UIView backing `window`, and
        // retaining it keeps it valid for as long as the text field needs it.
        let view = unsafe { Retained::retain(uikit.ui_view.as_ptr().cast::<UIView>()) }
            .ok_or(KeyboardAttachError::NotUiKit)?;

        let text_field = TextField::new(mtm, TextInputSink::new(handle, window.id()));
        view.addSubview(&text_field);

        Ok(Self {
            view,
            text_field,
            focused: Cell::new(None),
        })
    }

    /// Show a keyboard matching `role` while an input is focused, and hide it otherwise.
    pub(crate) fn update(&self, id: AccessibilityId, role: AccessibilityRole) {
        if !is_ime_role(role) {
            self.focused.set(None);
            if self.text_field.isFirstResponder() {
                self.text_field.reset();
                self.text_field.resignFirstResponder();
                // UIKit hands the first responder back to winit's view, which would then show
                // its own keyboard.
                self.view.resignFirstResponder();
            }
            return;
        }

        if self.focused.replace(Some(id)) != Some(id) {
            self.text_field.reset();
            self.text_field.set_traits(KeyboardTraits::from(role));
        }

        if !self.text_field.isFirstResponder() {
            self.text_field.becomeFirstResponder();
        }
    }
}
