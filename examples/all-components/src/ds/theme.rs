use dioxus_router::{components::Outlet, hooks::use_route};
use freya::prelude::*;

use crate::{ButtonThemeEditor, Route};

#[component]
pub fn ThemeEditor(theme: Signal<Theme>) -> Element {
    let route = use_route::<Route>();

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(230, 230, 230)",
            ScrollView {
                rect{
                    padding: "20",
                    {
                        match route {
                            Route::DsButton => {
                                rsx!(ButtonThemeEditor {theme: theme})
                            },
                            _ => rsx!(
                                label{
                                    "No theme editor for this component"
                                }
                            )
                        }
                    }
                }
            }
        }
    )
}
