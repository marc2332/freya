use std::{collections::HashMap, sync::Arc};

use dioxus_core::{AttributeValue, Scope, ScopeState};
use freya_common::Area;
use freya_node_state::{CanvasReference, CustomAttributeValues};
use skia_safe::{Canvas, RuntimeEffect};
use uuid::Uuid;

pub type RenderCallback = Box<dyn Fn(&mut Canvas, Area)>;

/// Holds the rendering hook ID.
pub struct UseCanvas {
    id: Uuid,
    renderer: Arc<RenderCallback>,
}

impl PartialEq for UseCanvas {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl UseCanvas {
    pub fn attribute<'a, T>(&self, cx: Scope<'a, T>) -> AttributeValue<'a> {
        cx.any_value(CustomAttributeValues::Canvas(CanvasReference {
            runner: self.renderer.clone(),
        }))
    }
}

/// Register a rendering hook to gain access to the Canvas.
///
/// ## Usage
/// ```rust
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     let canvas = use_canvas(cx, || {
///         Box::new(|canvas, area| {
///             // Draw using the canvas !
///         })
///     });
///
///     render!(
///         Canvas {
///             canvas: canvas
///         }
///     )
/// }
/// ```
pub fn use_canvas(cx: &ScopeState, renderer: impl FnOnce() -> RenderCallback) -> UseCanvas {
    let id = cx.use_hook(Uuid::new_v4);
    let renderer = cx.use_hook(|| Arc::new(renderer()));

    UseCanvas {
        id: *id,
        renderer: renderer.clone(),
    }
}

#[derive(Default)]
pub struct UniformsBuilder {
    uniforms: HashMap<String, UniformValue>,
}

pub enum UniformValue {
    Float(f32),
    #[allow(dead_code)]
    FloatVec(Vec<f32>),
}

impl UniformsBuilder {
    pub fn set(&mut self, name: &str, value: UniformValue) {
        self.uniforms.insert(name.to_string(), value);
    }

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
