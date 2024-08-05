use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn PageNotFound() -> Element {
    rsx!(
        label {
            "Oh, no! Freya doesn't have such component..."
        }
        Link{
            to: "https://github.com/marc2332/freya/",
            label {
                "Contribute"
            }
        }
    )
}
