use std::sync::Arc;

use dioxus_core::{
    prelude::consume_context,
    use_hook,
    AttributeValue,
};
use dioxus_hooks::{
    use_context,
    use_memo,
};
use dioxus_signals::{
    Memo,
    ReadOnlySignal,
    Readable,
    Signal,
};
use freya_core::{
    accessibility::{
        AccessibilityFocusStrategy,
        AccessibilityGenerator,
        ACCESSIBILITY_ROOT_ID,
    },
    custom_attributes::CustomAttributeValues,
    event_loop_messages::EventLoopAppMessageAction,
    platform_state::NavigationMode,
    types::{
        AccessibilityId,
        AccessibilityNode,
    },
};
use freya_elements::events::{
    keyboard::Code,
    KeyboardEvent,
};

use crate::{
    use_platform,
    UsePlatform,
};

/// Manage the focus operations of given Node
#[derive(Clone, Copy)]
pub struct UseFocus {
    id: AccessibilityId,
    is_focused_with_keyboard: Memo<bool>,
    is_focused: Memo<bool>,
    navigation_mode: Signal<NavigationMode>,
    platform: UsePlatform,
    focused_id: Signal<AccessibilityId>,
    focused_node: Signal<AccessibilityNode>,
}

impl UseFocus {
    pub fn new_id() -> AccessibilityId {
        let accessibility_generator = consume_context::<Arc<AccessibilityGenerator>>();

        AccessibilityId(accessibility_generator.new_id())
    }

    /// Request to **focus** accessibility node. This will not immediately update [Self::is_focused].
    pub fn request_focus(&mut self) {
        if !*self.is_focused.peek() {
            self.platform
                .request_focus(AccessibilityFocusStrategy::Node(self.id));
        }
    }

    /// Request to **unfocus** accessibility node. This will not immediately update [Self::is_focused].
    pub fn request_unfocus(&mut self) {
        self.platform
            .send_app_event(EventLoopAppMessageAction::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Node(ACCESSIBILITY_ROOT_ID),
            ))
    }

    /// Focus a given [AccessibilityId].
    pub fn focus_id(id: AccessibilityId) {
        UsePlatform::current().request_focus(AccessibilityFocusStrategy::Node(id));
    }

    /// Get [AccessibilityId] of this accessibility node.
    pub fn id(&self) -> AccessibilityId {
        self.id
    }

    /// Create a [freya_elements::elements::rect::a11y_id] attribute value for this accessibility node.
    pub fn attribute(&self) -> AttributeValue {
        Self::attribute_for_id(self.id)
    }

    /// Create a [freya_elements::elements::rect::a11y_id] attribute value for a given [AccessibilityId].
    pub fn attribute_for_id(id: AccessibilityId) -> AttributeValue {
        AttributeValue::any_value(CustomAttributeValues::AccessibilityId(id))
    }

    /// Subscribe to focus changes where this node was involved.
    pub fn is_focused(&self) -> bool {
        *self.is_focused.read()
    }

    /// Subscribe to focus changes where this node was involved and the keyboard was used.
    pub fn is_focused_with_keyboard(&self) -> bool {
        *self.is_focused_with_keyboard.read()
            && *self.navigation_mode.read() == NavigationMode::Keyboard
    }

    /// Useful if you want to trigger an action when `Enter` or `Space` is pressed and this Node was focused with the keyboard.
    pub fn validate_keydown(&self, e: &KeyboardEvent) -> bool {
        (e.data.code == Code::Enter || e.data.code == Code::Space)
            && self.is_focused_with_keyboard()
    }

    /// Get a readable of the currently focused Node Id.
    pub fn focused_id(&self) -> ReadOnlySignal<AccessibilityId> {
        self.focused_id.into()
    }

    /// Get a readable of the currently focused Node.
    pub fn focused_node(&self) -> ReadOnlySignal<AccessibilityNode> {
        self.focused_node.into()
    }
}

/// Create a focus manager for a node.
///
/// With this you can focus this node whenever you want or subscribe to any focus change,
/// this way you can style your element based on its focus state.
///
/// ### Simple example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     // Create a focus instance
///     let mut my_focus = use_focus();
///
///     rsx!(
///         rect {
///             // Bind the focus to this `rect`
///             a11y_id: my_focus.attribute(),
///             // This will focus this element and effectively cause a rerender updating the returned value of `is_focused()`
///             onclick: move |_| my_focus.request_focus(),
///             label {
///                 "Am I focused? {my_focus.is_focused()}"
///             }
///         }
///     )
/// }
/// ```
///
/// ### Style based on state
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut my_focus = use_focus();
///
///     let background = if my_focus.is_focused() {
///         "red"
///     } else {
///         "blue"
///     };
///
///     rsx!(
///         rect {
///             background,
///             a11y_id: my_focus.attribute(),
///             onclick: move |_| my_focus.request_focus(),
///             label {
///                 "Focus me!"
///             }
///         }
///     )
/// }
/// ```
///
/// ### Keyboard navigation
///
/// Elements can also be selected with the keyboard, for those cases you can also subscribe by calling [UseFocus::is_focused_with_keyboard].
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut my_focus = use_focus();
///
///     let background = if my_focus.is_focused_with_keyboard() {
///         "red"
///     } else {
///         "blue"
///     };
///
///     rsx!(
///         rect {
///             background,
///             a11y_id: my_focus.attribute(),
///             label {
///                 "Focus me!"
///             }
///         }
///     )
/// }
/// ```
pub fn use_focus() -> UseFocus {
    let id = use_hook(UseFocus::new_id);

    use_focus_for_id(id)
}

/// Same as [use_focus] but providing a Node instead of generating a new one.
///
/// This is an advance hook so you probably just want to use [use_focus].
pub fn use_focus_for_id(id: AccessibilityId) -> UseFocus {
    let focused_id = use_context::<Signal<AccessibilityId>>();
    let focused_node = use_context::<Signal<AccessibilityNode>>();
    let navigation_mode = use_context::<Signal<NavigationMode>>();
    let platform = use_platform();

    let is_focused = use_memo(move || id == *focused_id.read());

    let is_focused_with_keyboard =
        use_memo(move || *is_focused.read() && *navigation_mode.read() == NavigationMode::Keyboard);

    use_hook(move || UseFocus {
        id,
        is_focused,
        is_focused_with_keyboard,
        navigation_mode,
        platform,
        focused_id,
        focused_node,
    })
}
