use std::path::PathBuf;

use crate::{
    events::ErasedEventData,
    impl_event,
};
impl_event! [
    FileData;

    /// The `filedrop` event fires when the user drops a file over the element.
    ///
    /// Event Data: [`FileData`](crate::events::FileData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             background: "black",
    ///             onfiledrop: |e| println!("File dropped: {e:?}")
    ///         }
    ///     )
    /// }
    /// ```
    onfiledrop

    /// The `onglobalfilehover` event fires when the user hovers a file over the window.
    ///
    /// Event Data: [`FileData`](crate::events::FileData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             background: "black",
    ///             onglobalfilehover: |e| println!("File hover: {e:?}")
    ///         }
    ///     )
    /// }
    /// ```
    onglobalfilehover

    /// The `onglobalfilehovercancelled` event fires when the user cancels the hovering of a file over the window. It's the opposite of [`onglobalfilehover`](crate::elements::onglobalfilehover()).
    ///
    /// Event Data: [`FileData`](crate::events::FileData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             background: "black",
    ///             onglobalfilehovercancelled: |e| println!("File hover cancelled: {e:?}")
    ///         }
    ///     )
    /// }
    /// ```
    onglobalfilehovercancelled
];

/// Data of a File event.
#[derive(Debug, Clone, PartialEq)]
pub struct FileData {
    pub file_path: Option<PathBuf>,
}

impl From<&ErasedEventData> for FileData {
    fn from(val: &ErasedEventData) -> Self {
        val.downcast::<FileData>().cloned().unwrap()
    }
}
