use std::{
    any::Any,
    rc::Rc,
};

use freya::prelude::*;
use freya_terminal::{
    backends::{
        TerminalBackend,
        TerminalOutput,
    },
    element::Terminal,
    handle::{
        TerminalError,
        TerminalHandle,
        TerminalId,
    },
};
use freya_testing::TestingRunner;

struct TestBackend;

impl TerminalBackend for TestBackend {
    fn start(&mut self, output: TerminalOutput) -> Result<(), TerminalError> {
        output.write("First line\r\nSecond line".as_bytes())
    }

    fn write(&mut self, _data: &[u8]) -> Result<(), TerminalError> {
        Ok(())
    }

    fn resize(&mut self, _rows: u16, _cols: u16) {}
}

fn launch_terminal(
    height: f32,
    font_size: f32,
) -> (TestingRunner, State<(f32, f32)>, TerminalHandle) {
    fn app() -> impl IntoElement {
        let settings = use_consume::<State<(f32, f32)>>();
        let (height, font_size) = *settings.read();
        let handle = use_consume::<TerminalHandle>();
        rect()
            .width(Size::px(400.))
            .height(Size::px(height))
            .child(Terminal::new(handle).font_size(font_size))
    }

    let (runner, (settings, handle)) = TestingRunner::new(
        app,
        (600., 600.).into(),
        |runner| {
            let settings = runner.provide_root_context(|| State::create((height, font_size)));
            let handle = runner.provide_root_context(|| {
                TerminalHandle::new(TerminalId(1), TestBackend, None).unwrap()
            });
            (settings, handle)
        },
        1.,
    );
    (runner, settings, handle)
}

fn terminal_measurement(runner: &TestingRunner) -> Rc<dyn Any> {
    runner
        .find(|node, element| {
            if (element as &dyn Any).is::<Terminal>() {
                node.layout().data
            } else {
                None
            }
        })
        .unwrap()
}

#[test]
fn resizing_terminal_preserves_drawing() {
    let (mut runner, mut settings, handle) = launch_terminal(240., 14.);
    let original_drawing = runner.render();
    let original_measurement = terminal_measurement(&runner);
    let original_rows = handle.term().screen_lines();

    settings.set((120., 14.));
    runner.sync_and_update();
    assert!(handle.term().screen_lines() < original_rows);
    assert!(!Rc::ptr_eq(
        &original_measurement,
        &terminal_measurement(&runner)
    ));
    let smaller_drawing = runner.render();
    assert_eq!(smaller_drawing.as_bytes(), runner.render().as_bytes());

    settings.set((480., 14.));
    runner.sync_and_update();
    assert!(handle.term().screen_lines() > original_rows);
    runner.render_to_surface();

    settings.set((240., 14.));
    runner.sync_and_update();
    assert_eq!(handle.term().screen_lines(), original_rows);
    assert_eq!(original_drawing.as_bytes(), runner.render().as_bytes());
}

#[test]
fn font_change_matches_fresh_drawing() {
    let (mut runner, mut settings, _) = launch_terminal(240., 14.);
    let original_drawing = runner.render();

    settings.set((240., 22.));
    runner.sync_and_update();
    let changed_drawing = runner.render();
    assert_ne!(original_drawing.as_bytes(), changed_drawing.as_bytes());

    let (mut fresh_runner, _, _) = launch_terminal(240., 22.);
    assert_eq!(changed_drawing.as_bytes(), fresh_runner.render().as_bytes());
}
