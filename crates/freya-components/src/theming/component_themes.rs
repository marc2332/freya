use std::{
    any::{
        Any,
        type_name,
    },
    fmt,
};

use freya_core::{
    integration::FxHashMap,
    prelude::Color,
};

use crate::theming::themes::{
    ColorRole,
    light_theme,
};

#[derive(PartialEq)]
pub struct Theme {
    pub name: &'static str,
    pub variables: AnyMap,
    components: AnyMap,
}

impl Theme {
    pub fn new(name: &'static str, variables: AnyMap) -> Self {
        Self {
            name,
            variables,
            components: AnyMap::default(),
        }
    }

    /// Get a component theme by key.
    pub fn get_theme<T: 'static>(&self, key: &str) -> Option<&T> {
        self.components.try_get(key)
    }

    /// Set a component theme by key.
    pub fn set_theme<T: PartialEq + 'static>(&mut self, key: &'static str, value: T) {
        self.components.set(key, value);
    }

    /// Get a variable or panic if it is missing or has a different type.
    #[track_caller]
    pub fn get_var<T: 'static>(&self, key: &str) -> &T {
        self.variables.get(key)
    }

    /// Get a palette color or panic if the role is missing or has a different type.
    #[track_caller]
    pub fn color(&self, role: ColorRole) -> Color {
        *self.get_var(role.key())
    }

    /// Set or replace a variable, including its type.
    pub fn set_var<T: PartialEq + 'static>(&mut self, key: &'static str, value: T) {
        self.variables.set(key, value);
    }
}

impl fmt::Debug for Theme {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Theme")
            .field("name", &self.name)
            .field("variables", &self.variables)
            .field("components", &self.components)
            .finish()
    }
}

impl Default for Theme {
    fn default() -> Self {
        light_theme()
    }
}

/// A map of named values of different types.
#[derive(Default)]
pub struct AnyMap(FxHashMap<&'static str, Box<dyn AnyValue>>);

impl AnyMap {
    /// Set or replace a variable, including its type.
    pub fn set<T: PartialEq + 'static>(&mut self, key: &'static str, value: T) {
        self.0.insert(key, Box::new(value));
    }

    /// Look up a variable. Returns `None` if the key is missing or the type differs.
    pub fn try_get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.0.get(key)?.as_any().downcast_ref()
    }

    /// Get a variable or panic if it is missing or has a different type.
    #[track_caller]
    pub fn get<T: 'static>(&self, key: &str) -> &T {
        self.try_get(key).unwrap_or_else(|| {
            panic!(
                "Theme variable `{key}` is missing or is not of type `{}`",
                type_name::<T>()
            )
        })
    }
}

impl PartialEq for AnyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
            && self.0.iter().all(|(key, value)| {
                other
                    .0
                    .get(key)
                    .is_some_and(|other| value.equals(other.as_any()))
            })
    }
}

impl fmt::Debug for AnyMap {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_set().entries(self.0.keys()).finish()
    }
}

trait AnyValue: 'static {
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, other: &dyn Any) -> bool;
}

impl<T: PartialEq + 'static> AnyValue for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>() == Some(self)
    }
}
