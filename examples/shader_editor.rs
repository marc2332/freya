use std::{
    any::Any,
    borrow::Cow,
    collections::HashMap,
    rc::Rc,
    time::Instant,
};

use freya::{
    prelude::*,
    text_edit::*,
};
use freya_core::integration::{
    DiffModifies,
    ElementExt,
    RenderContext,
};
use freya_performance_plugin::PerformanceOverlayPlugin;
use skia_safe::{
    Data,
    Paint,
    Rect,
    RuntimeEffect,
    textlayout::{
        ParagraphBuilder,
        ParagraphStyle,
    },
};

const SHADER: &str = "
 uniform vec2 u_resolution;
 uniform float u_time;

 vec4 main(vec2 cords) {
     vec2 U = cords / 55.;

     float t = .8* u_time;
     float r = ceil(U.x + t) + ceil(U.y + t);
     float v = mod(r, 4.) > 1. ? U.x : U.y;
     float b = step(fract(v+.2), .5);

     vec4 C = vec4(.9*b, 0. + abs(sin(t) * 0.5), .6-b, 1.);
     return C;
 }
 ";

fn main() {
    launch(
        LaunchConfig::new()
            .with_window(WindowConfig::new(app).with_size(900., 600.))
            .with_plugin(PerformanceOverlayPlugin::default()),
    )
}

fn app() -> Element {
    let editable = use_editable(
        || SHADER.trim().to_string(),
        EditableConfig::new,
        EditableMode::SingleLineMultipleEditors,
    );

    rect()
        .horizontal()
        .child(ShaderEditor(editable))
        .child(ShaderView(editable))
        .into()
}

#[derive(PartialEq)]
struct ShaderEditor(UseEditable);

impl Render for ShaderEditor {
    fn render(&self) -> Element {
        let mut editable = self.0;

        let on_global_mouse_up = move |_: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Release);
        };

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                code: e.code,
                modifiers: e.modifiers,
            });
        };

        let on_global_key_up = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyUp { code: e.code });
        };

        rect()
            .on_global_mouse_up(on_global_mouse_up)
            .on_global_key_down(on_global_key_down)
            .on_global_key_up(on_global_key_up)
            .width(Size::percent(50.))
            .height(Size::fill())
            .child(
                VirtualScrollView::new(move |line_index, _| {
                    EditingLine {
                        line_index,
                        editable,
                    }
                    .into()
                })
                .length(editable.editor().read().len_lines() as i32)
                .item_size(27.),
            )
            .into()
    }
}

#[derive(PartialEq)]
struct EditingLine {
    line_index: usize,
    editable: UseEditable,
}

impl Render for EditingLine {
    fn render_key(&self) -> DiffKey {
        (&self.line_index).into()
    }
    fn render(&self) -> Element {
        let line_index = self.line_index;
        let mut editable = self.editable;
        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let line = editor.line(line_index).unwrap();

        let is_line_selected = editor.cursor_row() == line_index;

        // Only show the cursor in the active line
        let cursor_index = if is_line_selected {
            Some(editor.cursor_col())
        } else {
            None
        };

        // Only highlight the active line
        let line_background = if is_line_selected {
            (225, 225, 225).into()
        } else {
            Color::TRANSPARENT
        };

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_id: line_index,
                holder: &holder.read(),
            });
        };

        let on_mouse_move = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Move {
                location: e.element_location,
                editor_id: line_index,
                holder: &holder.read(),
            });
        };

        let highlights = editable.editor().read().get_visible_selection(line_index);

        rect()
            .width(Size::fill())
            .height(Size::px(27.))
            .horizontal()
            .color(Color::BLACK)
            .background(line_background)
            .child(
                label()
                    .width(Size::px(60.))
                    .text_align(TextAlign::Center)
                    .font_size(15.)
                    .color((90, 90, 90))
                    .text((line_index + 1).to_string()),
            )
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .on_mouse_down(on_mouse_down)
                    .on_mouse_move(on_mouse_move)
                    .cursor_index(cursor_index)
                    .highlights(highlights.map(|h| vec![h]))
                    .width(Size::fill())
                    .height(Size::fill())
                    .font_size(15.)
                    .max_lines(1)
                    .color((35, 35, 35))
                    .span(line.text.to_string()),
            )
            .into()
    }
}

#[derive(PartialEq)]
struct ShaderView(UseEditable);

