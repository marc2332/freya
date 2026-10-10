use std::rc::Rc;

use freya::prelude::*;
use freya_core::{
    element::AppComponent,
    node_id::NodeId,
};
use freya_testing::prelude::*;
use torin::prelude::Size2D;

struct EqualityTest {
    incremental: TestingRunner,
    full: TestingRunner,
}

impl EqualityTest {
    fn new(app: impl Into<AppComponent> + Copy) -> Self {
        let incremental = launch_test(app);
        let mut full = launch_test(app);
        full.set_force_full_render(true);
        Self { incremental, full }
    }

    fn step(&mut self, action: impl Fn(&mut TestingRunner)) {
        action(&mut self.incremental);
        action(&mut self.full);
        let incremental_pixels = self.incremental.render_pixels();
        let full_pixels = self.full.render_pixels();
        let differing = incremental_pixels
            .chunks(4)
            .zip(full_pixels.chunks(4))
            .filter(|(incremental, full)| incremental != full)
            .count();
        assert!(
            differing == 0,
            "Incremental and full renders diverged in {differing} pixels"
        );
    }
}

#[test]
fn unchanged_paragraphs_reuse_measurement_data() {
    let (mut test, mut revision) = TestingRunner::new(
        || {
            let revision = use_consume::<State<usize>>();
            ScrollView::new()
                .child(
                    paragraph()
                        .width(Size::fill())
                        .span(format!("Revision {}", revision())),
                )
                .child(paragraph().width(Size::fill()).span("Unchanged one"))
                .child(paragraph().width(Size::fill()).span("Unchanged two"))
        },
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(0usize)),
        1.,
    );
    let initial_pixels = test.render_pixels();
    let paragraphs = {
        let tree = test.tree().borrow();
        tree.elements
            .iter()
            .filter(|(_, element)| Paragraph::try_downcast(element.as_ref()).is_some())
            .map(|(node_id, _)| {
                (
                    *node_id,
                    tree.layout.get(node_id).unwrap().data.clone().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(paragraphs.len(), 3);

    test.tree().borrow_mut().layout.invalidate(NodeId::ROOT);
    test.sync_and_update();
    {
        let tree = test.tree().borrow();
        let render_state = &tree.render_state;
        for (node_id, previous) in &paragraphs {
            let current = tree.layout.get(node_id).unwrap().data.as_ref().unwrap();
            assert!(Rc::ptr_eq(previous, current));
            assert!(!render_state.dirty.contains(node_id));
        }
    }
    assert_eq!(initial_pixels, test.render_pixels());

    revision.set(1);
    test.sync_and_update();
    {
        let tree = test.tree().borrow();
        let render_state = &tree.render_state;
        for (node_id, previous) in &paragraphs {
            let paragraph =
                Paragraph::try_downcast(&**tree.elements.get(node_id).unwrap()).unwrap();
            let changed = paragraph.spans[0].text.starts_with("Revision ");
            let current = tree.layout.get(node_id).unwrap().data.as_ref().unwrap();
            assert_eq!(!Rc::ptr_eq(previous, current), changed);
            assert_eq!(render_state.dirty.contains(node_id), changed);
        }
    }
    assert_ne!(initial_pixels, test.render_pixels());
}

#[test]
fn equality_inline_paragraph_after_child_resize() {
    fn app() -> impl IntoElement {
        let mut expanded = use_state(|| false);
        paragraph()
            .width(Size::px(400.))
            .height(Size::px(100.))
            .on_press(move |_| expanded.toggle())
            .span("Some text before ")
            .child(
                rect()
                    .width(Size::px(if expanded() { 80. } else { 40. }))
                    .height(Size::px(20.))
                    .background(Color::BLUE),
            )
            .span(" and some text after.")
    }

    let mut test = EqualityTest::new(app);
    test.step(|_| {});
    test.step(|test| test.click_cursor((100., 20.)));
    test.step(|test| test.click_cursor((100., 20.)));
}

#[derive(PartialEq)]
struct RowsApp;

impl Component for RowsApp {
    fn render(&self) -> impl IntoElement {
        let mut selected = use_state(|| false);

        rect()
            .expanded()
            .on_mouse_up(move |_| selected.toggle())
            .child(ScrollView::new().children((0..500).map(|i| {
                let is_selected = i == 2 && selected();
                rect()
                    .width(Size::fill())
                    .height(Size::px(20.))
                    .horizontal()
                    .background(if is_selected {
                        (200, 60, 60)
                    } else if i % 2 == 0 {
                        (245, 245, 245)
                    } else {
                        (225, 225, 225)
                    })
                    .child(label().color((20, 20, 20)).text(if is_selected {
                        format!("Row {i} (selected)")
                    } else {
                        format!("Row {i}")
                    }))
            })))
    }
}

#[test]
pub fn equality_row_toggle_in_long_list() {
    let mut test = EqualityTest::new(|| RowsApp);
    test.step(|_| {});
    test.step(|test| test.click_cursor((100., 50.)));
    test.step(|test| test.click_cursor((100., 50.)));
}

#[derive(PartialEq)]
struct KeyedApp;

impl Component for KeyedApp {
    fn render(&self) -> impl IntoElement {
        let mut count = use_state(|| 3usize);

        rect()
            .expanded()
            .spacing(5.)
            .padding(20.)
            .on_mouse_up(move |_| {
                let next = (*count.peek() + 1) % 6;
                count.set(next);
            })
            .children((0..count()).map(|i| {
                rect()
                    .key(i)
                    .width(Size::px(100. + i as f32 * 20.))
                    .height(Size::px(30.))
                    .background((50 + i as u8 * 30, 100, 200 - i as u8 * 30))
            }))
    }
}

#[test]
pub fn equality_keyed_add_remove() {
    let mut test = EqualityTest::new(|| KeyedApp);
    for _ in 0..6 {
        test.step(|test| test.click_cursor((10., 10.)));
    }
}

#[derive(PartialEq)]
struct ScrollApp;

impl Component for ScrollApp {
    fn render(&self) -> impl IntoElement {
        ScrollView::new().children((0..40).map(|i| {
            rect()
                .width(Size::px(300.))
                .height(Size::px(40.))
                .background(if i % 2 == 0 {
                    (230, 240, 250)
                } else {
                    (40, 50, 60)
                })
                .child(label().color((120, 120, 120)).text(format!("Item {i}")))
        }))
    }
}

#[test]
pub fn equality_scroll_wheel_and_scrollbar_drag() {
    let mut test = EqualityTest::new(|| ScrollApp);
    test.step(|test| test.scroll((250., 250.), (0., -120.)));
    test.step(|test| test.scroll((250., 250.), (0., -1000.)));
    test.step(|test| test.scroll((250., 250.), (0., 1000.)));
    test.step(|test| test.scroll((250., 250.), (0., -200.)));
    test.step(|test| test.press_cursor((495., 200.)));
    test.step(|test| test.move_cursor((495., 300.)));
    test.step(|test| test.release_cursor((495., 300.)));
}

#[derive(PartialEq)]
struct VirtualScrollApp;

impl Component for VirtualScrollApp {
    fn render(&self) -> impl IntoElement {
        VirtualScrollView::new(|item, _| {
            rect()
                .width(Size::fill())
                .height(Size::px(25.))
                .background(if item.index % 2 == 0 {
                    (230, 240, 250)
                } else {
                    (40, 50, 60)
                })
                .child(
                    label()
                        .color((120, 120, 120))
                        .text(format!("Line {}", item.index)),
                )
                .into()
        })
        .length(500usize)
        .item_size(25.)
    }
}

#[test]
pub fn equality_virtual_scroll() {
    let mut test = EqualityTest::new(|| VirtualScrollApp);
    test.step(|test| test.scroll((250., 250.), (0., -10.)));
    test.step(|test| test.scroll((250., 250.), (0., -30.)));
    test.step(|test| test.scroll((250., 250.), (0., -500.)));
    test.step(|test| test.scroll((250., 250.), (0., 250.)));
}

#[derive(PartialEq)]
struct OverlayApp;

impl Component for OverlayApp {
    fn render(&self) -> impl IntoElement {
        let mut shown = use_state(|| false);

        rect()
            .expanded()
            .padding(20.)
            .on_mouse_up(move |_| shown.toggle())
            .child(
                rect()
                    .width(Size::px(400.))
                    .height(Size::px(400.))
                    .background((240, 240, 220))
                    .child(label().color((20, 20, 20)).text("Content behind")),
            )
            .maybe_child(shown().then(|| {
                rect()
                    .layer(Layer::Overlay)
                    .position(Position::new_global().top(150.).left(150.))
                    .width(Size::px(200.))
                    .height(Size::px(200.))
                    .corner_radius(12.)
                    .background((60, 60, 200))
                    .shadow(Shadow::new().y(6.).blur(24.).color((0, 0, 0, 120)))
            }))
    }
}

#[test]
pub fn equality_overlay_open_close() {
    let mut test = EqualityTest::new(|| OverlayApp);
    test.step(|test| test.click_cursor((10., 10.)));
    test.step(|test| test.click_cursor((10., 10.)));
    test.step(|test| test.click_cursor((10., 10.)));
}

#[derive(PartialEq)]
struct AnimatedShadowApp;

impl Component for AnimatedShadowApp {
    fn render(&self) -> impl IntoElement {
        let mut step = use_state(|| 0u32);

        let blur = [5., 20., 45., 20., 5.][(step() as usize) % 5];

        rect()
            .expanded()
            .padding(100.)
            .on_mouse_up(move |_| *step.write() += 1)
            .child(
                rect()
                    .width(Size::px(120.))
                    .height(Size::px(120.))
                    .background((80, 160, 80))
                    .shadow(Shadow::new().x(8.).y(8.).blur(blur).color((0, 0, 0))),
            )
    }
}

#[test]
pub fn equality_animated_shadow() {
    let mut test = EqualityTest::new(|| AnimatedShadowApp);
    for _ in 0..5 {
        test.step(|test| test.click_cursor((20., 20.)));
    }
}

#[derive(PartialEq)]
struct BackdropBlurApp;

impl Component for BackdropBlurApp {
    fn render(&self) -> impl IntoElement {
        let mut toggled = use_state(|| false);

        rect()
            .expanded()
            .on_mouse_up(move |_| toggled.toggle())
            .child(
                rect()
                    .position(Position::new_global().top(50.).left(50.))
                    .width(Size::px(200.))
                    .height(Size::px(200.))
                    .background(if toggled() {
                        (250, 100, 100)
                    } else {
                        (100, 100, 250)
                    }),
            )
            .child(
                rect()
                    .layer(Layer::Overlay)
                    .position(Position::new_global().top(100.).left(100.))
                    .width(Size::px(250.))
                    .height(Size::px(250.))
                    .backdrop_blur(12.)
                    .background((255, 255, 255, 60)),
            )
    }
}

#[test]
pub fn equality_backdrop_blur_over_changing_content() {
    let mut test = EqualityTest::new(|| BackdropBlurApp);
    test.step(|test| test.click_cursor((20., 20.)));
    test.step(|test| test.click_cursor((20., 20.)));
}

#[derive(PartialEq)]
struct TransformApp;

impl Component for TransformApp {
    fn render(&self) -> impl IntoElement {
        let mut transformed = use_state(|| false);

        rect()
            .expanded()
            .padding(150.)
            .on_mouse_up(move |_| transformed.toggle())
            .child(
                rect()
                    .width(Size::px(120.))
                    .height(Size::px(120.))
                    .rotate(if transformed() { 30. } else { 0. })
                    .scale(if transformed() { 1.4 } else { 1. })
                    .background((200, 120, 40))
                    .child(label().color((0, 0, 0)).text("Spin")),
            )
    }
}

#[test]
pub fn equality_rotation_and_scale_change() {
    let mut test = EqualityTest::new(|| TransformApp);
    test.step(|test| test.click_cursor((20., 20.)));
    test.step(|test| test.click_cursor((20., 20.)));
}

#[derive(PartialEq)]
struct OpacityApp;

impl Component for OpacityApp {
    fn render(&self) -> impl IntoElement {
        let mut faded = use_state(|| false);

        rect()
            .expanded()
            .padding(50.)
            .on_mouse_up(move |_| faded.toggle())
            .child(
                rect()
                    .opacity(if faded() { 0.35 } else { 1. })
                    .spacing(10.)
                    .child(
                        rect()
                            .width(Size::px(150.))
                            .height(Size::px(80.))
                            .background((200, 60, 60)),
                    )
                    .child(
                        rect()
                            .width(Size::px(150.))
                            .height(Size::px(80.))
                            .background((60, 60, 200))
                            .child(label().color((255, 255, 255)).text("Faded subtree")),
                    ),
            )
    }
}

#[test]
pub fn equality_opacity_change_on_subtree() {
    let mut test = EqualityTest::new(|| OpacityApp);
    test.step(|test| test.click_cursor((20., 20.)));
    test.step(|test| test.click_cursor((20., 20.)));
}

#[derive(PartialEq)]
struct OverlapApp;

impl Component for OverlapApp {
    fn render(&self) -> impl IntoElement {
        let mut toggled = use_state(|| false);

        rect()
            .expanded()
            .on_mouse_up(move |_| toggled.toggle())
            .child(
                rect()
                    .position(Position::new_global().top(100.).left(100.))
                    .width(Size::px(150.))
                    .height(Size::px(150.))
                    .background((60, 180, 60)),
            )
            .child(
                rect()
                    .position(Position::new_global().top(150.).left(150.))
                    .width(Size::px(150.))
                    .height(Size::px(150.))
                    .background(if toggled() {
                        (180, 60, 180)
                    } else {
                        (60, 60, 180)
                    }),
            )
    }
}

#[test]
pub fn equality_overlapping_siblings_one_changes() {
    let mut test = EqualityTest::new(|| OverlapApp);
    test.step(|test| test.click_cursor((20., 20.)));
    test.step(|test| test.click_cursor((20., 20.)));
}

#[test]
pub fn equality_window_resize_mid_scenario() {
    let mut test = EqualityTest::new(|| OverlayApp);
    test.step(|test| test.click_cursor((10., 10.)));
    test.step(|test| test.resize(Size2D::new(400., 350.)));
    test.step(|test| test.click_cursor((10., 10.)));
    test.step(|test| test.resize(Size2D::new(500., 500.)));
    test.step(|test| test.click_cursor((10., 10.)));
}
