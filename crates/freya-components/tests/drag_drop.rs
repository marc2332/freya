use freya::prelude::*;
use freya_testing::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
struct Dropped(Option<usize>);

fn drag_drop_app() -> impl IntoElement {
    let mut dropped = use_consume::<State<Dropped>>();

    rect()
        .child(
            DragZone::new(1usize).child(
                rect()
                    .width(Size::px(100.))
                    .height(Size::px(100.))
                    .background((125, 189, 25)),
            ),
        )
        .child(
            DropZone::new(move |data: usize| dropped.set(Dropped(Some(data)))).child(
                rect()
                    .width(Size::px(150.))
                    .height(Size::px(150.))
                    .background((25, 189, 125)),
            ),
        )
}

fn center_of(test: &mut TestingRunner, background: Color) -> CursorPoint {
    test.find(|node, element| {
        Rect::try_downcast(element)
            .filter(|rect| rect.style.background == Fill::Color(background))
            .map(move |_| node)
    })
    .unwrap()
    .layout()
    .area
    .center()
    .to_f64()
}

fn drag_and_drop(
    press: fn(&mut TestingRunner, CursorPoint),
    move_to: fn(&mut TestingRunner, CursorPoint),
    release: fn(&mut TestingRunner, CursorPoint),
) {
    let (mut test, dropped) = TestingRunner::new(
        drag_drop_app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(Dropped::default())),
        1.,
    );
    test.sync_and_update();

    let drag_center = center_of(&mut test, Color::from_rgb(125, 189, 25));
    let drop_center = center_of(&mut test, Color::from_rgb(25, 189, 125));

    press(&mut test, drag_center);
    move_to(&mut test, drop_center);
    release(&mut test, drop_center);

    assert_eq!(dropped.peek().0, Some(1));
}

#[test]
pub fn drag_drop_with_mouse() {
    drag_and_drop(
        TestingRunner::press_cursor,
        TestingRunner::move_cursor,
        TestingRunner::release_cursor,
    );
}

#[test]
pub fn drag_drop_with_touch() {
    drag_and_drop(
        TestingRunner::press_touch,
        TestingRunner::move_touch,
        TestingRunner::release_touch,
    );
}