impl Render for ShaderView {
    fn render(&self) -> Element {
        let editable = self.0;

        use_hook(|| {
            let mut ticker = consume_root_context::<RenderingTicker>();
            let event_notifier = EventNotifier::get();

            spawn(async move {
                loop {
                    ticker.tick().await;
                    event_notifier.send(UserEvent::RequestRedraw);
                }
            });
        });

        let runtime_effect = use_side_effect_value(move || {
            RuntimeEffect::make_for_shader(editable.editor().read().rope().to_string(), None)
                .map(Rc::from)
        });

        rect()
            .width(Size::percent(50.))
            .height(Size::fill())
            .background((0, 0, 0))
            .child(Shader(runtime_effect.read().clone(), Instant::now()))
            .into()
    }
}

struct Shader(Result<Rc<RuntimeEffect>, String>, Instant);

impl ElementExt for Shader {
    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Owned(LayoutData {
            layout: torin::node::Node::from_size_and_direction(
                Size::Fill,
                Size::Fill,
                Direction::Vertical,
            ),
        })
    }

    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(shader) = (other.as_ref() as &dyn Any).downcast_ref::<Self>() else {
            return true;
        };

        let is_equal = match (&self.0, &shader.0) {
            (Ok(a), Ok(b)) => Rc::ptr_eq(a, b),
            (Err(a), Err(b)) => a == b,
            _ => false,
        };

        !is_equal
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(shader) = (other.as_ref() as &dyn Any).downcast_ref::<Self>() else {
            return DiffModifies::all();
        };

        let is_equal = match (&self.0, &shader.0) {
            (Ok(a), Ok(b)) => Rc::ptr_eq(a, b),
            (Err(a), Err(b)) => a == b,
            _ => false,
        };
        if is_equal {
            DiffModifies::empty()
        } else {
            DiffModifies::STYLE
        }
    }

    fn render(&self, context: RenderContext) {
        match &self.0 {
            Ok(runtime_effect) => {
                let mut builder = UniformsBuilder::default();
                builder.set(
                    "u_resolution",
                    UniformValue::FloatVec(vec![
                        context.layout_node.area.width(),
                        context.layout_node.area.height(),
                    ]),
                );
                builder.set(
                    "u_time",
                    UniformValue::Float(self.1.elapsed().as_secs_f32()),
                );

                let uniforms = Data::new_copy(&builder.build(runtime_effect));

                let shader = runtime_effect.make_shader(uniforms, &[], None).unwrap();

                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_color(Color::WHITE);
                paint.set_shader(shader);

                context.canvas.draw_rect(
                    Rect::new(
                        context.layout_node.area.min_x(),
                        context.layout_node.area.min_y(),
                        context.layout_node.area.max_x(),
                        context.layout_node.area.max_y(),
                    ),
                    &paint,
                );
            }
            Err(err) => {
                let mut text_paint = Paint::default();
                text_paint.set_anti_alias(true);
                text_paint.set_color(Color::WHITE);
                let mut paragraph_builder = ParagraphBuilder::new(
                    &ParagraphStyle::default(),
                    context.font_collection.clone(),
                );
                paragraph_builder.add_text(err);
                let mut paragraph = paragraph_builder.build();
                paragraph.layout(context.layout_node.area.width());

                paragraph.paint(
                    context.canvas,
                    (
                        context.layout_node.area.min_x(),
                        context.layout_node.area.min_y(),
                    ),
                );
            }
        }
    }
}

impl From<Shader> for Element {
    fn from(value: Shader) -> Self {
        Element::Element {
            key: DiffKey::None,
            element: Rc::new(value),
            elements: vec![],
        }
    }
}

/// Pass uniform values to a Shader.
#[derive(Default)]
pub struct UniformsBuilder {
    uniforms: HashMap<String, UniformValue>,
}

/// Uniform value to be passed to a Shader.
pub enum UniformValue {
    Float(f32),
    #[allow(dead_code)]
    FloatVec(Vec<f32>),
}

impl UniformsBuilder {
    /// Set a uniform value.
    pub fn set(&mut self, name: &str, value: UniformValue) {
        self.uniforms.insert(name.to_string(), value);
    }

    /// Build the uniform bytes.
    pub fn build(&self, shader: &RuntimeEffect) -> Vec<u8> {
        let mut values = Vec::new();

        for uniform in shader.uniforms().iter() {
            let value = self.uniforms.get(uniform.name()).unwrap();
            match &value {
                UniformValue::Float(f) => {
                    values.extend(f.to_le_bytes());
                }
                UniformValue::FloatVec(f) => {
                    for n in f {
                        values.extend(n.to_le_bytes());
                    }
                }
            }
        }

        values
    }
}
