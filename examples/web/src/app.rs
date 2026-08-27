use freya::{
    i18n::*,
    prelude::*,
    router::*,
};

use crate::showcases::*;

pub fn app() -> impl IntoElement {
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("./i18n/en-US.ftl")))
            .with_locale((langid!("es-ES"), include_str!("./i18n/es-ES.ftl")))
            .with_fallback(langid!("en-US"))
    });

    Router::<Route>::new(RouterConfig::default)
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppShell)]
        #[route("/", WidgetsShowcase)]
        Widgets,
        #[route("/animation", AnimationShowcase)]
        Animation,
        #[route("/graphics", GraphicsShowcase)]
        Graphics,
        #[route("/material", MaterialShowcase)]
        Material,
        #[route("/markdown", MarkdownShowcase)]
        Markdown,
        #[route("/scroll", ScrollShowcase)]
        Scroll,
        #[route("/kanban", KanbanShowcase)]
        Kanban,
        #[route("/i18n", I18nShowcase)]
        I18n,
}

#[derive(PartialEq)]
struct AppShell;

impl Component for AppShell {
    fn render(&self) -> impl IntoElement {
        let theme = use_init_theme(light_theme);
        let surface = theme.read().colors.surface_tertiary;

        rect()
            .native_router()
            .expanded()
            .center()
            .theme_color()
            .theme_background()
            .child(
                rect()
                    .width(Size::percent(85.))
                    .height(Size::percent(85.))
                    .horizontal()
                    .background(surface)
                    .corner_radius(20.)
                    .shadow((0., 10., 30., 0., (0, 0, 0, 40)))
                    .overflow(Overflow::Clip)
                    .child(
                        rect()
                            .width(Size::px(210.))
                            .height(Size::fill())
                            .theme_background()
                            .padding(12.)
                            .spacing(4.)
                            .child(
                                rect()
                                    .padding((8., 8., 16., 8.))
                                    .font_size(20.)
                                    .font_weight(FontWeight::BOLD)
                                    .child("Freya on the web"),
                            )
                            .child(item(Route::Widgets, "Components", true))
                            .child(item(Route::Animation, "Animation", false))
                            .child(item(Route::Graphics, "Graphics", false))
                            .child(item(Route::Material, "Material Design", false))
                            .child(item(Route::Markdown, "Markdown", false))
                            .child(item(Route::Scroll, "Virtual Scroll", false))
                            .child(item(Route::Kanban, "Kanban", false))
                            .child(item(Route::I18n, "i18n", false)),
                    )
                    .child(
                        rect()
                            .expanded()
                            .height(Size::fill())
                            .padding(24.)
                            .child(Outlet::<Route>::new()),
                    ),
            )
    }
}

/// Sidebar entry that highlights itself while its route is active.
fn item(route: Route, title: &'static str, exact: bool) -> ActivableRoute<Route> {
    ActivableRoute::new(
        route.clone(),
        Link::new(route).child(SideBarItem::new().child(title)),
    )
    .exact(exact)
}
