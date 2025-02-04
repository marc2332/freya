pub use keyboard_types::{
    Code,
    Key,
    Modifiers,
};

use crate::{
    events::ErasedEventData,
    impl_event,
};

impl_event! [
    KeyboardData;

    /// The `keydown` event fires when the user starts pressing any key in the currently focused element.
    ///
    /// Event Data: [`KeyboardData`](crate::events::KeyboardData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             onkeydown: |e| println!("Event: {e:?}")
    ///         }
    ///     )
    /// }
    /// ```
    onkeydown

    /// The `keyup` event fires when the user releases any key being pressed in the currently focused element.
    ///
    /// Event Data: [`KeyboardData`](crate::events::KeyboardData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             onkeyup: |e| println!("Event: {e:?}")
    ///         }
    ///     )
    /// }
    /// ```
    onkeyup

    /// The `globalkeydown` event fires when the user starts pressing any key.
    ///
    /// Event Data: [`KeyboardData`](crate::events::KeyboardData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             onglobalkeydown: |e| println!("Event: {e:?}")
    ///         }
    ///     )
    /// }
    /// ```
    onglobalkeydown

    /// The `globalkeyup` event fires when the user releases any key being pressed.
    ///
    /// Event Data: [`KeyboardData`](crate::events::KeyboardData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             onglobalkeyup: |e| println!("Event: {e:?}")
    ///         }
    ///     )
    /// }
    /// ```
    onglobalkeyup
];

/// Data of a Keyboard event.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardData {
    pub key: Key,
    pub code: Code,
    pub modifiers: Modifiers,
}

impl KeyboardData {
    pub fn new(key: Key, code: Code, modifiers: Modifiers) -> Self {
        Self {
            key,
            code,
            modifiers,
        }
    }
}

impl KeyboardData {
    /// Try to get the text of the character
    pub fn to_text(&self) -> Option<&str> {
        if let Key::Character(c) = &self.key {
            Some(c)
        } else {
            None
        }
    }
}

impl From<&ErasedEventData> for KeyboardData {
    fn from(val: &ErasedEventData) -> Self {
        val.downcast::<KeyboardData>().cloned().unwrap()
    }
}
