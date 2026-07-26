use std::{
    borrow::Cow,
    time::Duration,
};

#[doc(hidden)]
pub use ::paste::paste;
use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    size::Size,
};

use crate::theming::component_themes::{
    ColorsSheet,
    Palette,
};

/// Setter parameter type for a theme field, `f32` fields take a plain `f32`.
#[doc(hidden)]
#[macro_export]
macro_rules! theme_setter_param {
    (f32) => { f32 };
    ($field_ty:ty) => { impl ::core::convert::Into<$field_ty> };
}

#[macro_export]
macro_rules! define_theme {
    (NOTHING=) => {};

    (
        @ext_impls
        [ $head_ty:ident $($rest_ty:ident)* ]
        [ $head_field:ident $($rest_field:ident)* ]
        $name:ident ;
        $( $(#[$field_attrs:meta])* $field_name:ident : $field_ty:tt , )*
    ) => {
        $crate::theming::macros::paste! {
            impl [<$name ThemePartialExt>] for $head_ty {
                $(
                    $(#[$field_attrs])*
                    fn $field_name(mut self, $field_name: $crate::theme_setter_param!($field_ty)) -> Self {
                        self.$head_field = Some(self.$head_field.unwrap_or_default().$field_name($field_name));
                        self
                    }
                )*
            }
        }
        $crate::define_theme! {
            @ext_impls
            [ $($rest_ty)* ]
            [ $($rest_field)* ]
            $name ;
            $( $(#[$field_attrs])* $field_name : $field_ty , )*
        }
    };

    (
        @ext_impls
        [] []
        $name:ident ;
        $( $(#[$field_attrs:meta])* $field_name:ident : $field_ty:tt , )*
    ) => {};

    // Emits the structs and their inherent impls.
    (
        @types
        $(#[$attrs:meta])*
        $(%[component$($component_attr_control:tt)?])?
        pub $name:ident {
            $(
                %[fields$($cows_attr_control:tt)?]
                $(
                    $(#[$field_attrs:meta])*
                    $field_name:ident: $field_ty:tt,
                )*
            )?
    }) => {
        $crate::define_theme!(NOTHING=$($($component_attr_control)?)?);
        $crate::theming::macros::paste! {
            #[derive(Default, Clone, Debug, PartialEq)]
            #[doc = "You can use this to change a theme for only one component, with the `theme` property."]
            $(#[$attrs])*
            pub struct [<$name ThemePartial>] {
                $($(
                    $(#[$field_attrs])*
                    pub $field_name: Option<$crate::theming::macros::Preference<$field_ty>>,
                )*)?
            }

            #[derive(Clone, Debug, PartialEq)]
            $(#[doc = "Theming properties for the `" $name "` component."] $($component_attr_control)?)?
            $(#[$attrs])*
            pub struct [<$name ThemePreference>] {
                $($(
                    $(#[$field_attrs])*
                    pub $field_name: $crate::theming::macros::Preference<$field_ty>,
                )*)?
            }

            #[derive(Clone, Debug, PartialEq)]
            $(#[doc = "Theming properties for the `" $name "` component."] $($component_attr_control)?)?
            $(#[$attrs])*
            pub struct [<$name Theme>] {
                $($(
                    $(#[$field_attrs])*
                    pub $field_name: $field_ty,
                )*)?
            }

            impl [<$name ThemePreference>] {
                #[doc = "Checks each field in `optional` and if it's `Some`, it overwrites the corresponding `self` field."]
                pub fn apply_optional(&mut self, optional: &[<$name ThemePartial>]) {

                    $($(
                        if let Some($field_name) = &optional.$field_name {
                            self.$field_name = $field_name.clone();
                        }
                    )*)?
                }

                #[doc = "Resolves every field against `palette`, turning references into concrete values."]
                pub fn resolve(&mut self, palette: &dyn $crate::theming::component_themes::Palette) -> [<$name Theme>] {
                    use $crate::theming::macros::ResolvablePreference;
                    [<$name Theme>] {
                        $(
                            $(
                                $field_name: self.$field_name.resolve(palette),
                            )*
                        )?
                    }
                }
            }

            impl [<$name ThemePartial>] {
                pub fn new() -> Self {
                    Self::default()
                }

                $($(
                    $(#[$field_attrs])*
                    pub fn $field_name(mut self, $field_name: $crate::theme_setter_param!($field_ty)) -> Self {
                        self.$field_name = Some($crate::theming::macros::Preference::Specific($field_name.into()));
                        self
                    }
                )*)?
            }
        }
    };

    (
        $(#[$attrs:meta])*
        $(for = $for_ty:ident ; theme_field = $theme_field:ident ;)+
        $(%[component$($component_attr_control:tt)?])?
        pub $name:ident {
            $(
                %[fields$($cows_attr_control:tt)?]
                $(
                    $(#[$field_attrs:meta])*
                    $field_name:ident: $field_ty:tt,
                )*
            )?
    }) => {
        $crate::define_theme! {
            @types
            $(#[$attrs])*
            $(%[component$($component_attr_control)?])?
            pub $name {
                $(
                    %[fields$($cows_attr_control)?]
                    $(
                        $(#[$field_attrs])*
                        $field_name: $field_ty,
                    )*
                )?
            }
        }
        $crate::theming::macros::paste! {
            pub trait [<$name ThemePartialExt>] {
                $($(
                    $(#[$field_attrs])*
                    fn $field_name(self, $field_name: $crate::theme_setter_param!($field_ty)) -> Self;
                )*)?
            }
        }
        $crate::define_theme! {
            @ext_impls
            [ $($for_ty)+ ]
            [ $($theme_field)+ ]
            $name ;
            $($( $(#[$field_attrs])* $field_name : $field_ty , )*)?
        }
    };

    // Skips the auto-generated `ThemePartialExt` trait.
    (
        $(#[$attrs:meta])*
        %[no_ext]
        $(%[component$($component_attr_control:tt)?])?
        pub $name:ident {
            $(
                %[fields$($cows_attr_control:tt)?]
                $(
                    $(#[$field_attrs:meta])*
                    $field_name:ident: $field_ty:tt,
                )*
            )?
    }) => {
        $crate::define_theme! {
            @types
            $(#[$attrs])*
            $(%[component$($component_attr_control)?])?
            pub $name {
                $(
                    %[fields$($cows_attr_control)?]
                    $(
                        $(#[$field_attrs])*
                        $field_name: $field_ty,
                    )*
                )?
            }
        }
    };

    (
        $(#[$attrs:meta])*
        $(%[component$($component_attr_control:tt)?])?
        pub $name:ident {
            $(
                %[fields$($cows_attr_control:tt)?]
                $(
                    $(#[$field_attrs:meta])*
                    $field_name:ident: $field_ty:tt,
                )*
            )?
    }) => {
        $crate::define_theme! {
            for = $name; theme_field = theme;
            $(%[component$($component_attr_control)?])?
            pub $name {
                $(
                    %[fields$($cows_attr_control)?]
                    $(
                        $(#[$field_attrs])*
                        $field_name: $field_ty,
                    )*
                )?
            }
        }
    };
}

/// Like [`get_theme!`](crate::get_theme), but falls back to a `default` callback
/// instead of panicking when the key is not registered.
#[macro_export]
macro_rules! get_theme_or_default {
    ($theme_prop:expr, $theme_type:ty, $theme_key:expr, $default:expr) => {{
        let theme = $crate::theming::hooks::get_theme_or_default();
        let theme = theme.read();
        let mut requested_theme = theme
            .get::<$theme_type>($theme_key)
            .cloned()
            .unwrap_or_else($default);

        if let Some(theme_override) = $theme_prop {
            requested_theme.apply_optional(&theme_override);
        }

        requested_theme.resolve(&*theme.palette)
    }};
}

#[macro_export]
macro_rules! get_theme {
    ($theme_prop:expr, $theme_type:ty, $theme_key:expr) => {
        $crate::get_theme_or_default!($theme_prop, $theme_type, $theme_key, || {
            ::core::panic!(concat!("Theme key not found: ", $theme_key))
        })
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Preference<T> {
    Specific(T),
    /// A named slot, resolved against the theme's [`Palette`]. Owned so a palette slot named at
    /// runtime (a theme loaded from disk) is expressible, not just a `&'static str` literal.
    Reference(Cow<'static, str>),
}

impl<T> Preference<T> {
    /// A reference to a named palette slot.
    pub fn reference(name: impl Into<Cow<'static, str>>) -> Self {
        Self::Reference(name.into())
    }
}

impl<T> From<T> for Preference<T> {
    fn from(value: T) -> Self {
        Preference::Specific(value)
    }
}

/// Resolve one of the core [`ColorsSheet`] slot names, or `None` when the name is not a core
/// slot (and so belongs to the palette's own extended namespace).
fn core_slot(colors_sheet: &ColorsSheet, name: &str) -> Option<Color> {
    Some(match name {
        // Brand & Accent
        "primary" => colors_sheet.primary,
        "secondary" => colors_sheet.secondary,
        "tertiary" => colors_sheet.tertiary,

        // Status
        "success" => colors_sheet.success,
        "warning" => colors_sheet.warning,
        "error" => colors_sheet.error,
        "info" => colors_sheet.info,

        // Surfaces
        "background" => colors_sheet.background,
        "surface_primary" => colors_sheet.surface_primary,
        "surface_secondary" => colors_sheet.surface_secondary,
        "surface_tertiary" => colors_sheet.surface_tertiary,
        "surface_inverse" => colors_sheet.surface_inverse,
        "surface_inverse_secondary" => colors_sheet.surface_inverse_secondary,
        "surface_inverse_tertiary" => colors_sheet.surface_inverse_tertiary,

        // Borders
        "border" => colors_sheet.border,
        "border_focus" => colors_sheet.border_focus,
        "border_disabled" => colors_sheet.border_disabled,

        // Text
        "text_primary" => colors_sheet.text_primary,
        "text_secondary" => colors_sheet.text_secondary,
        "text_placeholder" => colors_sheet.text_placeholder,
        "text_inverse" => colors_sheet.text_inverse,
        "text_highlight" => colors_sheet.text_highlight,

        // States
        "focus" => colors_sheet.focus,
        "active" => colors_sheet.active,
        "disabled" => colors_sheet.disabled,

        // Utility
        "overlay" => colors_sheet.overlay,
        "shadow" => colors_sheet.shadow,

        _ => return None,
    })
}

pub trait ResolvablePreference<T: Clone> {
    fn resolve(&self, palette: &dyn Palette) -> T;
}

impl ResolvablePreference<Color> for Preference<Color> {
    /// Core slots win over the palette's extended namespace, so an app palette can never break a
    /// built-in component by shadowing a slot it depends on. An unresolvable name falls back to
    /// `primary`.
    fn resolve(&self, palette: &dyn Palette) -> Color {
        match self {
            Self::Reference(reference) => {
                let colors_sheet = palette.sheet();
                core_slot(colors_sheet, reference)
                    .or_else(|| palette.color(reference))
                    .unwrap_or(colors_sheet.primary)
            }

            Self::Specific(value) => *value,
        }
    }
}

impl ResolvablePreference<Size> for Preference<Size> {
    fn resolve(&self, _palette: &dyn Palette) -> Size {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => value.clone(),
        }
    }
}

impl ResolvablePreference<Gaps> for Preference<Gaps> {
    fn resolve(&self, _palette: &dyn Palette) -> Gaps {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => *value,
        }
    }
}

impl ResolvablePreference<CornerRadius> for Preference<CornerRadius> {
    fn resolve(&self, _palette: &dyn Palette) -> CornerRadius {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => *value,
        }
    }
}

impl ResolvablePreference<f32> for Preference<f32> {
    fn resolve(&self, _palette: &dyn Palette) -> f32 {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => *value,
        }
    }
}

impl ResolvablePreference<Duration> for Preference<Duration> {
    fn resolve(&self, _palette: &dyn Palette) -> Duration {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => *value,
        }
    }
}

impl ResolvablePreference<String> for Preference<String> {
    fn resolve(&self, _palette: &dyn Palette) -> String {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => value.clone(),
        }
    }
}

impl ResolvablePreference<i32> for Preference<i32> {
    fn resolve(&self, _palette: &dyn Palette) -> i32 {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => *value,
        }
    }
}
