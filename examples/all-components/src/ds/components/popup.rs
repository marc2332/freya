use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsPopup() -> Element {
    let mut show_popup = use_signal(|| false);

    rsx!(
        if *show_popup.read() {
             Popup {
                 oncloserequest: move |_| {
                     show_popup.set(false)
                 },
                 PopupTitle {
                     label {
                         "Awesome Popup"
                     }
                 }
                 PopupContent {
                     label {
                         "Some content"
                     }
                 }
             }
         }
         Button {
             onclick: move |_| show_popup.set(true),
             label {
                 "Open"
             }
         }
    )
}
