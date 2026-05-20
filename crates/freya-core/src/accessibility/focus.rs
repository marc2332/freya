use keyboard_types::{
    Key,
    Modifiers,
    NamedKey,
};

use crate::{
    accessibility::id::AccessibilityId,
    integration::{
        ACCESSIBILITY_ROOT_ID,
        AccessibilityGenerator,
    },
    lifecycle::reactive::use_reactive,
    platform::{
        NavigationMode,
        Platform,
    },
    prelude::{
        AccessibilityFocusStrategy,
        KeyboardEventData,
        Memo,
        ScreenReader,
        UserEvent,
        consume_root_context,
        use_hook,
        use_memo,
    },
};

/// Extension trait for [`AccessibilityId`].
///
/// Pair an id with an element through `.a11y_id(...)`, then call any of these
/// methods on the id to interact with focus.
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// fn focusable_box() -> impl IntoElement {
///     let a11y_id = use_a11y();
///     rect()
///         .a11y_id(a11y_id)
///         .a11y_focusable(true)
///         .on_mouse_down(move |_| a11y_id.request_focus())
///         .child(if a11y_id.is_focused() {
///             "Focused"
///         } else {
///             "Not focused"
///         })
/// }
/// ```
pub trait AccessibilityIdExt {
    /// Whether the linked node is currently focused (via keyboard or pointer).
    fn is_focused(&self) -> bool;

    /// Request focus to be moved to the linked node.
    fn request_focus(&self);

    /// Request focus to be cleared from the linked node.
    fn request_unfocus(&self);

    /// Generate a unique [`AccessibilityId`]. Prefer [`use_a11y`] for component-scoped ids.
    fn new_unique() -> AccessibilityId;
}

impl AccessibilityIdExt for AccessibilityId {
    fn is_focused(&self) -> bool {
        let platform = Platform::get();
        *platform.focused_accessibility_id.read() == *self
    }

    fn request_focus(&self) {
        let platform = Platform::get();

        if *platform.focused_accessibility_id.peek() != *self {
            Platform::get().send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Node(*self),
            ));
        }
    }

    fn request_unfocus(&self) {
        let platform = Platform::get();

        if *platform.focused_accessibility_id.peek() == *self {
            Platform::get().send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Node(ACCESSIBILITY_ROOT_ID),
            ));
        }
    }

    fn new_unique() -> Self {
        let accessibility_generator = consume_root_context::<AccessibilityGenerator>();
        AccessibilityId(accessibility_generator.new_id())
    }
}

/// Create a unique [`AccessibilityId`] that persists for the lifetime of the component.
pub fn use_a11y() -> AccessibilityId {
    use_hook(AccessibilityId::new_unique)
}

/// Focus state for an [`AccessibilityId`], distinguishing keyboard vs pointer focus.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Focus {
    /// The node is not focused.
    Not,
    /// The node is focused after a pointer (mouse / touch) interaction.
    Pointer,
    /// The node is focused while the user is navigating with the keyboard.
    Keyboard,
}

impl Focus {
    /// Whether the node is focused, regardless of how it got focused.
    pub fn is_focused(&self) -> bool {
        matches!(self, Self::Pointer | Self::Keyboard)
    }
}

/// Extension trait for [`KeyboardEventData`] with focus-related helpers.
pub trait KeyboardEventExt {
    /// Whether this event is the "press" gesture for a focusable node (`Enter` / `Space`,
    /// or `Ctrl+Alt+Space` on macOS with a screen reader).
    fn is_press_event(&self) -> bool;
}

impl KeyboardEventExt for KeyboardEventData {
    fn is_press_event(&self) -> bool {
        let is_space = matches!(self.key, Key::Character(ref s) if s == " ");
        let is_enter = self.key == Key::Named(NamedKey::Enter);

        if cfg!(target_os = "macos") {
            let screen_reader = ScreenReader::get();
            if screen_reader.is_on() {
                is_space
                    && self.modifiers.contains(Modifiers::CONTROL)
                    && self.modifiers.contains(Modifiers::ALT)
            } else {
                is_enter || is_space
            }
        } else {
            is_enter || is_space
        }
    }
}

/// Reactively track the [`Focus`] state of an [`AccessibilityId`].
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// fn highlighted_box() -> impl IntoElement {
///     let a11y_id = use_a11y();
///     let focus = use_focus(a11y_id);
///     rect()
///         .a11y_id(a11y_id)
///         .a11y_focusable(true)
///         .maybe(focus() == Focus::Keyboard, |el| {
///             el.border(Border::new().fill(Color::BLUE).width(2.))
///         })
/// }
/// ```
pub fn use_focus(a11y_id: AccessibilityId) -> Memo<Focus> {
    let id = use_reactive(&a11y_id);
    use_memo(move || {
        let platform = Platform::get();
        let is_focused = *platform.focused_accessibility_id.read() == id();
        let is_keyboard = *platform.navigation_mode.read() == NavigationMode::Keyboard;

        match (is_focused, is_keyboard) {
            (true, false) => Focus::Pointer,
            (true, true) => Focus::Keyboard,
            _ => Focus::Not,
        }
    })
}
