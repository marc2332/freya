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
struct SharedRuntimeEffect(RuntimeEffect);

// SAFETY: `RuntimeEffect` is immutable.
unsafe impl Send for SharedRuntimeEffect {}
unsafe impl Sync for SharedRuntimeEffect {}

#[derive(Clone)]
pub struct ShaderFill {
    sksl: Arc<str>,
    effect: Arc<SharedRuntimeEffect>,
    provider: Arc<dyn ShaderProvider>,
}

impl ShaderFill {
    pub fn new<F>(sksl: impl Into<Arc<str>>, effect: RuntimeEffect, provider: F) -> Self
    where
        F: Fn(&RuntimeEffect, Area) -> Option<Shader> + Send + Sync + 'static,
    {
        Self::from_provider(sksl, effect, provider)
    }

    pub fn from_provider<S>(sksl: impl Into<Arc<str>>, effect: RuntimeEffect, provider: S) -> Self
    where
        S: ShaderProvider + 'static,
    {
        Self {
            sksl: sksl.into(),
            effect: Arc::new(SharedRuntimeEffect(effect)),
            provider: Arc::new(provider),
        }
    }

    /// Prepare the shader for use by providing the necessary uniforms.
    /// Returns [None] if the provider could not produce a [Shader], in which case the renderer will fallback to no fill.
    pub fn prepare_shader(&self, bounds: Area) -> Option<Shader> {
        self.provider.prepare_shader(&self.effect.0, bounds)
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
            .field("sksl", &self.sksl)
            .finish()
    }
}

impl PartialEq for ShaderFill {
    fn eq(&self, other: &Self) -> bool {
        *self.sksl == *other.sksl
            && Arc::ptr_eq(&self.effect, &other.effect)
            && Arc::ptr_eq(&self.provider, &other.provider)
    }
}

impl std::hash::Hash for ShaderFill {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Mirrors PartialEq: SKSL by bytes, effect/provider by Arc pointer.
        // The provider cast strips the vtable to match `Arc::ptr_eq` semantics.
        (*self.sksl).hash(state);
        Arc::as_ptr(&self.effect).hash(state);
        Arc::as_ptr(&self.provider).cast::<()>().hash(state);
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ShaderFill {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.sksl)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ShaderFill {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let sksl = String::deserialize(deserializer)?;
        let effect =
            RuntimeEffect::make_for_shader(&sksl, None).map_err(serde::de::Error::custom)?;

        Ok(Self {
            sksl: sksl.into(),
            effect: Arc::new(SharedRuntimeEffect(effect)),
            provider: Arc::new(|_: &RuntimeEffect, _: Area| None),
        })
    }
}
