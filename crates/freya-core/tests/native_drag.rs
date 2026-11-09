use std::pin::pin;

use freya_core::{
    integration::*,
    prelude::*,
};
use freya_testing::TestingRunner;
use futures_lite::future::{
    block_on,
    poll_once,
};
use torin::prelude::{
    CursorPoint,
    Size,
};

fn over(
    test: &mut TestingRunner,
    cursor: (f64, f64),
    data: &DragDataHandle,
) -> Option<(NodeId, DropAction)> {
    let decision = DragDecision::default();
    test.send_event(PlatformEvent::Drag {
        name: DragEventName::Over,
        cursor: cursor.into(),
        data: data.clone(),
        decision: Some(decision.clone()),
        target: None,
    });
    test.sync_and_update();

    decision.accepted()
}

#[test]
fn nearest_accepting_node_wins_and_receives_the_drop() {
    let (mut test, trace) = TestingRunner::new(
        || {
            let mut trace = use_consume::<State<Vec<&'static str>>>();
            rect()
                .expanded()
                .padding(5.)
                .background(Color::WHITE)
                .on_drag_over(|event: Event<DragEventData>| event.accept(DropAction::Move))
                .on_drop(move |_| trace.write().push("parent"))
                .child(
                    rect()
                        .width(Size::px(50.))
                        .height(Size::px(50.))
                        .background(Color::RED)
                        .on_drag_over(|event: Event<DragEventData>| {
                            assert_eq!(event.global_location, CursorPoint::new(10., 10.));
                            assert_eq!(event.element_location, CursorPoint::new(5., 5.));
                            event.accept(DropAction::Copy);
                        })
                        .on_drop(move |_| trace.write().push("child")),
                )
        },
        (200., 100.).into(),
        |runner| runner.provide_root_context(|| State::create(Vec::<&'static str>::new())),
        2.,
    );
    test.sync_and_update();
    let data = DragDataHandle::new(vec![DataFormat::Text]);
    test.send_event(PlatformEvent::Drag {
        name: DragEventName::Drop,
        cursor: (20., 20.).into(),
        data: data.clone(),
        decision: None,
        target: None,
    });
    test.sync_and_update();
    assert!(trace.peek().is_empty());

    let (target, action) = over(&mut test, (20., 20.), &data).unwrap();
    assert_eq!(action, DropAction::Copy);
    test.send_event(PlatformEvent::Drag {
        name: DragEventName::Drop,
        cursor: (20., 20.).into(),
        data: data.clone(),
        decision: None,
        target: Some(target),
    });
    test.sync_and_update();
    assert_eq!(*trace.peek(), ["child", "parent"]);
    assert_eq!(
        over(&mut test, (150., 20.), &data).unwrap().1,
        DropAction::Move
    );
}

#[test]
fn cloned_handles_share_pending_and_cached_reads() {
    let data = DragDataHandle::new(vec![DataFormat::Text]);
    let clone = data.clone();
    let mut first = pin!(data.read_text());
    let mut second = pin!(clone.read_text());
    assert!(block_on(poll_once(first.as_mut())).is_none());
    assert!(block_on(poll_once(second.as_mut())).is_none());

    let mut requests = data.take_read_requests().unwrap();
    assert_eq!(requests.len(), 1);
    let request = requests.pop().unwrap();
    assert_eq!(request.format, DataFormat::Text);
    request
        .response
        .send(Ok(DataValue::Text("Shared".into())))
        .unwrap();
    assert!(data.take_read_requests().unwrap().is_empty());
    assert_eq!(block_on(first).unwrap(), "Shared");
    assert_eq!(block_on(second).unwrap(), "Shared");

    data.expire();
    assert_eq!(block_on(data.read_text()).unwrap(), "Shared");
    assert_eq!(block_on(data.read_files()), Err(DragError::Expired));
}

#[test]
fn expiration_cancels_queued_reads() {
    let data = DragDataHandle::new(vec![DataFormat::Text]);
    let mut read = pin!(data.read_text());
    assert!(block_on(poll_once(read.as_mut())).is_none());

    data.expire();

    assert!(data.take_read_requests().unwrap().is_empty());
    assert_eq!(block_on(read), Err(DragError::Expired));
    assert_eq!(block_on(data.read_text()), Err(DragError::Expired));
}

#[test]
fn expiration_keeps_started_reads() {
    let data = DragDataHandle::new(vec![DataFormat::Text]);
    let mut read = pin!(data.read_text());
    assert!(block_on(poll_once(read.as_mut())).is_none());
    let request = data.take_read_requests().unwrap().pop().unwrap();

    data.expire();
    request
        .response
        .send(Ok(DataValue::Text("Received".into())))
        .unwrap();

    assert_eq!(block_on(read).unwrap(), "Received");
    assert_eq!(block_on(data.read_text()).unwrap(), "Received");
}
