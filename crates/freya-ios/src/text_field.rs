use std::cell::Cell;

use freya_core::{
    integration::*,
    prelude::*,
};
use freya_winit::plugins::PluginHandle;
use objc2::{
    DefinedClass,
    MainThreadMarker,
    MainThreadOnly,
    define_class,
    msg_send,
    rc::Retained,
    runtime::{
        AnyObject,
        NSObject,
        ProtocolObject,
    },
    sel,
};
use objc2_core_foundation::{
    CGPoint,
    CGRect,
    CGSize,
};
use objc2_foundation::NSObjectProtocol;
use objc2_ui_kit::{
    UIControl,
    UIControlEvents,
    UIKeyboardType,
    UIResponder,
    UIReturnKeyType,
    UITextAutocapitalizationType,
    UITextAutocorrectionType,
    UITextField,
    UITextFieldDelegate,
    UITextInlinePredictionType,
    UITextInput,
    UITextSmartDashesType,
    UITextSmartInsertDeleteType,
    UITextSmartQuotesType,
    UITextSpellCheckingType,
    UIView,
};
use winit::window::WindowId;

use crate::keyboard::KeyboardTraits;

/// Forwards what the user types in the native text field to a Freya window.
pub(crate) struct TextInputSink {
    handle: PluginHandle,
    window_id: WindowId,
}

impl TextInputSink {
    pub(crate) fn new(handle: PluginHandle, window_id: WindowId) -> Self {
        Self { handle, window_id }
    }

    fn preedit(&self, text: String) {
        let cursor = (!text.is_empty()).then_some((text.len(), text.len()));
        self.handle.send_platform_event(
            PlatformEvent::ImePreedit {
                name: ImeEventName::Preedit,
                text,
                cursor,
            },
            self.window_id,
        );
    }

    fn commit(&self, text: String) {
        self.handle.send_platform_event(
            PlatformEvent::Keyboard {
                name: KeyboardEventName::KeyDown,
                key: Key::Character(text),
                code: Code::Unidentified,
                modifiers: Modifiers::empty(),
            },
            self.window_id,
        );
    }

    fn press(&self, key: NamedKey, code: Code) {
        for name in [KeyboardEventName::KeyDown, KeyboardEventName::KeyUp] {
            self.handle.send_platform_event(
                PlatformEvent::Keyboard {
                    name,
                    key: Key::Named(key),
                    code,
                    modifiers: Modifiers::empty(),
                },
                self.window_id,
            );
        }
    }
}

pub(crate) struct TextFieldIvars {
    sink: TextInputSink,
    composing: Cell<bool>,
    traits: Cell<KeyboardTraits>,
}

