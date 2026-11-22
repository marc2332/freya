use std::time::Duration;

#[doc(hidden)]
pub use ::paste::paste;
use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    size::Size,
};

use crate::{
    corner_radius::CornerRadiusRole,
    theming::component_themes::{
        ColorsSheet,
        Theme,
    },
    typography::TextRole,
};

#[doc(hidden)]
#[macro_export]
macro_rules! theme_setter_param {
    (f32) => { f32 };
    ($field_ty:ty) => { impl ::core::convert::Into<$field_ty> };
}

#[doc(hidden)]
#[macro_export]
macro_rules! theme_resolved_type {
    (CornerRadiusSize) => {
        $crate::corner_radius::ResolvedCornerRadius
    };
    ($field_ty:ty) => {
        $field_ty
    };
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
            #[doc = "Theme overrides for a single component instance."]
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
                    pub $field_name: $crate::theme_resolved_type!($field_ty),
                )*)?
            }

            impl [<$name ThemePreference>] {
                pub fn apply_optional(&mut self, optional: &[<$name ThemePartial>]) {
                    $($(
                        if let Some($field_name) = &optional.$field_name {
                            self.$field_name = $field_name.clone();
                        }
                    )*)?
                }

                /// Resolve color references using a color sheet.
                /// Panics for typography references and named or relative radii. Use [`Self::resolve_with_theme`] for those.
                pub fn resolve(&self, colors_sheet: &$crate::theming::component_themes::ColorsSheet) -> [<$name Theme>] {
                    use $crate::theming::macros::ResolvablePreference;
                    [<$name Theme>] {
                        $($(
                            $field_name: self.$field_name.resolve(colors_sheet),
                        )*)?
                    }
                }

                /// Resolve references and relative radii using the supplied theme.
                pub fn resolve_with_theme(&self, theme: &$crate::theming::component_themes::Theme) -> [<$name Theme>] {
                    use $crate::theming::macros::ResolvablePreference;
                    [<$name Theme>] {
                        $($(
                            $field_name: self.$field_name.resolve_with_theme(theme),
                        )*)?
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
        let default_theme: $theme_type;
        let requested_theme = match theme.get::<$theme_type>($theme_key) {
            Some(requested_theme) => requested_theme,
            None => {
                default_theme = ($default)();
                &default_theme
            }
        };

        if let Some(theme_override) = $theme_prop {
            let mut requested_theme = requested_theme.clone();
            requested_theme.apply_optional(&theme_override);
            requested_theme.resolve_with_theme(&theme)
        } else {
            requested_theme.resolve_with_theme(&theme)
        }
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
    Reference(&'static str),
    /// A role for an `f32` font-size field. Resolve it with a full theme.
    Typography(TextRole),
    /// A named size for a `CornerRadiusSize` field. Resolve it with a full theme.
    CornerRadius(CornerRadiusRole),
}

impl<T> From<T> for Preference<T> {
    fn from(value: T) -> Self {
        Preference::Specific(value)
    }
}

pub trait ResolvablePreference<T: Clone> {
    fn resolve(&self, colors_sheet: &ColorsSheet) -> T;

    fn resolve_with_theme(&self, theme: &Theme) -> T {
        self.resolve(&theme.colors)
    }
}

impl ResolvablePreference<Color> for Preference<Color> {
    fn resolve(&self, colors_sheet: &ColorsSheet) -> Color {
        match self {
            Self::Reference(reference) => match *reference {
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

                _ => colors_sheet.primary,
            },

            Self::Specific(value) => *value,
            Self::Typography(_) => panic!("Typography references require a font-size field."),
            Self::CornerRadius(_) => panic!("Radius references require a corner-radius field."),
        }
    }
}

impl ResolvablePreference<Size> for Preference<Size> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> Size {
        match self {
            Self::Specific(value) => value.clone(),
            _ => panic!("Size fields do not support references."),
        }
    }
}

impl ResolvablePreference<Gaps> for Preference<Gaps> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> Gaps {
        match self {
            Self::Specific(value) => *value,
            _ => panic!("Gaps fields do not support references."),
        }
    }
}

impl ResolvablePreference<CornerRadius> for Preference<CornerRadius> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> CornerRadius {
        match self {
            Self::Specific(value) => *value,
            _ => panic!("Corner-radius fields do not support references."),
        }
    }
}

impl ResolvablePreference<f32> for Preference<f32> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> f32 {
        match self {
            Self::Reference(_) => panic!("Only Colors support color references."),
            Self::Typography(_) => panic!("Typography references require a full theme."),
            Self::CornerRadius(_) => panic!("Radius references require a corner-radius field."),
            Self::Specific(value) => *value,
        }
    }

    fn resolve_with_theme(&self, theme: &Theme) -> f32 {
        match self {
            Self::Typography(role) => theme.font_size(*role),
            _ => self.resolve(&theme.colors),
        }
    }
}

impl ResolvablePreference<Duration> for Preference<Duration> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> Duration {
        match self {
            Self::Specific(value) => *value,
            _ => panic!("Duration fields do not support references."),
        }
    }
}
