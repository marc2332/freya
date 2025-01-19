mod base;
mod themes;

#[doc(hidden)]
pub use ::core::default::Default;
#[doc(hidden)]
pub use ::paste::paste;
#[doc(hidden)]
pub use ::std::borrow::Cow;
pub use themes::*;

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
///     pub Test {
///         %[cows]
///         cow_string: str,
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
                %[subthemes$($subthemes_attr_control:tt)?]
                $(
                    $(#[$subtheme_field_attrs:meta])*
                    $subtheme_field_name:ident: $subtheme_field_ty_name:ident $(<$subtheme_field_ty_lifetime:lifetime>)?,
                )*
            )?
    }) => {
        $crate::define_theme!(NOTHING=$($($component_attr_control)?)?);
        $crate::define_theme!(NOTHING=$($($cows_attr_control)?)?);
        $crate::define_theme!(NOTHING=$($($subthemes_attr_control)?)?);
        $crate::paste! {
            #[derive(Default, Clone, Debug, PartialEq, Eq)]
            #[doc = "You can use this to change a theme for only one component, with the `theme` property."]
            $(#[$attrs])*
            $vis struct [<$name ThemeWith>] $(<$lifetime>)? {
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
                    $(#[$subtheme_field_attrs])*
                    pub $subtheme_field_name: $subtheme_field_ty_name $(<$subtheme_field_ty_lifetime>)?,
                )*)?
                $($(
                    $(#[$cow_field_attrs])*
                    pub $cow_field_name: $crate::Cow<'static, $cow_field_ty>,
                )*)?
            }

            impl $(<$lifetime>)? [<$name Theme>] $(<$lifetime>)? {

                pub fn apply_colors(&mut self, colors: &$crate::ColorsSheet) {
                    $($(
                        self.$subtheme_field_name.apply_colors(colors);
                    )*)?

                    $($(
                        self.$cow_field_name = colors.resolve(self.$cow_field_name.clone());
                    )*)?
                }

                #[doc = "Checks each field in `optional` and if it's `Some`, it overwrites the corresponding `self` field."]
                pub fn apply_optional(&mut self, optional: & $($lifetime)? [<$name ThemeWith>]) {
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
            [<$theme_name With>] {
                #[allow(clippy::needless_update)]
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
        width: str,
        margin: str,
        dropdown_background: str,
        background_button: str,
        hover_background: str,
        border_fill: str,
        focus_border_fill: str,
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
        border_fill: str,
        select_border_fill: str,
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
        shadow: str,
        width: str,
        margin: str,
        corner_radius: str,
        %[subthemes]
        font_theme: FontTheme,
        placeholder_font_theme: FontTheme,
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
        margin: str,
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
        spacing: str,
        background: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub SidebarItem {
        %[cows]
        margin: str,
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
        border_fill: str,
    }
}

define_theme! {
    %[component]
    pub Checkbox {
        %[cows]
        unselected_fill: str,
        selected_fill: str,
        selected_icon_fill: str,
        border_fill: str,
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

define_theme! {
    %[component]
    pub Tab {
        %[cows]
        background: str,
        hover_background: str,
        border_fill: str,
        focus_border_fill: str,
        width: str,
        height: str,
        padding: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub BottomTab {
        %[cows]
        background: str,
        hover_background: str,
        width: str,
        height: str,
        padding: str,
        %[subthemes]
        font_theme: FontTheme,
    }
}

define_theme! {
    %[component]
    pub ResizableHandle {
        %[cows]
        background: str,
        hover_background: str,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorsSheet {
    pub primary: Cow<'static, str>,
    pub secondary: Cow<'static, str>,
    pub tertiary: Cow<'static, str>,
    pub surface: Cow<'static, str>,
    pub secondary_surface: Cow<'static, str>,
    pub neutral_surface: Cow<'static, str>,
    pub focused_surface: Cow<'static, str>,
    pub opposite_surface: Cow<'static, str>,
    pub secondary_opposite_surface: Cow<'static, str>,
    pub tertiary_opposite_surface: Cow<'static, str>,
    pub background: Cow<'static, str>,
    pub focused_border: Cow<'static, str>,
    pub solid: Cow<'static, str>,
    pub color: Cow<'static, str>,
    pub primary_color: Cow<'static, str>,
    pub placeholder_color: Cow<'static, str>,
    pub highlight_color: Cow<'static, str>,
}

impl ColorsSheet {
    pub fn resolve(&self, val: Cow<'static, str>) -> Cow<'static, str> {
        if val.starts_with("key") {
            let key_val = val.replace("key(", "").replace(")", "");
            match key_val.as_str() {
                "primary" => self.primary.clone(),
                "secondary" => self.secondary.clone(),
                "tertiary" => self.tertiary.clone(),
                "surface" => self.surface.clone(),
                "secondary_surface" => self.secondary_surface.clone(),
                "neutral_surface" => self.neutral_surface.clone(),
                "focused_surface" => self.focused_surface.clone(),
                "opposite_surface" => self.opposite_surface.clone(),
                "secondary_opposite_surface" => self.secondary_opposite_surface.clone(),
                "tertiary_opposite_surface" => self.tertiary_opposite_surface.clone(),
                "background" => self.background.clone(),
                "focused_border" => self.focused_border.clone(),
                "solid" => self.solid.clone(),
                "color" => self.color.clone(),
                "primary_color" => self.primary_color.clone(),
                "placeholder_color" => self.placeholder_color.clone(),
                "highlight_color" => self.highlight_color.clone(),
                _ => self.primary.clone(),
            }
        } else {
            val
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    pub name: &'static str,
    pub colors: ColorsSheet,
    pub body: BodyTheme,
    pub button: ButtonTheme,
    pub filled_button: ButtonTheme,
    pub outline_button: ButtonTheme,
    pub switch: SwitchTheme,
    pub scroll_bar: ScrollBarTheme,
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
    pub tab: TabTheme,
    pub bottom_tab: BottomTabTheme,
    pub resizable_handle: ResizableHandleTheme,
}

impl Default for Theme {
    fn default() -> Self {
        LIGHT_THEME
    }
}
