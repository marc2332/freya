use std::time::Duration;

use freya::{
    animation::*,
    code_editor::*,
    material_design::{ButtonRippleExt, Ripple},
    prelude::*,
    router::*,
    text_edit::Rope,
};

pub fn app() -> impl IntoElement {
    Router::<Route>::new(RouterConfig::default)
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppTopBar)]
        #[route("/")]
        ScrollViewDemo,
        #[route("/ripple")]
        RippleDemo,
        #[route("/portal")]
        PortalDemo,
        #[route("/editor")]
        EditorDemo,
}

#[derive(PartialEq)]
struct AppTopBar;

impl Component for AppTopBar {
    fn render(&self) -> impl IntoElement {
        NativeRouter::new().child(AnimatedRouter::<Route>::new(
            rect()
                .content(Content::flex())
                .child(
                    rect()
                        .horizontal()
                        .width(Size::fill())
                        .main_align(Alignment::center())
                        .padding((40., 8., 8., 8.))
                        .spacing(4.)
                        .child(tab(Route::ScrollViewDemo, "Scroll"))
                        .child(tab(Route::RippleDemo, "Ripple"))
                        .child(tab(Route::PortalDemo, "Portal"))
                        .child(tab(Route::EditorDemo, "Editor")),
                )
                .child(
                    rect()
                        .width(Size::fill())
                        .height(Size::flex(1.))
                        .child(AnimatedOutlet),
                ),
        ))
    }
}

fn tab(route: Route, label: &'static str) -> ActivableRoute<Route> {
    ActivableRoute::new(
        route.clone(),
        Link::new(route).child(FloatingTab::new().child(label)),
    )
    .exact(true)
}

const ROUTES: [Route; 4] = [
    Route::ScrollViewDemo,
    Route::RippleDemo,
    Route::PortalDemo,
    Route::EditorDemo,
];

fn route_index(route: &Route) -> usize {
    ROUTES.iter().position(|r| r == route).unwrap_or(0)
}

fn route_element(route: &Route) -> Element {
    match route {
        Route::ScrollViewDemo => ScrollViewDemo.into_element(),
        Route::RippleDemo => RippleDemo.into_element(),
        Route::PortalDemo => PortalDemo.into_element(),
        Route::EditorDemo => EditorDemo.into_element(),
    }
}

#[derive(Clone, PartialEq)]
struct FromRouteToCurrent {
    from: Element,
    left_to_right: bool,
    area: State<Area>,
}

impl Component for FromRouteToCurrent {
    fn render(&self) -> impl IntoElement {
        let mut animated_router = use_animated_router::<Route>();
        let animations = use_animation_with_dependencies(
            &(self.left_to_right, self.from.clone()),
            move |conf, (left_to_right, _)| {
                conf.on_change(OnChange::Rerun);
                conf.on_creation(OnCreation::Run);

                let (start, end) = if *left_to_right {
                    (1., 0.)
                } else {
                    (0., 1.)
                };
                (
                    AnimNum::new(start, end)
                        .time(500)
                        .ease(Ease::Out)
                        .function(Function::Expo),
                    AnimNum::new(1., 0.4)
                        .time(500)
                        .ease(Ease::Out)
                        .function(Function::Expo),
                    AnimNum::new(0.4, 1.)
                        .time(500)
                        .ease(Ease::Out)
                        .function(Function::Expo),
                    AnimNum::new(50., 0.)
                        .time(500)
                        .ease(Ease::Out)
                        .function(Function::Expo),
                )
            },
        );

        use_side_effect(move || {
            if !*animations.is_running().read() && *animations.has_run_yet().read() {
                animated_router.write().settle();
            }
        });

        let (offset, scale_a, scale_b, corner_radius) = animations.get().value();
        let (scale_out, scale_in) = if self.left_to_right {
            (scale_a, scale_b)
        } else {
            (scale_b, scale_a)
        };

        let width = self.area.read().width();
        let offset = width - (offset * width);

        let to = Outlet::<Route>::new().into_element();
        let (left, right) = if self.left_to_right {
            (self.from.clone(), to)
        } else {
            (to, self.from.clone())
        };

        rect()
            .expanded()
            .offset_x(-offset)
            .horizontal()
            .child(animated_page(scale_out, corner_radius, left))
            .child(animated_page(scale_in, corner_radius, right))
    }
}

