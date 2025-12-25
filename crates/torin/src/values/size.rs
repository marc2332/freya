use std::{
    fmt::Debug,
    hash::Hash,
    sync::Arc,
};

pub use euclid::Rect;

use crate::{
    geometry::Length,
    measure::Phase,
    scaled::Scaled,
};

pub struct SizeFnContext {
    pub parent: f32,
    pub available_parent: f32,
    pub parent_margin: f32,
    pub root: f32,
    pub phase: Phase,
}

#[cfg(feature = "serde")]
pub use serde::*;

#[derive(Clone)]
pub struct SizeFn(Arc<dyn Fn(SizeFnContext) -> Option<f32> + Sync + Send>, u64);

#[cfg(feature = "serde")]
impl Serialize for SizeFn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Fn")
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for SizeFn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FnVisitor;
        use serde::de::Visitor;

        impl Visitor<'_> for FnVisitor {
            type Value = SizeFn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("\"Fn\"")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v == "Fn" {
                    Ok(SizeFn(Arc::new(|_ctx| None), 0))
                } else {
                    Err(E::custom(format!("expected \"Fn\", got {v}")))
                }
            }
        }

        deserializer.deserialize_str(FnVisitor)
    }
}

impl SizeFn {
    pub fn new(func: impl Fn(SizeFnContext) -> Option<f32> + 'static + Sync + Send) -> Self {
        Self(Arc::new(func), 0)
    }

    pub fn new_data<D: Hash>(
        func: impl Fn(SizeFnContext) -> Option<f32> + 'static + Sync + Send,
        data: &D,
    ) -> Self {
        use std::hash::Hasher;
        let mut hasher = std::hash::DefaultHasher::default();
        data.hash(&mut hasher);
        Self(Arc::new(func), hasher.finish())
    }

    pub fn call(&self, context: SizeFnContext) -> Option<f32> {
        (self.0)(context)
    }
}

impl Debug for SizeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SizeFn")
    }
}

impl PartialEq for SizeFn {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Clone, Debug)]
pub enum Size {
    Inner,
    Fill,
    FillMinimum,
    Percentage(Length),
    Pixels(Length),
    RootPercentage(Length),
    Fn(Box<SizeFn>),
    Flex(Length),
}

impl Default for Size {
    fn default() -> Self {
        Self::Inner
    }
}

impl Size {
    pub(crate) fn flex_grow(&self) -> Option<Length> {
        match self {
            Self::Flex(f) => Some(*f),
            _ => None,
        }
    }

    pub(crate) fn is_flex(&self) -> bool {
        matches!(self, Self::Flex(_))
    }

    pub(crate) fn inner_sized(&self) -> bool {
        matches!(self, Self::Inner | Self::FillMinimum)
    }

    pub fn pretty(&self) -> String {
        match self {
            Self::Inner => "auto".to_string(),
            Self::Pixels(s) => format!("{}", s.get()),
            Self::Fn(_) => "Fn".to_string(),
            Self::Percentage(p) => format!("{}%", p.get()),
            Self::Fill => "fill".to_string(),
            Self::FillMinimum => "fill-min".to_string(),
            Self::RootPercentage(p) => format!("{}% of root", p.get()),
            Self::Flex(f) => format!("flex({})", f.get()),
        }
    }

    pub(crate) fn eval(
        &self,
        parent: f32,
        available_parent: f32,
        parent_margin: f32,
        root: f32,
        phase: Phase,
    ) -> Option<f32> {
        match self {
            Self::Pixels(px) => Some(px.get() + parent_margin),
            Self::Percentage(per) => Some(parent / 100.0 * per.get()),
            Self::Fill => Some(available_parent),
            Self::RootPercentage(per) => Some(root / 100.0 * per.get()),
            Self::Flex(_) | Self::FillMinimum if phase == Phase::Final => Some(available_parent),
            Self::Fn(f) => f.call(SizeFnContext {
                parent,
                available_parent,
                parent_margin,
                root,
                phase,
            }),
            _ => None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn min_max(
        &self,
        value: f32,
        parent_value: f32,
        available_parent_value: f32,
        single_margin: f32,
        margin: f32,
        minimum: &Self,
        maximum: &Self,
        root_value: f32,
        phase: Phase,
    ) -> f32 {
        let value = self
            .eval(
                parent_value,
                available_parent_value,
                margin,
                root_value,
                phase,
            )
            .unwrap_or(value + margin);

        let minimum_value = minimum
            .eval(
                parent_value,
                available_parent_value,
                margin,
                root_value,
                phase,
            )
            .map(|v| v + single_margin);
        let maximum_value = maximum.eval(
            parent_value,
            available_parent_value,
            margin,
            root_value,
            phase,
        );

        let mut final_value = value;

        if let Some(minimum_value) = minimum_value
            && minimum_value > final_value
        {
            final_value = minimum_value;
        }

        if let Some(maximum_value) = maximum_value
            && final_value > maximum_value
        {
            final_value = maximum_value;
        }

        final_value
    }

    pub(crate) fn most_fitting_size<'a>(&self, size: &'a f32, available_size: &'a f32) -> &'a f32 {
        match self {
            Self::Inner => available_size,
            _ => size,
        }
    }
}

impl Scaled for Size {
    fn scale(&mut self, scale_factor: f32) {
        if let Self::Pixels(s) = self {
            *s *= scale_factor;
        }
    }
}
