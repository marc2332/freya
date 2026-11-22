#[doc(hidden)]
pub use ::paste::paste;

use crate::theming::component_themes::AnyMap;
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
                    pub $field_name: $field_ty,
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

                /// Resolve references using the supplied variables.
                pub fn resolve(&self, variables: &$crate::theming::component_themes::AnyMap) -> [<$name Theme>] {
                    [<$name Theme>] {
                        $($(
                            $field_name: self.$field_name.resolve(variables),
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
        let requested_theme = match theme.get_theme::<$theme_type>($theme_key) {
            Some(requested_theme) => requested_theme,
            None => {
                default_theme = ($default)();
                &default_theme
            }
        };

        if let Some(theme_override) = $theme_prop {
            let mut requested_theme = requested_theme.clone();
            requested_theme.apply_optional(&theme_override);
            requested_theme.resolve(&theme.variables)
        } else {
            requested_theme.resolve(&theme.variables)
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
}

impl<T> From<T> for Preference<T> {
    fn from(value: T) -> Self {
        Self::Specific(value)
    }
}

impl<T: Clone + 'static> Preference<T> {
    /// Resolve the preference. A reference must exist with the expected type.
    pub fn resolve(&self, variables: &AnyMap) -> T {
        match self {
            Self::Specific(value) => value.clone(),
            Self::Reference(key) => variables.get::<T>(key).clone(),
        }
    }
}