fn animated_page(scale: f32, corner_radius: f32, content: impl Into<Element>) -> Rect {
    rect()
        .width(Size::percent(100.))
        .height(Size::percent(100.))
        .center()
        .background((235, 235, 235))
        .scale(scale)
        .corner_radius(corner_radius)
        .child(content)
}

#[derive(Clone, PartialEq)]
struct AnimatedOutlet;

impl Component for AnimatedOutlet {
    fn render(&self) -> impl IntoElement {
        let mut area = use_state(Area::default);
        let mut animated_router = use_animated_router();
        let involves_scroll = matches!(
            &*animated_router.read(),
            AnimatedRouterContext::FromTo(from, to)
                if *from == Route::ScrollViewDemo || *to == Route::ScrollViewDemo
        );

        let from_route = if involves_scroll {
            animated_router.write().settle();
            None
        } else {
            match &*animated_router.read() {
                AnimatedRouterContext::FromTo(from, to) => {
                    let left_to_right = route_index(to) > route_index(from);
                    Some((route_element(from), left_to_right))
                }
                _ => None,
            }
        };

        rect()
            .on_sized(move |e: Event<SizedEventData>| area.set(e.area))
            .child(match from_route {
                Some((from, left_to_right)) => FromRouteToCurrent {
                    left_to_right,
                    from,
                    area,
                }
                .into_element(),
                None => animated_page(1., 0., Outlet::<Route>::new()).into_element(),
            })
    }
}

// --- ScrollView route ---

#[derive(PartialEq)]
struct ScrollViewDemo;

impl Component for ScrollViewDemo {
    fn render(&self) -> impl IntoElement {
        VirtualScrollView::new(|i, _| {
            AnimatedContainer {
                height: 70.,
                i,
                children: rect()
                    .width(Size::fill())
                    .height(Size::fill())
                    .padding(4.)
                    .corner_radius(8.)
                    .color((255, 255, 255))
                    .background((0, 119, 182))
                    .child(format!("Item {i}"))
                    .into(),
            }
            .into()
        })
        .length(300usize)
        .item_size(70.)
        .height(Size::percent(100.))
    }
}

#[derive(PartialEq)]
struct AnimatedContainer {
    height: f32,
    i: usize,
    children: Element,
}

impl Component for AnimatedContainer {
    fn render(&self) -> impl IntoElement {
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);
            AnimNum::new(350., 0.)
                .time(500)
                .ease(Ease::InOut)
                .function(Function::Expo)
        });

        let pos = animation.get().value();

        rect()
            .offset_x(pos)
            .width(Size::fill())
            .height(Size::px(self.height))
            .padding(4.)
            .child(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.i)
    }
}

// --- Ripple route ---

#[derive(PartialEq)]
struct RippleDemo;

impl Component for RippleDemo {
    fn render(&self) -> impl IntoElement {
        rect()
            .expanded()
            .center()
            .spacing(16.)
            .child(Button::new().ripple().child("Ripple Button"))
            .child(
                Ripple::new().child(
                    rect()
                        .width(Size::px(200.))
                        .height(Size::px(100.))
                        .center()
                        .background((230, 230, 240))
                        .corner_radius(12.)
                        .color((30, 30, 30))
                        .child("Tap for ripple"),
                ),
            )
            .child(
                Ripple::new().color((255, 80, 80)).child(
                    rect()
                        .width(Size::px(200.))
                        .height(Size::px(100.))
                        .center()
                        .background((255, 240, 240))
                        .corner_radius(12.)
                        .color((30, 30, 30))
                        .child("Red ripple"),
                ),
            )
    }
}

// --- Portal route ---

fn portal_card(i: i32) -> impl IntoElement {
    rect()
        .expanded()
        .background((103, 80, 164))
        .corner_radius(16.)
        .center()
        .color(Color::WHITE)
        .child(format!("Card {}", i))
}

fn portal_popup(i: i32, mut show_popup: State<Option<i32>>) -> impl IntoElement {
    Popup::new()
        .on_close_request(move |_| show_popup.set(None))
        .width(Size::px(350.))
        .child(PopupTitle::new(format!("Card {i}")))
        .child(
            PopupContent::new().child(
                Portal::new(i)
                    .width(Size::px(250.))
                    .height(Size::px(150.))
                    .function(Function::Expo)
                    .duration(Duration::from_millis(500))
                    .child(portal_card(i)),
            ),
        )
        .child(
            PopupButtons::new()
                .child(
                    Button::new()
                        .expanded()
                        .rounded()
                        .on_press(move |_| show_popup.set(None))
                        .child("Close"),
                )
                .child(
                    Button::new()
                        .filled()
                        .expanded()
                        .rounded()
                        .on_press(move |_| show_popup.set(None))
                        .child("Accept"),
                ),
        )
}

