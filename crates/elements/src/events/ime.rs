use crate::{ErasedEventData, impl_event};

impl_event![
    ImeData;

    /// The `onimepreedit` event fires when the user enters a pre-edit string in an IME (Input Method Editor).
    ///
    /// Event Data: [`ImeData`](crate::events::ImeData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             onimepreedit: |e| println!("Event: {e:?}")
    ///         }
    ///     )
    /// }
    /// ```
    onimepreedit
];

#[derive(Debug, Clone, PartialEq)]
pub struct ImeData {
    pub text: String,
    pub cursor_pos: Option<(usize, usize)>,
}

impl ImeData {
    pub fn new(text: String, cursor_pos: Option<(usize, usize)>) -> Self {
        Self { text, cursor_pos }
    }
}

impl From<&ErasedEventData> for ImeData {
    fn from(val: &ErasedEventData) -> Self {
        val.downcast::<ImeData>().cloned().unwrap()
    }
}
