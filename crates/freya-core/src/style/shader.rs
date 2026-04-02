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
        Arc::ptr_eq(&self.provider, &other.provider)
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
