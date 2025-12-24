#[doc(hidden)]
pub use ::paste::paste;
use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    size::Size,
};

use crate::theming::component_themes::ColorsSheet;

#[macro_export]
macro_rules! define_theme {
    (NOTHING=) => {};

    (
        $(#[$attrs:meta])*
        for = $for_ty:ident ;
        theme_field = $theme_field:ident ;
        $(%[component$($component_attr_control:tt)?])?
        $vis:vis $name:ident $(<$lifetime:lifetime>)? {
            $(
                %[fields$($cows_attr_control:tt)?]
                $(
                    $(#[$field_attrs:meta])*
                    $field_name:ident: $field_ty:ty,
                )*
            )?
    }) => {
        $crate::define_theme!(NOTHING=$($($component_attr_control)?)?);
        $crate::theming::macros::paste! {
            #[derive(Default, Clone, Debug, PartialEq)]
            #[doc = "You can use this to change a theme for only one component, with the `theme` property."]
            $(#[$attrs])*
            $vis struct [<$name ThemePartial>] $(<$lifetime>)? {
                $($(
                    $(#[$field_attrs])*
                    pub $field_name: Option<$crate::theming::macros::Preference<$field_ty>>,
                )*)?
            }

            #[derive(Clone, Debug, PartialEq)]
            $(#[doc = "Theming properties for the `" $name "` component."] $($component_attr_control)?)?
            $(#[$attrs])*
            $vis struct [<$name ThemePreference>] $(<$lifetime>)? {
                $($(
                    $(#[$field_attrs])*
                    pub $field_name: $crate::theming::macros::Preference<$field_ty>,
                )*)?
            }

            #[derive(Clone, Debug, PartialEq)]
            $(#[doc = "Theming properties for the `" $name "` component."] $($component_attr_control)?)?
            $(#[$attrs])*
            $vis struct [<$name Theme>] $(<$lifetime>)? {
                $($(
                    $(#[$field_attrs])*
                    pub $field_name: $field_ty,
                )*)?
            }

            impl $(<$lifetime>)? [<$name ThemePreference>] $(<$lifetime>)? {
                #[doc = "Checks each field in `optional` and if it's `Some`, it overwrites the corresponding `self` field."]
                pub fn apply_optional(&mut self, optional: & $($lifetime)? [<$name ThemePartial>]) {

                    $($(
                        if let Some($field_name) = &optional.$field_name {
                            self.$field_name = $field_name.clone();
                        }
                    )*)?
                }

                #[doc = "Checks each field in `optional` and if it's `Some`, it overwrites the corresponding `self` field."]
                pub fn resolve(&mut self, colors_sheet: &$crate::theming::component_themes::ColorsSheet) -> [<$name Theme>] {
                    use $crate::theming::macros::ResolvablePreference;
                    [<$name Theme>] {
                        $(
                            $(
                                $field_name: self.$field_name.resolve(colors_sheet),
                            )*
                        )?
                    }
                }
            }

            impl $(<$lifetime>)? [<$name ThemePartial>] $(<$lifetime>)? {
                pub fn new() -> Self {
                    Self::default()
                }

                $($(
                    $(#[$field_attrs])*
                    pub fn $field_name(mut self, $field_name: impl Into<$field_ty>) -> Self {
                        self.$field_name = Some($crate::theming::macros::Preference::Specific($field_name.into()));
                        self
                    }
                )*)?
            }

            pub trait [<$name ThemePartialExt>] {
                $($(
                    $(#[$field_attrs])*
                    fn $field_name(self, $field_name: impl Into<$field_ty>) -> Self;
                )*)?
            }

            impl $(<$lifetime>)? [<$name ThemePartialExt>] for $for_ty $(<$lifetime>)? {
                $($(
                    $(#[$field_attrs])*
                    fn $field_name(mut self, $field_name: impl Into<$field_ty>) -> Self {
                        self.$theme_field = Some(self.$theme_field.unwrap_or_default().$field_name($field_name));
                        self
                    }
                )*)?
            }
        }
    };

    (
        $(#[$attrs:meta])*
        $(%[component$($component_attr_control:tt)?])?
        $vis:vis $name:ident $(<$lifetime:lifetime>)? {
            $(
                %[fields$($cows_attr_control:tt)?]
                $(
                    $(#[$field_attrs:meta])*
                    $field_name:ident: $field_ty:ty,
                )*
            )?
    }) => {
        $crate::define_theme!(NOTHING=$($($component_attr_control)?)?);
        $crate::theming::macros::paste! {
            #[derive(Default, Clone, Debug, PartialEq)]
            #[doc = "You can use this to change a theme for only one component, with the `theme` property."]
            $(#[$attrs])*
            $vis struct [<$name ThemePartial>] $(<$lifetime>)? {
                $($(
                    $(#[$field_attrs])*
                    pub $field_name: Option<$crate::theming::macros::Preference<$field_ty>>,
                )*)?
            }

            #[derive(Clone, Debug, PartialEq)]
            $(#[doc = "Theming properties for the `" $name "` component."] $($component_attr_control)?)?
            $(#[$attrs])*
            $vis struct [<$name ThemePreference>] $(<$lifetime>)? {
                $($(
                    $(#[$field_attrs])*
                    pub $field_name: $crate::theming::macros::Preference<$field_ty>,
                )*)?
            }

            #[derive(Clone, Debug, PartialEq)]
            $(#[doc = "Theming properties for the `" $name "` component."] $($component_attr_control)?)?
            $(#[$attrs])*
            $vis struct [<$name Theme>] $(<$lifetime>)? {
                $($(
                    $(#[$field_attrs])*
                    pub $field_name: $field_ty,
                )*)?
            }

            impl $(<$lifetime>)? [<$name ThemePreference>] $(<$lifetime>)? {
                #[doc = "Checks each field in `optional` and if it's `Some`, it overwrites the corresponding `self` field."]
                pub fn apply_optional(&mut self, optional: & $($lifetime)? [<$name ThemePartial>]) {
                    $($(
                        if let Some($field_name) = &optional.$field_name {
                            self.$field_name = $field_name.clone();
                        }
                    )*)?
                }

                #[doc = "Checks each field in `optional` and if it's `Some`, it overwrites the corresponding `self` field."]
                pub fn resolve(&mut self, colors_sheet: &$crate::theming::component_themes::ColorsSheet) -> [<$name Theme>] {
                    use $crate::theming::macros::ResolvablePreference;
                    [<$name Theme>] {
                        $(
                            $(
                                $field_name: self.$field_name.resolve(colors_sheet),
                            )*
                        )?
                    }
                }
            }

            impl $(<$lifetime>)? [<$name ThemePartial>] $(<$lifetime>)? {
                pub fn new() -> Self {
                    Self::default()
                }

                $($(
                    $(#[$field_attrs])*
                    pub fn $field_name(mut self, $field_name: impl Into<$field_ty>) -> Self {
                        self.$field_name = Some($crate::theming::macros::Preference::Specific($field_name.into()));
                        self
                    }
                )*)?
            }

            pub trait [<$name ThemePartialExt>] {
                $($(
                    $(#[$field_attrs])*
                    fn $field_name(self, $field_name: impl Into<$field_ty>) -> Self;
                )*)?
            }

            impl $(<$lifetime>)? [<$name ThemePartialExt>] for $name $(<$lifetime>)? {
                $($(
                    $(#[$field_attrs])*
                    fn $field_name(mut self, $field_name: impl Into<$field_ty>) -> Self {
                        self.theme = Some(self.theme.unwrap_or_default().$field_name($field_name));
                        self
                    }
                )*)?
            }
        }
    };
}

#[macro_export]
macro_rules! get_theme {
    ($theme_prop:expr, $theme_name:ident) => {{
        let theme = $crate::theming::hooks::get_theme_or_default();
        let theme = theme.read();
        let mut requested_theme = theme.$theme_name.clone();

        if let Some(theme_override) = $theme_prop {
            requested_theme.apply_optional(&theme_override);
        }

        requested_theme.resolve(&theme.colors)
    }};
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Preference<T> {
    Specific(T),
    Reference(&'static str),
}

impl<T> From<T> for Preference<T> {
    fn from(value: T) -> Self {
        Preference::Specific(value)
    }
}

pub trait ResolvablePreference<T: Clone> {
    fn resolve(&self, colors_sheet: &ColorsSheet) -> T;
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
                "hover" => colors_sheet.hover,
                "focus" => colors_sheet.focus,
                "active" => colors_sheet.active,
                "disabled" => colors_sheet.disabled,

                // Utility
                "overlay" => colors_sheet.overlay,
                "shadow" => colors_sheet.shadow,

                // Fallback
                _ => colors_sheet.primary,
            },

            Self::Specific(value) => *value,
        }
    }
}

impl ResolvablePreference<Size> for Preference<Size> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> Size {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => value.clone(),
        }
    }
}

impl ResolvablePreference<Gaps> for Preference<Gaps> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> Gaps {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => *value,
        }
    }
}

impl ResolvablePreference<CornerRadius> for Preference<CornerRadius> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> CornerRadius {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => *value,
        }
    }
}

impl ResolvablePreference<f32> for Preference<f32> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> f32 {
        match self {
            Self::Reference(_) => {
                panic!("Only Colors support references.")
            }
            Self::Specific(value) => *value,
        }
    }
}
