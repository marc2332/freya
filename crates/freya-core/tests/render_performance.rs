use std::{
    cell::Cell,
    rc::Rc,
    time::Duration,
};

use freya::{
    animation::*,
    code_editor::*,
    prelude::{
        Button,
        Input,
        ScrollView,
        Size,
        Switch,
        VirtualScrollView,
    },
};
use freya_core::{
    element::AppComponent,
    elements::image::ImageHandle,
    prelude::*,
};
use freya_engine::prelude::{
    SkData,
    SkImage,
};
use freya_testing::TestingRunner;

const ITERATIONS: usize = 100;
const COLUMNS: usize = 45;
const ROWS: usize = 40;
const CELL_WIDTH: f32 = 54.;
const CELL_HEIGHT: f32 = 32.;

#[derive(PartialEq)]
struct ControlPanel {
    index: usize,
    key: DiffKey,
}

impl KeyExt for ControlPanel {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for ControlPanel {
    fn render(&self) -> impl IntoElement {
        let mut revision = use_state(|| 0usize);
        let text = use_state(|| "Edit me".to_string());
        let count = use_consume::<Rc<Cell<usize>>>();

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(8.)
            .spacing(4.)
            .background((235, 235, 240))
            .child(format!("Panel {}, updates {}", self.index, revision()))
            .child(
                Button::new()
                    .on_press(move |_| {
                        *revision.write() += 1;
                        count.set(count.get() + 1);
                    })
                    .child("Update"),
            )
            .child(Input::new(text))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq)]
struct TextBlock {
    index: usize,
    lines: usize,
    key: DiffKey,
}

impl KeyExt for TextBlock {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for TextBlock {
    fn render(&self) -> impl IntoElement {
        let mut revision = use_state(|| 0usize);
        let count = use_consume::<Rc<Cell<usize>>>();
        let revision_value = revision();

        paragraph()
            .width(Size::fill())
            .font_size(14.)
            .padding(8.)
            .span(
                (0..self.lines)
                    .map(|line| {
                        format!(
                            "Block {}, line {line}, revision {revision_value}. Some text with enough words to wrap across the paragraph.\n",
                            self.index,
                        )
                    })
                    .collect::<String>(),
            )
            .on_press(move |_| {
                *revision.write() += 1;
                count.set(count.get() + 1);
            })
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq)]
struct ImageTile {
    handle: ImageHandle,
    key: DiffKey,
}

impl KeyExt for ImageTile {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for ImageTile {
    fn render(&self) -> impl IntoElement {
        let mut hovered = use_state(|| false);
        let count = use_consume::<Rc<Cell<usize>>>();
        let is_hovered = hovered();

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(8.)
            .corner_radius(8.)
            .background(if is_hovered {
                Color::BLUE
            } else {
                Color::WHITE
            })
            .opacity(if is_hovered { 0.65 } else { 1. })
            .on_pointer_enter(move |_| {
                hovered.set_if_modified(true);
                count.set(count.get() + 1);
            })
            .on_pointer_leave(move |_| hovered.set_if_modified(false))
            .child(
                image(self.handle.clone())
                    .width(Size::fill())
                    .height(Size::fill()),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(Default, PartialEq)]
struct StatefulSwitch {
    key: DiffKey,
}

impl KeyExt for StatefulSwitch {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for StatefulSwitch {
    fn render(&self) -> impl IntoElement {
        let mut toggled = use_state(|| false);
        let count = use_consume::<Rc<Cell<usize>>>();

        Switch::new().toggled(toggled()).on_toggle(move |_| {
            toggled.toggle();
            count.set(count.get() + 1);
        })
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

fn controls_app(panels: usize) -> impl IntoElement {
    rect()
        .expanded()
        .spacing(4.)
        .children((0..panels.div_ceil(3)).map(|row| {
            rect()
                .horizontal()
                .height(Size::px(140.))
                .spacing(4.)
                .children((row * 3..((row + 1) * 3).min(panels)).map(|index| {
                    rect().width(Size::percent(100. / 3.)).child(ControlPanel {
                        index,
                        key: (&index).into(),
                    })
                }))
        }))
}

fn switches_app() -> impl IntoElement {
    rect().expanded().children((0..ROWS).map(|row| {
        rect()
            .width(Size::fill())
            .height(Size::px(CELL_HEIGHT))
            .horizontal()
            .children((0..COLUMNS).map(move |column| {
                let index = row * COLUMNS + column;
                rect()
                    .width(Size::px(CELL_WIDTH))
                    .child(StatefulSwitch::default().key(index))
            }))
    }))
}

fn images_app() -> impl IntoElement {
    let handle = use_consume::<ImageHandle>();
    rect().expanded().children((0..2).map(|row| {
        let handle = handle.clone();
        rect()
            .horizontal()
            .height(Size::percent(50.))
            .children((0..5).map(move |column| {
                let index = row * 5 + column;
                rect().width(Size::percent(20.)).child(ImageTile {
                    handle: handle.clone(),
                    key: (&index).into(),
                })
            }))
    }))
}

fn scroll_item(index: usize) -> impl IntoElement {
    rect()
        .key(index)
        .width(Size::fill())
        .height(Size::px(32.))
        .horizontal()
        .padding(4.)
        .background(if index % 2 == 0 {
            (240, 240, 240)
        } else {
            (220, 220, 225)
        })
        .child(format!("Item {index}"))
        .child(Button::new().child("Action"))
}

fn editor_app() -> impl IntoElement {
    let mut editor = use_consume::<State<CodeEditorData>>();
    use_hook(move || editor.write().measure(14., "Jetbrains Mono"));
    let a11y_id = use_a11y();
    CodeEditor::new(editor, a11y_id).a11y_auto_focus(true)
}

fn animation_app() -> impl IntoElement {
    let animation = use_animation(|config| {
        config.on_creation(OnCreation::Run);
        config.on_finish(OnFinish::reverse());
        AnimNum::new(0., 650.)
            .time(400)
            .ease(Ease::InOut)
            .function(Function::Sine)
    });
    let progress = animation.get().value();
    let changed = use_consume::<Rc<Cell<bool>>>();
    if progress > 0. {
        changed.set(true);
    }

    rect().expanded().center().children((0..32).map(|index| {
        rect()
            .key(index)
            .offset_x(progress - index as f32 * 10.)
            .horizontal()
            .children(
                [
                    (7, 102, 173),
                    (166, 207, 152),
                    (179, 19, 18),
                    (255, 108, 34),
                ]
                .map(|color| {
                    rect()
                        .width(Size::px(45.))
                        .height(Size::px(25.))
                        .background(color)
                        .corner_radius(100.)
                }),
            )
    }))
}

struct RenderWorkload {
    runner: TestingRunner,
}

impl RenderWorkload {
    fn run(&mut self, action: impl FnMut(&mut TestingRunner, usize)) {
        let initial = self.runner.render();
        self.run_with_interval(Duration::ZERO, action);
        assert_ne!(initial.as_bytes(), self.runner.render().as_bytes());
    }

    fn run_with_interval(
        &mut self,
        interval: Duration,
        mut action: impl FnMut(&mut TestingRunner, usize),
    ) {
        for iteration in 0..ITERATIONS {
            if !interval.is_zero() {
                std::thread::sleep(interval);
            }
            hotpath::measure_block!("Interaction to rendered frame", {
                hotpath::measure_block!("Interaction and update", {
                    action(&mut self.runner, iteration);
                    self.runner.poll_n(Duration::ZERO, 2);
                });
                hotpath::measure_block!("Painting", {
                    self.runner.render_to_surface();
                });
            });
        }
    }

    fn buttons(&self) -> Vec<(f64, f64)> {
        self.runner.find_many(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| {
                    rect.accessibility.builder.role() == AccessibilityRole::Button
                        && node.is_visible()
                })
                .map(|_| {
                    let center = node.layout().visible_area().center();
                    (f64::from(center.x), f64::from(center.y))
                })
        })
    }
}

fn mount(app: impl Into<AppComponent>) -> (RenderWorkload, Rc<Cell<usize>>) {
    let (mut runner, count) = TestingRunner::new(
        app,
        (900., 600.).into(),
        |runner| runner.provide_root_context(|| Rc::new(Cell::new(0usize))),
        1.,
    );
    runner.animation_clock().disable();
    runner.render_to_surface();
    (RenderWorkload { runner }, count)
}

#[test]
#[ignore = "Render profiling workload"]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [50, 95, 99]))]
fn small_ui() {
    let (mut workload, count) = mount(|| controls_app(2));
    let targets = workload.buttons();
    assert_eq!(targets.len(), 2);

    workload.run(|runner, iteration| runner.click_cursor(targets[iteration % targets.len()]));
    assert_eq!(count.get(), ITERATIONS);
}

#[test]
#[ignore = "Render profiling workload"]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [50, 95, 99]))]
fn medium_ui() {
    let (mut workload, count) = mount(|| controls_app(12));
    let targets = workload.buttons();
    assert_eq!(targets.len(), 12);

    workload.run(|runner, iteration| runner.click_cursor(targets[iteration % targets.len()]));
    assert_eq!(count.get(), ITERATIONS);
}

#[test]
#[ignore = "Render profiling workload"]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [50, 95, 99]))]
fn large_switches() {
    let (mut runner, count) = TestingRunner::new(
        switches_app,
        (COLUMNS as f32 * CELL_WIDTH, ROWS as f32 * CELL_HEIGHT).into(),
        |runner| runner.provide_root_context(|| Rc::new(Cell::new(0usize))),
        1.,
    );
    runner.animation_clock().disable();
    runner.render_to_surface();
    let mut workload = RenderWorkload { runner };

    workload.run(|runner, iteration| {
        let index = 1 + iteration * 17 % (COLUMNS * ROWS - 1);
        runner.click_cursor((
            (index % COLUMNS) as f64 * f64::from(CELL_WIDTH) + 24.,
            (index / COLUMNS) as f64 * f64::from(CELL_HEIGHT) + 14.,
        ));
    });
    assert_eq!(count.get(), ITERATIONS);
}

#[test]
#[ignore = "Render profiling workload"]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [50, 95, 99]))]
fn images_medium() {
    let bytes = Bytes::from_static(include_bytes!("../../../examples/rust_logo.png"));
    let image = SkImage::from_encoded(SkData::new_copy(&bytes)).expect("Image must decode");
    let (mut runner, count) = TestingRunner::new(
        images_app,
        (900., 600.).into(),
        |runner| {
            runner.provide_root_context(|| ImageHandle::new(image, bytes));
            runner.provide_root_context(|| Rc::new(Cell::new(0usize)))
        },
        1.,
    );
    runner.render_to_surface();
    runner.move_cursor((-10., -10.));
    runner.render_to_surface();
    count.set(0);
    let mut workload = RenderWorkload { runner };

    workload.run(|runner, iteration| {
        let index = iteration % 10;
        runner.move_cursor((
            (index % 5) as f64 * 180. + 90.,
            (index / 5) as f64 * 300. + 150.,
        ));
    });
    assert_eq!(count.get(), ITERATIONS);
}

#[test]
#[ignore = "Render profiling workload"]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [50, 95, 99]))]
fn medium_text() {
    let (mut workload, count) = mount(|| {
        ScrollView::new().children((0..20).map(|index| TextBlock {
            index,
            lines: 5,
            key: (&index).into(),
        }))
    });
    workload.run(|runner, _| runner.click_cursor((100., 40.)));
    assert_eq!(count.get(), ITERATIONS);
}

#[test]
#[ignore = "Render profiling workload"]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [50, 95, 99]))]
fn large_text() {
    let (mut workload, count) = mount(|| {
        ScrollView::new().children((0..200).map(|index| TextBlock {
            index,
            lines: 20,
            key: (&index).into(),
        }))
    });
    workload.run(|runner, _| runner.click_cursor((100., 40.)));
    assert_eq!(count.get(), ITERATIONS);
}

#[test]
#[ignore = "Render profiling workload"]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [50, 95, 99]))]
fn medium_code_editor_typing() {
    let source = (0..200)
        .map(|line| format!("fn function_{line}() -> usize {{ {line} }}\n"))
        .collect::<String>();
    let source = format!("// Typing here\n{source}");
    let initial_length = source.chars().count();
    let (mut runner, editor) = TestingRunner::new(
        editor_app,
        (900., 600.).into(),
        |runner| {
            runner.provide_root_context(|| {
                let language = EditorLanguage::new(
                    tree_sitter_rust::LANGUAGE,
                    tree_sitter_rust::HIGHLIGHTS_QUERY,
                );
                let mut editor = CodeEditorData::new(Rope::from_str(&source), language);
                editor.parse();
                State::create(editor)
            })
        },
        1.,
    );
    runner.animation_clock().disable();
    runner.sync_and_update();
    runner.press_key(Key::Named(NamedKey::End));
    runner.render_to_surface();
    let mut workload = RenderWorkload { runner };

    workload.run(|runner, _| runner.write_text("x"));
    let editor = editor.peek();
    assert_eq!(editor.rope.len_chars(), initial_length + ITERATIONS);
    assert_eq!(
        editor.rope.line(0).to_string(),
        format!("// Typing here{}\n", "x".repeat(ITERATIONS)),
    );
}

#[test]
#[ignore = "Render profiling workload"]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [50, 95, 99]))]
fn medium_scrolling() {
    let (mut workload, _) = mount(|| ScrollView::new().children((0..200).map(scroll_item)));
    workload.run(|runner, _| runner.scroll((450., 300.), (0., -32.)));
}

#[test]
#[ignore = "Render profiling workload"]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [50, 95, 99]))]
fn medium_virtualscrolling() {
    let (mut workload, _) = mount(|| {
        VirtualScrollView::new(|item, _| scroll_item(item.index).into_element())
            .length(200usize)
            .item_size(32.)
    });
    workload.run(|runner, _| runner.scroll((450., 300.), (0., -32.)));
}

#[test]
#[ignore = "Render profiling workload"]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [50, 95, 99]))]
fn medium_animation() {
    let (mut runner, changed) = TestingRunner::new(
        animation_app,
        (900., 600.).into(),
        |runner| runner.provide_root_context(|| Rc::new(Cell::new(false))),
        1.,
    );
    runner.render_to_surface();
    let mut workload = RenderWorkload { runner };

    workload.run_with_interval(Duration::from_millis(16), |_, _| {});
    assert!(changed.get());
}
