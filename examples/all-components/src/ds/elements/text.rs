use freya::prelude::*;

static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

#[allow(non_snake_case)]
pub fn DsText() -> Element {
    let loremipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    rsx!(paragraph {
        text{
            color: "red",
            {loremipsum}
        }
        text{
            color: "green",
            {loremipsum}
        }
        text{
            color: "blue",
            {loremipsum}
        }
    })
}
