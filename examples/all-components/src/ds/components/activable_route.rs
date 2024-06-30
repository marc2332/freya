use freya::prelude::*;

use crate::Route;

#[allow(non_snake_case)]
pub fn DsActivableRoute() -> Element {
    rsx!(
        ActivableRoute {
            route: Route::Home,
            label {
                "ActivableRoute 1"
            }
        }
    )
}
