use std::sync::Arc;

use freya_engine::prelude::*;
use torin::prelude::Area;

pub trait ShaderProvider: Send + Sync {
    fn prepare_shader(&self, effect: &RuntimeEffect, bounds: Area) -> Option<Shader>;
}

impl<F> ShaderProvider for F
where
    F: Fn(&RuntimeEffect, Area) -> Option<Shader> + Send + Sync,
{
    fn prepare_shader(&self, effect: &RuntimeEffect, bounds: Area) -> Option<Shader> {
        self(effect, bounds)
    }
}

#[derive(Clone)]
pub struct ShaderFill {
    effect: Option<RuntimeEffect>,
    provider: Arc<dyn ShaderProvider>,
}

unsafe impl Send for ShaderFill {}
unsafe impl Sync for ShaderFill {}

impl ShaderFill {
    pub fn new<F>(sksl: impl AsRef<str>, provider: F) -> Self
    where
        F: Fn(&RuntimeEffect, Area) -> Option<Shader> + Send + Sync + 'static,
    {
        Self::from_provider(sksl, provider)
    }

    pub fn from_provider<S>(sksl: impl AsRef<str>, provider: S) -> Self
    where
        S: ShaderProvider + Send + Sync + 'static,
    {
        let effect = RuntimeEffect::make_for_shader(sksl, None)
            .map_err(|err| {
                tracing::error!("Failed to create shader: {err}");
                err
            })
            .ok();

        Self {
            effect,
            provider: Arc::new(provider),
        }
    }

    /// Prepare the shader for use by providing the necessary uniforms.
    /// Returns [None] if the shader could not be prepared, in which case the renderer will fallback to no fill
    pub fn prepare(&self, bounds: Area) -> Option<Shader> {
        self.effect
            .as_ref()
            .and_then(|effect| self.provider.prepare_shader(effect, bounds))
    }
}

impl std::fmt::Display for ShaderFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shader({:p})", Arc::as_ptr(&self.provider))
    }
}

impl std::fmt::Debug for ShaderFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FillShader")
            .field("effect", &self.effect)
            .finish()
    }
}

impl PartialEq for ShaderFill {
    fn eq(&self, other: &Self) -> bool {
        if let Some(effect) = &self.effect
            && let Some(other_effect) = &other.effect
        {
            std::ptr::eq(effect.inner(), other_effect.inner())
        } else {
            Arc::ptr_eq(&self.provider, &other.provider)
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ShaderFill {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(format!("shader({:p})", Arc::as_ptr(&self.provider)).as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ShaderFill {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            effect: None,
            provider: Arc::new(|_: &RuntimeEffect, _: Area| None),
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UniformsBuilder {
    uniforms: std::collections::HashMap<String, UniformValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UniformValue {
    // Scalar floats
    Float(f32),                 // float
    Float2(f32, f32),           // float2
    Float3(f32, f32, f32),      // float3
    Float4(f32, f32, f32, f32), // float4

    // Scalar integers
    Int(i32),                 // int
    Int2(i32, i32),           // int2
    Int3(i32, i32, i32),      // int3
    Int4(i32, i32, i32, i32), // int4

    // Boolean types
    Bool(bool),                    // bool
    Bool2(bool, bool),             // bool2
    Bool3(bool, bool, bool),       // bool3
    Bool4(bool, bool, bool, bool), // bool4

    // Matrices
    Mat2([f32; 4]),  // float2x2
    Mat3([f32; 9]),  // float3x3
    Mat4([f32; 16]), // float4x4

    // Arrays
    FloatArray(Vec<f32>),
    IntArray(Vec<i32>),
    BoolArray(Vec<bool>),
}

impl UniformsBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a uniform value.
    pub fn set(&mut self, name: impl AsRef<str>, value: UniformValue) {
        self.uniforms.insert(name.as_ref().to_string(), value);
    }

    pub fn build(self, shader: &RuntimeEffect) -> Vec<u8> {
        let mut values = Vec::new();

        for uniform in shader.uniforms() {
            let value = self.uniforms.get(uniform.name()).unwrap_or_else(|| {
                panic!(
                    "Uniform '{}' not found for shader. Available uniforms: {:?}",
                    uniform.name(),
                    self.uniforms.keys().collect::<Vec<_>>()
                )
            });

            Self::push_uniform(value, &mut values);
        }

        values
    }

    #[rustfmt::skip]
    fn push_uniform(value: &UniformValue, values: &mut Vec<u8>) {
        macro_rules! push_f32 {
            ($x:expr) => { values.extend($x.to_le_bytes()) };
        }
        macro_rules! push_i32 {
            ($x:expr) => { values.extend($x.to_le_bytes()) };
        }
        macro_rules! push_bool32 {
            ($x:expr) => { values.extend(($x as u32).to_le_bytes()) };
        }

        match value {
            // --- Scalars & Vectors ---
            UniformValue::Float(f) => push_f32!(*f),
            UniformValue::Float2(x,y) => { push_f32!(*x); push_f32!(*y); },
            UniformValue::Float3(x,y,z) => { push_f32!(*x); push_f32!(*y); push_f32!(*z); },
            UniformValue::Float4(x,y,z,w) => { push_f32!(*x); push_f32!(*y); push_f32!(*z); push_f32!(*w); },

            UniformValue::Int(i) => push_i32!(*i),
            UniformValue::Int2(x,y) => { push_i32!(*x); push_i32!(*y); },
            UniformValue::Int3(x,y,z) => { push_i32!(*x); push_i32!(*y); push_i32!(*z); },
            UniformValue::Int4(x,y,z,w) => { push_i32!(*x); push_i32!(*y); push_i32!(*z); push_i32!(*w); },

            UniformValue::Bool(b) => push_bool32!(*b),
            UniformValue::Bool2(x,y) => { push_bool32!(*x); push_bool32!(*y); },
            UniformValue::Bool3(x,y,z) => { push_bool32!(*x); push_bool32!(*y); push_bool32!(*z); },
            UniformValue::Bool4(x,y,z,w) => { push_bool32!(*x); push_bool32!(*y); push_bool32!(*z); push_bool32!(*w); },

            // Matrices (column-major order)
            UniformValue::Mat2(m) => {
                push_f32!(m[0]); push_f32!(m[2]); push_f32!(m[1]); push_f32!(m[3]);
            },
            UniformValue::Mat3(m) => {
                push_f32!(m[0]); push_f32!(m[3]); push_f32!(m[6]);
                push_f32!(m[1]); push_f32!(m[4]); push_f32!(m[7]);
                push_f32!(m[2]); push_f32!(m[5]); push_f32!(m[8]);
            }
            UniformValue::Mat4(m) => {
                push_f32!(m[0]); push_f32!(m[4]); push_f32!(m[8]);  push_f32!(m[12]);
                push_f32!(m[1]); push_f32!(m[5]); push_f32!(m[9]);  push_f32!(m[13]);
                push_f32!(m[2]); push_f32!(m[6]); push_f32!(m[10]); push_f32!(m[14]);
                push_f32!(m[3]); push_f32!(m[7]); push_f32!(m[11]); push_f32!(m[15]);
            }

            // --- Arrays (recursive) ---
            UniformValue::FloatArray(items) => {
                for f in items {
                    Self::push_uniform(&UniformValue::Float(*f), values);
                }
            }
            UniformValue::IntArray(items) => {
                for i in items {
                    Self::push_uniform(&UniformValue::Int(*i), values);
                }
            }
            UniformValue::BoolArray(items) => {
                for b in items {
                    Self::push_uniform(&UniformValue::Bool(*b), values);
                }
            }
        }
    }
}
