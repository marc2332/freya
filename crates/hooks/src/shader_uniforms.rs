use freya_engine::prelude::RuntimeEffect;
use std::collections::HashMap;

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
