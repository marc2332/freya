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
    /// Sizes the element based on its content. This is the default.
    ///
    /// Can also be created with [`Size::auto`].
    ///
    /// ```
    /// # use torin::prelude::*;
    /// let size = Size::auto();
    /// ```
    Inner,

    /// Expands to fill all the available space from its parent.
    ///
    /// Can also be created with [`Size::fill`].
    ///
    /// ```
    /// # use torin::prelude::*;
    /// let size = Size::fill();
    /// ```
    Fill,

    /// Expand to the biggest sibling when using [`Content::fit`](crate::content::Content::fit).
    ///
    /// Can also be created with [`Size::fill_minimum`].
    ///
    /// ```
    /// # use torin::prelude::*;
    /// let size = Size::fill_minimum();
    /// ```
    FillMinimum,

    /// Sizes as a percentage relative to the parent's size.
    ///
    /// Can also be created with [`Size::percent`].
    ///
    /// ```
    /// # use torin::prelude::*;
    /// let size = Size::percent(50.0);
    /// ```
    Percentage(Length),

    /// Fixed size in pixels.
    ///
    /// Can also be created with [`Size::px`].
    ///
    /// ```
    /// # use torin::prelude::*;
    /// let size = Size::px(200.0);
    /// ```
    Pixels(Length),

    /// Sizes as a percentage relative to the root (window) size.
    ///
    /// Can also be created with [`Size::window_percent`].
    ///
    /// ```
    /// # use torin::prelude::*;
    /// let size = Size::window_percent(80.0);
    /// ```
    RootPercentage(Length),

    /// Dynamic size computed by a closure at layout time.
    ///
    /// Can also be created with [`Size::func`] or [`Size::func_data`].
    Fn(Box<SizeFn>),

    /// Flex grow factor, fills the available space proportionally in the final layout phase.
    ///
    /// Can also be created with [`Size::flex`].
    ///
    /// ```
    /// # use torin::prelude::*;
    /// let size = Size::flex(1.0);
    /// ```
    Flex(Length),
}

impl Default for Size {
    fn default() -> Self {
        Self::Inner
    }
}

impl Size {
    /// Use an [`Inner`](Size::Inner) size.
    pub fn auto() -> Size {
        Size::Inner
    }

    /// Use a [`Fill`](Size::Fill) size.
    pub fn fill() -> Size {
        Size::Fill
    }

    /// Use a [`FillMinimum`](Size::FillMinimum) size.
    pub fn fill_minimum() -> Size {
        Size::FillMinimum
    }

    /// Use a [`Percentage`](Size::Percentage) size.
    pub fn percent(percent: impl Into<f32>) -> Size {
        Size::Percentage(Length::new(percent.into()))
    }

    /// Use a [`Pixels`](Size::Pixels) size.
    pub fn px(px: impl Into<f32>) -> Size {
        Size::Pixels(Length::new(px.into()))
    }

    /// Use a [`RootPercentage`](Size::RootPercentage) size.
    pub fn window_percent(percent: impl Into<f32>) -> Size {
        Size::RootPercentage(Length::new(percent.into()))
    }

    /// Use a [`Flex`](Size::Flex) size.
    pub fn flex(flex: impl Into<f32>) -> Size {
        Size::Flex(Length::new(flex.into()))
    }

    /// Use a dynamic [`Fn`](Size::Fn) size computed by the given closure.
    pub fn func(func: impl Fn(SizeFnContext) -> Option<f32> + 'static + Sync + Send) -> Size {
        Self::Fn(Box::new(SizeFn::new(func)))
    }

    /// Use a dynamic [`Fn`](Size::Fn) size with hashable data for equality checks.
    pub fn func_data<D: Hash>(
        func: impl Fn(SizeFnContext) -> Option<f32> + 'static + Sync + Send,
        data: &D,
    ) -> Size {
        Self::Fn(Box::new(SizeFn::new_data(func, data)))
    }

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