#[derive(PartialEq)]
struct PortalDemo;

impl Component for PortalDemo {
    fn render(&self) -> impl IntoElement {
        let mut show_popup = use_state::<Option<i32>>(|| None);

        rect()
            .expanded()
            .spacing(8.)
            .padding(12.)
            .maybe_child(show_popup().map(|i| portal_popup(i, show_popup)))
            .children((0..3).map(|i| {
                rect()
                    .key(i)
                    .spacing(6.)
                    .width(Size::fill())
                    .child(
                        Portal::new(i)
                            .key(show_popup())
                            .show(show_popup() != Some(i))
                            .width(Size::fill())
                            .height(Size::px(120.))
                            .function(Function::Expo)
                            .duration(Duration::from_millis(500))
                            .child(portal_card(i)),
                    )
                    .child(
                        Button::new()
                            .child("Open")
                            .rounded()
                            .on_press(move |_| show_popup.set(Some(i))),
                    )
                    .into()
            }))
    }
}

// --- Editor route ---

const SAMPLE_CODE: &str = r#"use freya::prelude::*;

/// A simple counter app built with Freya.
fn app() -> impl IntoElement {
    let mut count = use_state(|| 0);
    let is_positive = *count.read() >= 0;

    rect()
        .expanded()
        .center()
        .spacing(12.)
        .child(
            rect()
                .width(Size::px(250.))
                .height(Size::px(120.))
                .center()
                .background(if is_positive {
                    (15, 163, 242)
                } else {
                    (220, 50, 50)
                })
                .corner_radius(16.)
                .color(Color::WHITE)
                .font_size(56.)
                .font_weight(FontWeight::BOLD)
                .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
                .child(count.read().to_string()),
        )
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .child(
                    Button::new()
                        .filled()
                        .on_press(move |_| {
                            *count.write() -= 1;
                        })
                        .child("Decrease"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| {
                            count.set(0);
                        })
                        .child("Reset"),
                )
                .child(
                    Button::new()
                        .filled()
                        .on_press(move |_| {
                            *count.write() += 1;
                        })
                        .child("Increase"),
                ),
        )
}

#[derive(PartialEq)]
struct TodoItem {
    label: String,
    done: bool,
}

/// A minimal todo list component.
#[derive(PartialEq)]
struct TodoList;

impl Component for TodoList {
    fn render(&self) -> impl IntoElement {
        let mut items = use_state::<Vec<TodoItem>>(Vec::new);
        let mut input = use_state(String::new);

        let on_submit = move |_| {
            let text = input.read().trim().to_string();
            if !text.is_empty() {
                items.write().push(TodoItem {
                    label: text,
                    done: false,
                });
                input.set(String::new());
            }
        };

        rect()
            .width(Size::px(400.))
            .spacing(8.)
            .padding(16.)
            .child(
                rect()
                    .horizontal()
                    .spacing(8.)
                    .child(
                        Input::new()
                            .value(input.read().clone())
                            .on_change(move |txt| input.set(txt))
                            .placeholder("Add a task..."),
                    )
                    .child(
                        Button::new()
                            .filled()
                            .on_press(on_submit)
                            .child("Add"),
                    ),
            )
            .children(
                items
                    .read()
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        let label = if item.done {
                            format!("[x] {}", item.label)
                        } else {
                            format!("[ ] {}", item.label)
                        };

                        Button::new()
                            .flat()
                            .on_press(move |_| {
                                items.write()[idx].done =
                                    !items.read()[idx].done;
                            })
                            .child(label)
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
    }
}"#;

#[derive(PartialEq)]
struct EditorDemo;

impl Component for EditorDemo {
    fn render(&self) -> impl IntoElement {
        let focus = use_focus();

        let editor = use_state(move || {
            let rope = Rope::from_str(SAMPLE_CODE);
            let mut editor = CodeEditorData::new(rope, LanguageId::Rust);
            editor.set_theme(SyntaxTheme::default());
            editor.parse();
            editor.measure(14., "Jetbrains Mono");
            editor
        });

        rect()
            .expanded()
            .padding(8.)
            .child(CodeEditor::new(editor, focus.a11y_id()).line_height(1.3))
    }
}
