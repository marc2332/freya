use freya::prelude::*;
use freya_testing::prelude::*;

#[derive(PartialEq)]
struct PanelContent {
    name: &'static str,
    key: DiffKey,
}

impl KeyExt for PanelContent {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for PanelContent {
    fn render(&self) -> impl IntoElement {
        let mut count = use_state(|| 0);
        rect()
            .expanded()
            .on_press(move |_| *count.write() += 1)
            .child(
                label()
                    .expanded()
                    .text(format!("{}:{}", self.name, count())),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(Clone, PartialEq)]
struct Configuration {
    panels: Vec<ResizablePanel>,
    direction: Direction,
}

fn app() -> impl IntoElement {
    let configuration = use_consume::<State<Configuration>>();
    let controller = use_consume::<State<ResizableContext>>();
    let configuration = configuration.read();

    ResizableContainer::new()
        .direction(configuration.direction)
        .controller(controller)
        .panels_iter(configuration.panels.iter().cloned())
}

fn launch(
    panels: Vec<ResizablePanel>,
) -> (TestingRunner, State<Configuration>, State<ResizableContext>) {
    let (test, (configuration, controller)) = TestingRunner::new(
        app,
        (500., 500.).into(),
        move |runner| {
            let configuration = runner.provide_root_context(|| {
                State::create(Configuration {
                    panels,
                    direction: Direction::Horizontal,
                })
            });
            let controller =
                runner.provide_root_context(|| State::create(ResizableContext::default()));
            (configuration, controller)
        },
        1.,
    );
    (test, configuration, controller)
}

fn panel(name: &'static str, initial_size: PanelSize) -> ResizablePanel {
    ResizablePanel::new(initial_size)
        .key(name)
        .child(PanelContent {
            name,
            key: DiffKey::None,
        })
}

fn panels(names: &[&'static str]) -> Vec<ResizablePanel> {
    names
        .iter()
        .map(|&name| panel(name, PanelSize::px(150.)))
        .collect()
}

fn sizes(controller: State<ResizableContext>) -> Vec<f32> {
    controller
        .peek()
        .panels()
        .iter()
        .map(|panel| panel.size)
        .collect()
}

fn snapshot(test: &TestingRunner) -> Vec<(String, f32)> {
    test.find_many(|node, element| Label::try_downcast(element).map(|_| node))
        .iter()
        .map(|node| {
            (
                Label::try_downcast(&*node.element())
                    .unwrap()
                    .text
                    .to_string(),
                node.layout().area.width(),
            )
        })
        .collect()
}

#[test]
fn keyed_moves_preserve_sizes_and_child_state() {
    let (mut test, mut configuration, mut controller) = launch(panels(&["A", "B"]));
    test.sync_and_update();
    test.click_cursor((50., 50.));
    assert!(controller.write().apply_resize(1, 30., 500.));
    test.sync_and_update();
    assert_eq!(
        snapshot(&test),
        vec![("A:1".into(), 180.), ("B:0".into(), 120.)]
    );

    configuration.write().panels.swap(0, 1);
    test.sync_and_update();
    assert_eq!(
        snapshot(&test),
        vec![("B:0".into(), 120.), ("A:1".into(), 180.)]
    );
    assert_eq!(sizes(controller), vec![120., 180.]);

    test.press_cursor((122., 50.));
    test.move_cursor((142., 50.));
    test.release_cursor((142., 50.));
    let panels = snapshot(&test);
    assert!(panels[0].1 > 120.);
    assert!(panels[1].1 < 180.);
    assert_eq!(panels[1].0, "A:1");
}

#[test]
fn hide_show_uses_declaration_order_and_resets_removed_panels() {
    let (mut test, mut configuration, mut controller) = launch(panels(&["A", "B"]));
    test.sync_and_update();
    test.click_cursor((50., 50.));
    controller.write().apply_resize(1, 30., 500.);
    test.sync_and_update();

    configuration.write().panels = panels(&["B"]);
    test.sync_and_update();
    assert_eq!(snapshot(&test), vec![("B:0".into(), 120.)]);

    configuration.write().panels = panels(&["A", "B"]);
    test.sync_and_update();
    assert_eq!(
        snapshot(&test),
        vec![("A:0".into(), 150.), ("B:0".into(), 120.)]
    );

    test.press_cursor((152., 50.));
    test.move_cursor((172., 50.));
    test.release_cursor((172., 50.));
    let panels = snapshot(&test);
    assert!(panels[0].1 > 150.);
    assert!(panels[1].1 < 120.);
}

#[test]
fn middle_insertion_and_removal_preserve_surviving_panels() {
    let (mut test, mut configuration, mut controller) = launch(panels(&["A", "C"]));
    test.sync_and_update();
    test.click_cursor((200., 50.));
    controller.write().apply_resize(1, 20., 500.);
    test.sync_and_update();

    configuration.write().panels = panels(&["A", "B", "C"]);
    test.sync_and_update();
    assert_eq!(
        snapshot(&test),
        vec![
            ("A:0".into(), 170.),
            ("B:0".into(), 150.),
            ("C:1".into(), 130.),
        ]
    );

    configuration.write().panels = panels(&["A", "C"]);
    test.sync_and_update();
    assert_eq!(
        snapshot(&test),
        vec![("A:0".into(), 170.), ("C:1".into(), 130.)]
    );
}

#[test]
fn structural_changes_cancel_an_active_drag() {
    for (names, direction) in [
        (vec!["B", "A"], Direction::Horizontal),
        (vec!["A", "C", "B"], Direction::Horizontal),
        (vec!["B"], Direction::Horizontal),
        (vec!["A", "B"], Direction::Vertical),
    ] {
        let (mut test, mut configuration, controller) = launch(panels(&["A", "B"]));
        test.sync_and_update();
        test.press_cursor((152., 50.));

        configuration.set(Configuration {
            panels: panels(&names),
            direction,
        });
        test.sync_and_update();
        test.move_cursor((200., 200.));
        test.release_cursor((200., 200.));
        assert_eq!(sizes(controller), vec![150.; names.len()]);
    }
}

#[test]
fn only_the_active_divider_resizes_until_pointer_up() {
    let (mut test, _, controller) = launch(panels(&["A", "B", "C"]));
    test.sync_and_update();
    test.press_cursor((152., 50.));
    test.move_cursor((172., 50.));
    let resized = sizes(controller);
    assert!(resized[0] > 150.);
    assert!(resized[1] < 150.);
    assert_eq!(resized[2], 150.);

    test.release_cursor((172., 50.));
    test.move_cursor((200., 50.));
    assert_eq!(sizes(controller), resized);
}

#[test]
fn empty_containers_and_stale_divider_indices_are_safe() {
    let (mut test, mut configuration, mut controller) = launch(panels(&["A", "B"]));
    test.sync_and_update();
    assert!(!controller.write().apply_resize(0, 20., 500.));
    assert!(!controller.write().apply_resize(2, 20., 500.));

    configuration.write().panels.clear();
    test.sync_and_update();
    assert!(snapshot(&test).is_empty());
    assert!(controller.peek().panels().is_empty());
    assert!(!controller.write().apply_resize(1, 20., 500.));

    configuration.write().panels = panels(&["A"]);
    test.sync_and_update();
    assert_eq!(snapshot(&test), vec![("A:0".into(), 150.)]);
}

#[test]
fn percentage_reorders_do_not_redistribute_sizes() {
    let (mut test, mut configuration, mut controller) = launch(vec![
        panel("A", PanelSize::percent(50.)),
        panel("B", PanelSize::percent(50.)),
    ]);
    test.sync_and_update();
    assert!(controller.write().apply_resize(1, 50., 500.));
    test.sync_and_update();
    let previous = sizes(controller);

    configuration.write().panels.swap(0, 1);
    test.sync_and_update();
    assert_eq!(sizes(controller), vec![previous[1], previous[0]]);
}

#[test]
fn percentage_membership_changes_redistribute_centrally() {
    let (mut test, mut configuration, mut controller) = launch(vec![
        panel("A", PanelSize::percent(50.)),
        panel("B", PanelSize::percent(50.)),
    ]);
    test.sync_and_update();
    controller.write().apply_resize(1, 50., 500.);
    test.sync_and_update();

    let left = configuration.write().panels.remove(0);
    test.sync_and_update();
    assert_eq!(sizes(controller), vec![50.]);

    configuration.write().panels.insert(0, left);
    test.sync_and_update();
    assert_eq!(sizes(controller), vec![50., 50.]);
}

#[test]
fn changed_constraints_and_sizing_modes_follow_declarations() {
    let (mut test, mut configuration, mut controller) = launch(panels(&["A", "B"]));
    test.sync_and_update();
    controller.write().apply_resize(1, 30., 500.);
    test.sync_and_update();

    configuration.write().panels[1] = panel("B", PanelSize::px(150.)).min_size(140.);
    test.sync_and_update();
    assert_eq!(sizes(controller), vec![180., 140.]);

    configuration.set(Configuration {
        panels: vec![
            panel("A", PanelSize::px(150.)),
            panel("B", PanelSize::percent(50.)),
        ],
        direction: Direction::Vertical,
    });
    test.sync_and_update();
    assert_eq!(sizes(controller), vec![180., 50.]);
    assert_eq!(controller.peek().direction(), Direction::Vertical);

    controller.write().reset();
    test.sync_and_update();
    assert_eq!(sizes(controller), vec![150., 50.]);
}

#[test]
fn snapshots_cannot_modify_membership_or_sizes() {
    let (mut test, _, controller) = launch(panels(&["A"]));
    test.sync_and_update();

    let mut snapshot = controller.peek().panels();
    snapshot[0].size = 400.;
    snapshot.clear();
    assert_eq!(sizes(controller), vec![150.]);
}

#[test]
fn unkeyed_sizes_follow_positions_when_content_moves() {
    let (mut test, mut configuration, mut controller) = launch(vec![
        ResizablePanel::new(PanelSize::px(150.)).child("Left"),
        ResizablePanel::new(PanelSize::px(150.)).child("Right"),
    ]);
    test.sync_and_update();
    controller.write().apply_resize(1, 30., 500.);
    test.sync_and_update();

    configuration.write().panels.swap(0, 1);
    test.sync_and_update();
    assert_eq!(sizes(controller), vec![180., 120.]);
}
