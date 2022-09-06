use components::*;
use dioxus::prelude::*;
use elements_namespace as dioxus_elements;
use fermi::*;

use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let theme = use_atom_ref(&cx, THEME);

    cx.render(rsx!(view {
        height: "100%",
        width: "100%",
        Button {
            on_click: |_| {
                *theme.write() = LIGHT_THEME;
            },
            text {
                width: "100",
                "Light"
            }
        }
        Button {
            on_click: |_| {
                *theme.write() = DARK_THEME;
            },
            text {
                width: "100",
                "Dark"
            }
        }
    }))
}
