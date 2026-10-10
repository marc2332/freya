use std::rc::Rc;

use freya_engine::prelude::*;
use torin::prelude::Area;

pub trait ShaderProvider {
    fn prepare_shader(&self, effect: &RuntimeEffect, bounds: Area) -> Option<Shader>;
}

impl<F> ShaderProvider for F
where
    F: Fn(&RuntimeEffect, Area) -> Option<Shader>,
{
    fn prepare_shader(&self, effect: &RuntimeEffect, bounds: Area) -> Option<Shader> {
        self(effect, bounds)
    }
}

/// A custom paint source backed by an SkSL shader.
///
/// Build it with [`ShaderFill::new`], passing the SkSL source, a compiled
/// [`RuntimeEffect`] and a provider closure that supplies the shader's uniforms
/// for a given bounds. Use it as a [`Fill`](crate::style::fill::Fill) for
/// backgrounds or text.
#[derive(Clone)]
pub struct ShaderFill {
    sksl: Rc<str>,
    effect: Rc<RuntimeEffect>,
    provider: Rc<dyn ShaderProvider>,
}

impl ShaderFill {
    pub fn new<F>(sksl: impl Into<Rc<str>>, effect: RuntimeEffect, provider: F) -> Self
    where
        F: Fn(&RuntimeEffect, Area) -> Option<Shader> + 'static,
    {
        Self::from_provider(sksl, effect, provider)
    }

    pub fn from_provider<S>(sksl: impl Into<Rc<str>>, effect: RuntimeEffect, provider: S) -> Self
    where
        S: ShaderProvider + 'static,
    {
        Self {
            sksl: sksl.into(),
            effect: Rc::new(effect),
            provider: Rc::new(provider),
        }
    }

    /// Prepare the shader for use by providing the necessary uniforms.
    /// Returns [None] if the provider could not produce a [Shader], in which case the renderer will fallback to no fill.
    pub fn prepare_shader(&self, bounds: Area) -> Option<Shader> {
        self.provider.prepare_shader(&self.effect, bounds)
    }
}

impl std::fmt::Display for ShaderFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shader({:p})", Rc::as_ptr(&self.provider))
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
            && Rc::ptr_eq(&self.effect, &other.effect)
            && Rc::ptr_eq(&self.provider, &other.provider)
    }
}

impl std::hash::Hash for ShaderFill {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self.sksl).hash(state);
        Rc::as_ptr(&self.effect).hash(state);
        Rc::as_ptr(&self.provider).hash(state);
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
            effect: Rc::new(effect),
            provider: Rc::new(|_: &RuntimeEffect, _: Area| None),
        })
    }
}
