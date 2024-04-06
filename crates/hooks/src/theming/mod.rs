mod dark;
mod light;

pub use dark::*;
pub use light::*;

#[doc(hidden)]
pub use ::core::default::Default;
#[doc(hidden)]
pub use ::paste::paste;
#[doc(hidden)]
pub use ::std::borrow::Cow;

/// Alias for `Cow::Borrowed`, because that's used a million times so shortening it is nice.
/// Makes the code more readable.
#[macro_export]
macro_rules! cow_borrowed {
    ($val:expr) => {
        $crate::Cow::Borrowed($val)
    };
}

/// Example usage:
///
/// ```rust
/// # use crate::freya_hooks::define_theme;
/// # use crate::freya_hooks::FontTheme;
/// # use crate::freya_hooks::FontThemeWith;
/// # #[derive(Clone, Debug, PartialEq, Eq)]
/// # struct Bar;
/// # #[derive(Clone, Debug, PartialEq, Eq)]
/// # struct Foo;
/// define_theme! {
///     %[component]
///     pub Test<'a> {
///         %[cows]
///         cow_string: str,
///         %[borrowed]
///         borrowed_data: &'a Foo,
///         %[owned]
///         owned_data: Bar,
///         %[subthemes]
///         font_theme: FontTheme,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_theme {
    (NOTHING=) => {};

    (
        $(#[$attrs:meta])*
        $(%[component$($component_attr_control:tt)?])?
        $vis:vis $name:ident $(<$lifetime:lifetime>)? {
            $(
                %[cows$($cows_attr_control:tt)?]
                $(
                    $(#[$cow_field_attrs:meta])*
                    $cow_field_name:ident: $cow_field_ty:ty,
                )*
            )?
            $(
                %[borrowed$($borrowed_attr_control:tt)?]
                $(
                    $(#[$borrowed_field_attrs:meta])*
                    $borrowed_field_name:ident: $borrowed_field_ty:ty,
                )*
            )?
            $(
                %[owned$($owned_attr_control:tt)?]
                $(
                    $(#[$owned_field_attrs:meta])*
                    $owned_field_name:ident: $owned_field_ty:ty,
                )*
            )?
            $(
                %[subthemes$($subthemes_attr_control:tt)?]
                $(
                    $(#[$subtheme_field_attrs:meta])*
                    $subtheme_field_name:ident: $subtheme_field_ty_name:ident $(<$subtheme_field_ty_lifetime:lifetime>)?,
                )*
            )?
    }) => {
        $crate::define_theme!(NOTHING=$($($component_attr_control)?)?);
        $crate::define_theme!(NOTHING=$($($cows_attr_control)?)?);
        $crate::define_theme!(NOTHING=$($($borrowed_attr_control)?)?);
        $crate::define_theme!(NOTHING=$($($owned_attr_control)?)?);
        $crate::define_theme!(NOTHING=$($($subthemes_attr_control)?)?);
        $crate::paste! {
            #[derive(Default, Clone, Debug, PartialEq, Eq)]
            #[doc = "You can use this to change a theme for only one component, with the `theme` property."]
            $(#[$attrs])*
            $vis struct [<$name ThemeWith>] $(<$lifetime>)? {
                $($(
                    $(#[$borrowed_field_attrs])*
                    pub $borrowed_field_name: Option<$borrowed_field_ty>,
                )*)?
                $($(
                    $(#[$owned_field_attrs])*
                    pub $owned_field_name: Option<$owned_field_ty>,
                )*)?
                $($(
                    $(#[$subtheme_field_attrs])*
                    pub $subtheme_field_name: Option< [<$subtheme_field_ty_name With>] $(<$subtheme_field_ty_lifetime>)? >,
                )*)?
                $($(
                    $(#[$cow_field_attrs])*
                    pub $cow_field_name: Option<$crate::Cow<'static, $cow_field_ty>>,
                )*)?
            }

            #[derive(Clone, Debug, PartialEq, Eq)]
            $(#[doc = "Theming properties for the `" $name "` component."] $($component_attr_control)?)?
            $(#[$attrs])*
            $vis struct [<$name Theme>] $(<$lifetime>)? {
                $($(
                    $(#[$borrowed_field_attrs])*
                    pub $borrowed_field_name: $borrowed_field_ty,
                )*)?
                $($(
                    $(#[$owned_field_attrs])*
                    pub $owned_field_name: $owned_field_ty,
                )*)?
                $($(
                    $(#[$subtheme_field_attrs])*
                    pub $subtheme_field_name: $subtheme_field_ty_name $(<$subtheme_field_ty_lifetime>)?,
                )*)?
                $($(
                    $(#[$cow_field_attrs])*
                    pub $cow_field_name: $crate::Cow<'static, $cow_field_ty>,
                )*)?
            }

            impl $(<$lifetime>)? [<$name Theme>] $(<$lifetime>)? {
                #[doc = "Checks each field in `optional` and if it's `Some`, it overwrites the corresponding `self` field."]
                pub fn apply_optional(&mut self, optional: & $($lifetime)? [<$name ThemeWith>]) {
                    $($(
                        if let Some($borrowed_field_name) = optional.$borrowed_field_name {
                            self.$borrowed_field_name = $borrowed_field_name;
                        }
                    )*)?

                    $($(
                        if let Some($owned_field_name) = &optional.$owned_field_name {
                            self.$owned_field_name = $owned_field_name.clone();
                        }
                    )*)?

                    $($(
                        if let Some($subtheme_field_name) = &optional.$subtheme_field_name {
                            self.$subtheme_field_name.apply_optional($subtheme_field_name);
                        }
                    )*)?

                    $($(
                        if let Some($cow_field_name) = &optional.$cow_field_name {
                            self.$cow_field_name = $cow_field_name.clone();
                        }
                    )*)?
                }
            }
        }
    };
}

/// Create `FooThemeWith` structs without having to deal with the verbose syntax.
///
/// # Examples
///
/// Without the macro:
///
/// ```no_run
/// # use freya::prelude::*;
/// # fn theme_with_example_no_macro() -> Element {
/// rsx! {
///     Button {
///         theme: ButtonThemeWith {
///             background: Some("blue".into()),
///             font_theme: FontThemeWith {
///                 color: Some("white".into()),
///                 ..Default::default()
///             }.into(),
///             ..Default::default()
///         }
///     }
/// }
/// # }
/// ```
///
/// With the macro:
///
/// ```no_run
/// # use freya::prelude::*;
/// # fn theme_with_example_no_macro() -> Element {
/// rsx! {
///     Button {
///         theme: theme_with!(ButtonTheme {
///             background: "blue".into(),
///             font_theme: theme_with!(FontTheme {
///                 color: "white".into(),
///             }),
///         })
///     }
/// }
/// # }
/// ```
#[macro_export]
macro_rules! theme_with {
    ($theme_name:ident {
        $(
            $theme_field_name:ident: $theme_field_val:expr
        ),* $(,)?
    }) => {
        $crate::paste! {
            #[allow(clippy::needless_update)]
            [<$theme_name With>] {
                $($theme_field_name: Some($theme_field_val),)*
                ..$crate::Default::default()
            }
        }
    };
}

define_theme! {
    %[component]
    pub Dropdown {
        %[cows]
        dropdown_background: str,
        background_button: str,
        hover_background: str,
        border_fill: str,
        arrow_fill: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub DropdownItem {
        %[cows]
        background: str,
        select_background: str,
        hover_background: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub Button {
        %[cows]
        background: str,
        hover_background: str,
        border_fill: str,
        focus_border_fill: str,
        shadow: str,
        margin: str,
        corner_radius: str,
        width: str,
        height: str,
        padding: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub Input {
        %[cows]
        background: str,
        hover_background: str,
        border_fill: str,
        width: str,
        margin: str,
        corner_radius: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    /// Theming properties for Fonts.
    pub Font {
        %[cows]
        color: str,
    }
}

define_theme! {
    %[component]
    pub Switch {
        %[cows]
        background: str,
        thumb_background: str,
        enabled_background: str,
        enabled_thumb_background: str,
        focus_border_fill: str,
        enabled_focus_border_fill: str,
    }
}

define_theme! {
    %[component]
    pub ScrollBar {
        %[cows]
        background: str,
        thumb_background: str,
        hover_thumb_background: str,
        active_thumb_background: str,
        size: str,
    }
}

define_theme! {
    /// Also used by `VirtualScrollView`.
    %[component]
    pub ScrollView {
        %[cows]
        height: str,
        width: str,
        padding: str,
    }
}

define_theme! {
    %[component]
    pub Body {
        %[cows]
        background: str,
        color: str,
        padding: str,
    }
}

define_theme! {
    %[component]
    pub Slider {
        %[cows]
        background: str,
        thumb_background: str,
        thumb_inner_background: str,
        border_fill: str,
    }
}

define_theme! {
    %[component]
    pub Tooltip {
        %[cows]
        background: str,
        color: str,
        border_fill: str,
    }
}

define_theme! {
    %[component]
    pub Accordion {
        %[cows]
        color: str,
        background: str,
        border_fill: str,
    }
}

define_theme! {
    %[component]
    pub Loader {
        %[cows]
        primary_color: str,
    }
}

define_theme! {
    %[component]
    pub Link {
        %[cows]
        highlight_color: str,
    }
}

define_theme! {
    %[component]
    pub ProgressBar {
        %[cows]
        color: str,
        background: str,
        progress_background: str,
        width: str,
        height: str,
    }
}

define_theme! {
    %[component]
    pub Table {
        %[cows]
        background: str,
        arrow_fill: str,
        alternate_row_background: str,
        row_background: str,
        divider_fill: str,
        height: str,
        corner_radius: str,
        shadow: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub Canvas {
        %[cows]
        width: str,
        height: str,
        background: str,
    }
}

define_theme! {
    %[component]
    pub Graph {
        %[cows]
        width: str,
        height: str,
    }
}

define_theme! {
    %[component]
    pub NetworkImage {
        %[cows]
        width: str,
        height: str,
    }
}

define_theme! {
    %[component]
    pub Icon {
        %[cows]
        margin: str,
        width: str,
        height: str,
    }
}

define_theme! {
    %[component]
    pub Sidebar {
        %[cows]
        background: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub SidebarItem {
        %[cows]
        background: str,
        hover_background: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub Tile {
        %[cows]
        padding: str,
    }
}

define_theme! {
    %[component]
    pub MenuItem {
        %[cows]
        hover_background: str,
        corner_radius: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub MenuContainer {
        %[cows]
        background: str,
        padding: str,
        shadow: str,
    }
}

define_theme! {
    %[component]
    pub SnackBar {
        %[cows]
        background: str,
        color: str,
     }
}

define_theme! {
    %[component]
    pub Radio {
        %[cows]
        unselected_fill: str,
        selected_fill: str,
    }
}

define_theme! {
    %[component]
    pub Checkbox {
        %[cows]
        unselected_fill: str,
        selected_fill: str,
        selected_icon_fill: str,
    }
}

define_theme! {
    %[component]
    pub Popup {
        %[cows]
        background: str,
        color: str,
        cross_fill: str,
        width: str,
        height: str,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    pub name: &'static str,
    pub body: BodyTheme,
    pub button: ButtonTheme,
    pub switch: SwitchTheme,
    pub scroll_bar: ScrollBarTheme,
    pub scroll_view: ScrollViewTheme,
    pub slider: SliderTheme,
    pub tooltip: TooltipTheme,
    pub dropdown: DropdownTheme,
    pub dropdown_item: DropdownItemTheme,
    pub accordion: AccordionTheme,
    pub loader: LoaderTheme,
    pub link: LinkTheme,
    pub progress_bar: ProgressBarTheme,
    pub table: TableTheme,
    pub input: InputTheme,
    pub canvas: CanvasTheme,
    pub graph: GraphTheme,
    pub network_image: NetworkImageTheme,
    pub icon: IconTheme,
    pub sidebar: SidebarTheme,
    pub sidebar_item: SidebarItemTheme,
    pub tile: TileTheme,
    pub radio: RadioTheme,
    pub checkbox: CheckboxTheme,
    pub menu_item: MenuItemTheme,
    pub menu_container: MenuContainerTheme,
    pub snackbar: SnackBarTheme,
    pub popup: PopupTheme,
}

impl Default for Theme {
    fn default() -> Self {
        LIGHT_THEME
    }
}
