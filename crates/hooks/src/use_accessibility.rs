use dioxus_core::{use_hook, AttributeValue};
use dioxus_hooks::{use_context, use_memo, use_signal};
use dioxus_signals::{Memo, Readable, Signal, Writable};
use freya_core::{
    accessibility::ACCESSIBILITY_ROOT_ID, platform_state::NavigationMode, prelude::EventMessage,
    types::AccessibilityId,
};
use freya_engine::prelude::Color;
use freya_node_state::{
    AccessibilityOptions, AutoComplete, CellIndex, Current, CustomAction, CustomAttributeValues,
    DefaultActionVerb, HasPopup, Invalid, ListStyle, Live, NumericValue, Orientation, Role,
    ScrollValue, SortDirection, TextSelection, Toggled, VerticalOffset,
};

use crate::{use_platform, AccessibilityIdCounter, NavigationMark, UsePlatform};

/// Manage the focus operations of given Node
#[derive(Clone, Copy)]
pub struct UseAccessibility {
    id: AccessibilityId,
    is_focused: Memo<bool>,
    options: Signal<AccessibilityOptions>,
    navigation_mode: Signal<NavigationMode>,
    navigation_mark: Signal<NavigationMark>,
    platform: UsePlatform,
}

macro_rules! impl_accessibility_methods {
    (
        $(
            $(#[$meta:meta])*
            ($name:ident, $setter_name:ident): $type:ty
        ),+ $(,)?
    ) => {
        impl UseAccessibility {
            $(
                $(#[$meta])*
                pub fn $setter_name(&mut self, $name: $type) {
                    self.options.write().$name = Some($name);
                }
            )+
        }

        impl UseAccessibilityBuilder {
            $(
                $(#[$meta])*
                pub fn $name(&mut self, $name: $type) -> &mut Self {
                    self.options.$name = Some($name);
                    self
                }
            )+
        }
    };
}

impl UseAccessibility {
    /// Focus this node
    pub fn focus(&mut self) {
        if !*self.is_focused.peek() {
            self.platform
                .send(EventMessage::FocusAccessibilityNode(self.id))
                .ok();
        }
    }

    /// Queue a focus to this node
    pub fn queue_focus(&mut self) {
        if !*self.is_focused.peek() {
            self.platform
                .send(EventMessage::QueueFocusAccessibilityNode(self.id))
                .ok();
        }
    }

    /// Get the node focus ID
    pub fn id(&self) -> AccessibilityId {
        self.id
    }

    /// Create a node focus ID attribute
    pub fn attribute(&self) -> AttributeValue {
        AttributeValue::any_value(CustomAttributeValues::Accessibility(
            self.id,
            self.options.read().clone(),
        ))
    }

    /// Check if this node is currently focused
    pub fn is_focused(&self) -> bool {
        *self.is_focused.read()
    }

    pub fn is_focusable(&self) -> bool {
        self.options.read().is_focusable
    }

    /// Check if this node is currently selected
    pub fn is_keyboard_focused(&self) -> bool {
        self.is_focused() && *self.navigation_mode.read() == NavigationMode::Keyboard
    }

    /// Unfocus the currently focused node.
    pub fn unfocus(&mut self) {
        self.platform
            .send(EventMessage::FocusAccessibilityNode(ACCESSIBILITY_ROOT_ID))
            .ok();
    }

    /// Prevent navigating the accessible nodes with the keyboard.
    /// You must use this this inside of a `onkeydown` event handler.
    pub fn prevent_navigation(&mut self) {
        self.navigation_mark.write().set_allowed(false);
    }

    pub fn set_role(&mut self, role: Role) {
        self.options.write().role = role;
    }

    pub fn set_is_focusable(&mut self, is_focusable: bool) {
        self.options.write().is_focusable = is_focusable;
    }
}

pub struct UseAccessibilityBuilder {
    options: AccessibilityOptions,
}

impl UseAccessibilityBuilder {
    pub(crate) fn new(role: Role) -> Self {
        let mut options = AccessibilityOptions::default();
        options.role = role;
        Self { options }
    }

    pub fn build(self) -> UseAccessibility {
        let accessibility_id_counter = use_context::<AccessibilityIdCounter>();
        let focused_id = use_context::<Signal<AccessibilityId>>();
        let navigation_mode = use_context::<Signal<NavigationMode>>();
        let navigation_mark = use_context::<Signal<NavigationMark>>();
        let platform = use_platform();

        let id = use_hook(|| {
            let mut counter = accessibility_id_counter.borrow_mut();
            *counter += 1;
            AccessibilityId(*counter)
        });

        let is_focused = use_memo(move || id == *focused_id.read());

        use_hook(move || UseAccessibility {
            id,
            options: use_signal(move || self.options),
            is_focused,
            navigation_mode,
            navigation_mark,
            platform,
        })
    }
}

impl_accessibility_methods! {
    (details, set_details): Vec<AccessibilityId>,
    (controls, set_controls): Vec<AccessibilityId>,
    (described_by, set_described_by): Vec<AccessibilityId>,
    (flow_to, set_flow_to): Vec<AccessibilityId>,
    (named_by, set_named_by): Vec<AccessibilityId>,
    (owns, set_owns): Vec<AccessibilityId>,
    (radio_group, set_radio_group): Vec<AccessibilityId>,
    (active_descendant, set_active_descendant): AccessibilityId,
    (in_page_link_target, set_in_page_link_target): AccessibilityId,
    (member_of, set_member_of): AccessibilityId,
    (next_on_line, set_next_on_line): AccessibilityId,
    (previous_on_line, set_previous_on_line): AccessibilityId,
    (popup_for, set_popup_for): AccessibilityId,
    (name, set_name): String,
    (description, set_description): String,
    (value, set_value): String,
    (author_id, set_author_id): String,
    (keyboard_shortcuts, set_keyboard_shortcuts): String,
    (language, set_language): String,
    (placeholder, set_placeholder): String,
    (role_description, set_role_description): String,
    (state_description, set_state_description): String,
    (tooltip, set_tooltip): String,
    (url, set_url): String,
    (access_key, set_access_key): char,
    (row_count, set_row_count): usize,
    (column_count, set_column_count): usize,
    (row_span, set_row_span): usize,
    (column_span, set_column_span): usize,
    (level, set_level): usize,
    (size_of_set, set_size_of_set): usize,
    (position_in_set, set_position_in_set): usize,
    (is_disabled, set_is_disabled): bool,
    (is_expanded, set_is_expanded): bool,
    (is_selected, set_is_selected): bool,
    (is_hidden, set_is_hidden): bool,
    (is_linked, set_is_linked): bool,
    (is_multiselectable, set_is_multiselectable): bool,
    (is_required, set_is_required): bool,
    (is_visited, set_is_visited): bool,
    (is_busy, set_is_busy): bool,
    (is_live_atomic, set_is_live_atomic): bool,
    (is_modal, set_is_modal): bool,
    (is_touch_transparent, set_is_touch_transparent): bool,
    (is_read_only, set_is_read_only): bool,
    (is_grammar_error, set_is_grammar_error): bool,
    (is_spelling_error, set_is_spelling_error): bool,
    (is_search_match, set_is_search_match): bool,
    (is_suggestion, set_is_suggestion): bool,
    (color_value, set_color_value): Color,
    (vertical_offset, set_vertical_offset): VerticalOffset,
    (numeric_value, set_numeric_value): NumericValue,
    (scroll_x, set_scroll_x): ScrollValue,
    (scroll_y, set_scroll_y): ScrollValue,
    (row_index, set_row_index): CellIndex,
    (column_index, set_column_index): CellIndex,
    (has_popup, set_has_popup): HasPopup,
    (list_style, set_list_style): ListStyle,
    (sort_direction, set_sort_direction): SortDirection,
    (auto_complete, set_auto_complete): AutoComplete,
    (orientation, set_orientation): Orientation,
    (current, set_current): Current,
    (default_action_verb, set_default_action_verb): DefaultActionVerb,
    (toggled, set_toggled): Toggled,
    (live, set_live): Live,
    (invalid, set_invalid): Invalid,
    (custom_actions, set_custom_actions): Vec<CustomAction>,
    (text_selection, set_text_selection): TextSelection,
}

/// Create a focus manager for a node.
pub fn use_accessibility(role: Role) -> UseAccessibilityBuilder {
    UseAccessibilityBuilder::new(role)
}
