use freya::prelude::*;
use freya_testing::prelude::*;
use torin::prelude::{
    Area,
    Point2D,
    Size2D,
};

/// Assert that every pixel that changed between two frames is covered by the damage.
fn assert_damage_covers_diff(before: &[u8], after: &[u8], damage: &Damage) {
    for y in 0..500 {
        for x in 0..500 {
            let offset = (y * 500 + x) * 4;
            if before[offset..offset + 4] != after[offset..offset + 4] {
                let pixel = Area::new(
                    Point2D::new(x as f32 + 0.25, y as f32 + 0.25),
                    Size2D::new(0.5, 0.5),
                );
                assert!(
                    damage.intersects(&pixel),
                    "Changed pixel at ({x}, {y}) is not covered by the damage {damage:?}"
                );
            }
        }
    }
}

/// Render, interact, render again, and assert the collected damage covers the pixel diff.
fn check_interaction(test: &mut TestingRunner, interact: impl FnOnce(&mut TestingRunner)) {
    test.sync_and_update();
    let before = test.render_pixels();
    test.tree().borrow_mut().render_state.last_damage.take();

    interact(test);

    let after = test.render_pixels();
    let damage = test.tree().borrow_mut().render_state.last_damage.take();
    assert_ne!(before, after, "The interaction should change some pixels");
    assert_damage_covers_diff(&before, &after, &damage);
}

/// Cycles through a background change, a text change, a shadow growing while a
/// node is added, and back.
#[derive(PartialEq)]
struct CompoundApp;

impl Component for CompoundApp {
    fn render(&self) -> impl IntoElement {
        let mut step = use_state(|| 0usize);

        rect()
            .padding(50.)
            .spacing(10.)
            .on_mouse_up(move |_| {
                let next = (*step.peek() + 1) % 3;
                step.set(next);
            })
            .child(
                rect()
                    .width(Size::px(100.))
                    .height(Size::px(100.))
                    .background(if step() == 1 {
                        (200, 50, 50)
                    } else {
                        (50, 50, 200)
                    })
                    .shadow(
                        Shadow::new()
                            .x(10.)
                            .y(10.)
                            .blur(if step() == 2 { 40. } else { 5. })
                            .color((0, 0, 0)),
                    ),
            )
            .child(label().color((10, 10, 10)).text(if step() == 1 {
                "A longer text content"
            } else {
                "Short"
            }))
            .maybe_child((step() == 2).then(|| {
                rect()
                    .width(Size::px(150.))
                    .height(Size::px(80.))
                    .background((50, 200, 50))
            }))
    }
}

#[test]
pub fn damage_covers_changes() {
    let mut test = launch_test(|| CompoundApp);
    for _ in 0..3 {
        check_interaction(&mut test, |test| test.click_cursor((25., 25.)));
    }
}

#[derive(PartialEq)]
struct ScrollApp;

impl Component for ScrollApp {
    fn render(&self) -> impl IntoElement {
        ScrollView::new().children((0..30).map(|i| {
            rect()
                .width(Size::px(200.))
                .height(Size::px(50.))
                .background(if i % 2 == 0 {
                    (240, 240, 240)
                } else {
                    (30, 30, 30)
                })
        }))
    }
}

#[test]
pub fn damage_covers_scroll() {
    let mut test = launch_test(|| ScrollApp);
    check_interaction(&mut test, |test| test.scroll((250., 250.), (0., -175.)));
    check_interaction(&mut test, |test| test.scroll((250., 250.), (0., -600.)));
    check_interaction(&mut test, |test| test.scroll((250., 250.), (0., 300.)));
}
