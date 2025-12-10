use keyboard_types::{
    Code,
    Modifiers,
};

use crate::{
    accessibility::id::AccessibilityId,
    integration::{
        ACCESSIBILITY_ROOT_ID,
        AccessibilityGenerator,
    },
    platform_state::{
        NavigationMode,
        PlatformState,
    },
    prelude::{
        AccessibilityFocusStrategy,
        KeyboardEventData,
        Memo,
        Platform,
        ScreenReader,
        UserEvent,
        consume_root_context,
        use_hook,
        use_memo,
    },
};

#[derive(Clone, Copy)]
pub struct Focus {
    a11y_id: AccessibilityId,
}

impl Focus {
    pub fn create() -> Self {
        Self::new_for_id(Self::new_id())
    }

    pub fn new_for_id(a11y_id: AccessibilityId) -> Self {
        Self { a11y_id }
    }

    pub fn new_id() -> AccessibilityId {
        let accessibility_generator = consume_root_context::<AccessibilityGenerator>();
        AccessibilityId(accessibility_generator.new_id())
    }

    pub fn a11y_id(&self) -> AccessibilityId {
        self.a11y_id
    }

    pub fn is_focused(&self) -> bool {
        let platform_state = PlatformState::get();
        *platform_state.focused_accessibility_id.peek() == self.a11y_id
    }

    pub fn is_focused_with_keyboard(&self) -> bool {
        let platform_state = PlatformState::get();
        *platform_state.focused_accessibility_id.peek() == self.a11y_id
            && *platform_state.navigation_mode.peek() == NavigationMode::Keyboard
    }

    pub fn request_focus(&self) {
        Platform::get().send(UserEvent::FocusAccessibilityNode(
            AccessibilityFocusStrategy::Node(self.a11y_id),
        ));
    }

    pub fn request_unfocus(&self) {
        Platform::get().send(UserEvent::FocusAccessibilityNode(
            AccessibilityFocusStrategy::Node(ACCESSIBILITY_ROOT_ID),
        ));
    }

    pub fn is_pressed(event: &KeyboardEventData) -> bool {
        if cfg!(target_os = "macos") {
            let screen_reader = ScreenReader::get();
            if screen_reader.is_on() {
                event.code == Code::Space
                    && event.modifiers.contains(Modifiers::CONTROL)
                    && event.modifiers.contains(Modifiers::ALT)
            } else {
                event.code == Code::Enter || event.code == Code::Space
            }
        } else {
            event.code == Code::Enter || event.code == Code::Space
        }
    }
}

pub fn use_focus() -> Focus {
    use_hook(Focus::create)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FocusStatus {
    Not,
    Pointer,
    Keyboard,
}

pub fn use_focus_status(focus: Focus) -> Memo<FocusStatus> {
    use_memo(move || {
        let platform_state = PlatformState::get();
        let is_focused = *platform_state.focused_accessibility_id.read() == focus.a11y_id;
        let is_keyboard = *platform_state.navigation_mode.read() == NavigationMode::Keyboard;

        match (is_focused, is_keyboard) {
            (true, false) => FocusStatus::Pointer,
            (true, true) => FocusStatus::Keyboard,
            _ => FocusStatus::Not,
        }
    })
}
