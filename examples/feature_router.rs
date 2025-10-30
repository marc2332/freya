use freya::prelude::*;
use freya_router::prelude::{
    Routable,
    RouterConfig,
    RouterContext,
    outlet,
    router,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    router::<Route>(|| RouterConfig::default().with_initial_path(Route::Settings))
}

#[derive(PartialEq)]
struct Layout;
impl Render for Layout {
    fn render(&self) -> Element {
        rect().center().expanded().child(outlet::<Route>()).into()
    }
}

#[derive(PartialEq)]
struct Home {}
impl Render for Home {
    fn render(&self) -> Element {
        Button::new()
            .on_press(|_| {
                RouterContext::get().replace(Route::Settings);
            })
            .child("Go Settings")
            .into()
    }
}

#[derive(PartialEq)]
struct Settings {}
impl Render for Settings {
    fn render(&self) -> Element {
        Button::new()
            .on_press(|_| {
                RouterContext::get().replace(Route::Home);
            })
            .child("Go Home")
            .into()
    }
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home,
        #[route("/settings")]
        Settings,
}