define_class!(
    // SAFETY:
    // - UITextField has no subclassing requirements beyond initializing it through `initWithFrame:`.
    // - `TextField` does not implement `Drop`.
    #[unsafe(super(UITextField, UIControl, UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "FreyaTextField"]
    #[ivars = TextFieldIvars]
    /// Invisible text field that owns the soft keyboard.
    ///
    /// UIKit handles composition (IME), keyboard types and return keys on it, and every change is
    /// forwarded to Freya. Committed text is cleared right away, so the field only ever holds the
    /// text that is still being composed.
    pub(crate) struct TextField;

    impl TextField {
        #[unsafe(method(deleteBackward))]
        fn delete_backward(&self) {
            if self.ivars().composing.get() {
                // SAFETY: `deleteBackward` takes no arguments and returns nothing.
                let _: () = unsafe { msg_send![super(self), deleteBackward] };
            } else {
                self.ivars().sink.press(NamedKey::Backspace, Code::Backspace);
            }
        }

        #[unsafe(method(freyaEditingChanged:))]
        fn editing_changed(&self, _sender: &AnyObject) {
            self.sync();
        }

        // UITextField forwards its UITextInputTraits setters to a private object, so they cannot
        // be called directly. UIKit reads the traits through these getters instead.

        #[unsafe(method(keyboardType))]
        fn keyboard_type(&self) -> UIKeyboardType {
            self.ivars().traits.get().keyboard_type
        }

        #[unsafe(method(returnKeyType))]
        fn return_key_type(&self) -> UIReturnKeyType {
            self.ivars().traits.get().return_key
        }

        #[unsafe(method(isSecureTextEntry))]
        fn is_secure_text_entry(&self) -> bool {
            self.ivars().traits.get().secure
        }

        // The field is emptied after every commit, so UIKit never sees the surrounding text.
        // Corrections and smart punctuation based on that empty context would rewrite the input.

        #[unsafe(method(autocorrectionType))]
        fn autocorrection_type(&self) -> UITextAutocorrectionType {
            UITextAutocorrectionType::No
        }

        #[unsafe(method(autocapitalizationType))]
        fn autocapitalization_type(&self) -> UITextAutocapitalizationType {
            UITextAutocapitalizationType::None
        }

        #[unsafe(method(spellCheckingType))]
        fn spell_checking_type(&self) -> UITextSpellCheckingType {
            UITextSpellCheckingType::No
        }

        #[unsafe(method(smartQuotesType))]
        fn smart_quotes_type(&self) -> UITextSmartQuotesType {
            UITextSmartQuotesType::No
        }

        #[unsafe(method(smartDashesType))]
        fn smart_dashes_type(&self) -> UITextSmartDashesType {
            UITextSmartDashesType::No
        }

        #[unsafe(method(smartInsertDeleteType))]
        fn smart_insert_delete_type(&self) -> UITextSmartInsertDeleteType {
            UITextSmartInsertDeleteType::No
        }

        #[unsafe(method(inlinePredictionType))]
        fn inline_prediction_type(&self) -> UITextInlinePredictionType {
            UITextInlinePredictionType::No
        }
    }

    unsafe impl NSObjectProtocol for TextField {}

    unsafe impl UITextFieldDelegate for TextField {
        #[unsafe(method(textFieldShouldReturn:))]
        fn text_field_should_return(&self, _text_field: &UITextField) -> bool {
            self.ivars().sink.press(NamedKey::Enter, Code::Enter);
            false
        }
    }
);

impl TextField {
    pub(crate) fn new(mtm: MainThreadMarker, sink: TextInputSink) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(TextFieldIvars {
            sink,
            composing: Cell::new(false),
            traits: Cell::new(KeyboardTraits::default()),
        });
        let frame = CGRect::new(CGPoint::new(0., 0.), CGSize::new(1., 1.));
        // SAFETY: `initWithFrame:` is the designated initializer of UIView subclasses.
        let this: Retained<Self> = unsafe { msg_send![super(this), initWithFrame: frame] };

        this.setAlpha(0.);
        this.setDelegate(Some(ProtocolObject::from_ref(&*this)));
        // SAFETY: The target is the field itself, which outlives its own control events, and
        // `freyaEditingChanged:` is defined above with the expected `(id)sender` signature.
        unsafe {
            this.addTarget_action_forControlEvents(
                Some(&this),
                sel!(freyaEditingChanged:),
                UIControlEvents::EditingChanged,
            );
        }

        this
    }

    /// Change the keyboard layout, reloading it if it is already on screen.
    pub(crate) fn set_traits(&self, traits: KeyboardTraits) {
        if self.ivars().traits.replace(traits) != traits && self.isFirstResponder() {
            self.reloadInputViews();
        }
    }

    /// Discard any text that is still being composed.
    pub(crate) fn reset(&self) {
        if self.ivars().composing.replace(false) {
            self.ivars().sink.preedit(String::new());
        }
        self.setText(None);
    }

    fn sync(&self) {
        let ivars = self.ivars();
        let text = UITextField::text(self)
            .map(|text| text.to_string())
            .unwrap_or_default();

        if self.markedTextRange().is_some() {
            ivars.composing.set(true);
            ivars.sink.preedit(text);
            return;
        }

        if ivars.composing.replace(false) {
            ivars.sink.preedit(String::new());
        }
        if !text.is_empty() {
            self.setText(None);
            ivars.sink.commit(text);
        }
    }
}
