mod components;
mod elements;
mod theme;

pub use components::*;
pub use elements::*;
pub use theme::*;

use freya::prelude::*;

#[component]
pub fn DsGui(children: Element) -> Element {
    let theme_signal = use_init_default_theme();

    rsx!(
        rect{
            main_align: "center",
            cross_align: "center",
            height: "100%",
            width: "100%",
            rect {
                height: "80%",
                width: "100%",
                {children}
            }
            rect{
                height: "20%",
                width: "100%",
                ThemeEditor{
                    theme: theme_signal
                }
            }
        }
    )
}
