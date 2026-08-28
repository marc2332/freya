use freya::{
    animation::*,
    i18n::*,
    icons::lucide,
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
        #[route("/", ComponentsShowcase)]
        Components,
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
        let route = use_route::<Route>();
        let theme = use_init_theme(light_theme);
        let surface = theme.read().colors.surface_tertiary;
        let icon_color = theme.read().colors.text_primary;
        let mut window_width = use_state(|| f32::MAX);
        let mut sidebar_open = use_state(|| false);
        let mut slide = use_animation(|_| {
            AnimNum::new(-280., 0.)
                .time(250)
                .ease(Ease::Out)
                .function(Function::Expo)
        });

        let drawer_left = slide.get().value();
        let progress = 1. + drawer_left / 280.;

        let compact = *window_width.read() < 800.;
        let sliding_sidebar = compact && (*sidebar_open.read() || *slide.is_running().read());

        let mut close_sidebar = move || {
            if *sidebar_open.peek() {
                slide.reverse();
                sidebar_open.set(false);
            }
        };

        use_side_effect_with_deps(&route, move |_| close_sidebar());

        let open_sidebar = move |_| {
            slide.start();
            sidebar_open.set(true);
        };

        rect()
            .native_router()
            .expanded()
            .center()
            .theme_color()
            .theme_background()
            .on_sized(move |event: Event<SizedEventData>| {
                window_width.set_if_modified(event.area.width());
                if event.area.width() >= 800. && *sidebar_open.peek() {
                    sidebar_open.set(false);
                    slide.reset();
                }
            })
            .child(
                rect()
                    .width(Size::percent(85.))
                    .height(Size::percent(85.))
                    .horizontal()
                    .background(surface)
                    .corner_radius(20.)
                    .shadow((0., 10., 30., 0., (0, 0, 0, 40)))
                    .overflow(Overflow::Clip)
                    .maybe_child((!compact).then(|| sidebar().key("sidebar")))
                    .child(
                        rect()
                            .key("content")
                            .expanded()
                            .height(Size::fill())
                            .padding(24.)
                            .maybe(compact, |el| {
                                el.spacing(16.).child(
                                    Button::new().key("burger").on_press(open_sidebar).child(
                                        SvgViewer::new(lucide::menu())
                                            .stroke(icon_color)
                                            .width(Size::px(20.))
                                            .height(Size::px(20.)),
                                    ),
                                )
                            })
                            .child(
                                rect()
                                    .key("page")
                                    .width(Size::fill())
                                    .expanded()
                                    .child(Outlet::<Route>::new()),
                            ),
                    )
                    .maybe(sliding_sidebar, |el| {
                        el.child(
                            rect()
                                .key("backdrop")
                                .position(Position::new_absolute().left(0.).top(0.))
                                .width(Size::percent(100.))
                                .height(Size::percent(100.))
                                .background((0, 0, 0, (progress * 100.) as u8))
                                .layer(Layer::Relative(90))
                                .on_press(move |_| close_sidebar()),
                        )
                        .child(
                            sidebar()
                                .key("floating-sidebar")
                                .position(Position::new_absolute().left(drawer_left).top(0.))
                                .layer(Layer::Relative(100))
                                .opacity(0.6 + 0.4 * progress)
                                .shadow((10., 0., 30., 0., (0, 0, 0, 40))),
                        )
                    }),
            )
    }
}

fn sidebar() -> Rect {
    rect()
        .width(Size::px(240.))
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
        .child(item(Route::Components, "Components", true))
        .child(item(Route::Animation, "Animation", false))
        .child(item(Route::Graphics, "Graphics", false))
        .child(item(Route::Material, "Material Design", false))
        .child(item(Route::Markdown, "Markdown", false))
        .child(item(Route::Scroll, "Virtual Scroll", false))
        .child(item(Route::Kanban, "Kanban", false))
        .child(item(Route::I18n, "i18n", false))
}

/// Sidebar entry that highlights itself while its route is active.
fn item(route: Route, title: &'static str, exact: bool) -> ActivableRoute<Route> {
    ActivableRoute::new(
        route.clone(),
        Link::new(route).child(SideBarItem::new().child(title)),
    )
    .exact(exact)
}
