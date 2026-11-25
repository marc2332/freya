//! # Theming
//!
//! 1. [Use built-in components](#1-use-built-in-components)
//! 2. [Use theme extension traits](#2-use-theme-extension-traits)
//! 3. [Set custom theme values](#3-set-custom-theme-values)
//! 4. [Define a custom component theme](#4-define-a-custom-component-theme)
//!
//! ## 1. Use built-in components
//!
//! Built-in components use the active theme automatically. Set it at the app root with
//! [`use_init_theme`](crate::prelude::use_init_theme) and replace the returned state to switch themes.
//! In a descendant component, call [`use_theme`](crate::prelude::use_theme) to read or update that state.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn app() -> impl IntoElement {
//!     let mut theme = use_init_theme(light_theme);
//!     Button::new()
//!         .filled()
//!         .compact()
//!         .corner_radius(4.)
//!         .on_press(move |_| theme.set(dark_theme()))
//!         .child("Use dark theme")
//! }
//! ```
//!
//! Examples: [switch themes](https://github.com/marc2332/freya/blob/main/examples/theme_switcher.rs),
//! [button variants](https://github.com/marc2332/freya/blob/main/examples/component_button.rs).
//!
//! ## 2. Use theme extension traits
//!
//! [`RectThemeExt`](crate::prelude::RectThemeExt) and
//! [`LabelThemeExt`](crate::prelude::LabelThemeExt) add builders that read from the active theme.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # fn content() -> impl IntoElement {
//! rect()
//!     .theme_background()
//!     .child(label().theme_color().title().text("Settings"))
//! # }
//! ```
//!
//! Example: [interactive typography, padding, and corner radius](https://github.com/marc2332/freya/blob/main/examples/theme_scales.rs).
//!
//! ## 3. Set custom theme values
//!
//! Start with a built-in theme. Use [`Theme::set_var`](crate::prelude::Theme::set_var) with
//! a [`ColorRole`](crate::prelude::ColorRole) key to replace a built-in value, or use your
//! own key to add a value. [`Theme::get_var`](crate::prelude::Theme::get_var) reads it back.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn app() -> impl IntoElement {
//!     let theme = use_init_theme(|| {
//!         let mut theme = light_theme();
//!         theme.set_var(ColorRole::Primary.key(), Color::from_rgb(40, 95, 160));
//!         theme.set_var("app.badge_background", Color::from_rgb(60, 110, 80));
//!         CornerRadiusScale {
//!             base_radius: 12.,
//!             ..CornerRadiusScale::default()
//!         }
//!         .apply_to(&mut theme.variables);
//!         PaddingScale { factor: 0.875 }.apply_to(&mut theme.variables);
//!         theme
//!     });
//!
//!     let badge_background = *theme.read().get_var::<Color>("app.badge_background");
//!     rect().child(Button::new().filled().child("Save")).child(
//!         rect()
//!             .background(badge_background)
//!             .color(Color::WHITE)
//!             .child("Ready"),
//!     )
//! }
//! ```
//!
//! The filled button uses the overridden primary color, radius scale, and padding scale.
//! Replace a single shared padding value with [`PaddingRole`](crate::prelude::PaddingRole)
//! and `Theme::set_var`, or use a component's `padding` builder for an instance override.
//!
//! Examples: [custom accent colors](https://github.com/marc2332/freya/blob/main/examples/theme_color_accent.rs),
//! [interactive theme scales](https://github.com/marc2332/freya/blob/main/examples/theme_scales.rs).
//!
//! ## 4. Define a custom component theme
//!
//! Declare the component theme with [`define_theme!`](crate::prelude::define_theme), then register it
//! with [`Theme::set_theme`](crate::prelude::Theme::set_theme) in your theme, and resolve it in
//! `render` with [`get_theme!`](crate::prelude::get_theme). Use the same key for both calls.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! define_theme! {
//!     %[component]
//!     pub StatusBadge {
//!         %[fields]
//!         background: Color,
//!     }
//! }
//! # #[derive(PartialEq)]
//! # struct StatusBadge { theme: Option<StatusBadgeThemePartial>, key: DiffKey }
//! # impl KeyExt for StatusBadge {
//! #     fn key_mut(&mut self) -> &mut DiffKey {
//! #         &mut self.key
//! #     }
//! # }
//!
//! fn custom_theme() -> Theme {
//!     let mut theme = light_theme();
//!     theme.set_var("app.badge_background", Color::from_rgb(60, 110, 80));
//!     theme.set_theme(
//!         "app.status_badge",
//!         StatusBadgeThemePreference {
//!             background: Preference::Reference("app.badge_background"),
//!         },
//!     );
//!     theme
//! }
//!
//! impl Component for StatusBadge {
//!     fn render(&self) -> impl IntoElement {
//!         let theme = get_theme!(&self.theme, StatusBadgeThemePreference, "app.status_badge");
//!         rect().background(theme.background).child("Active")
//!     }
//! #     fn render_key(&self) -> DiffKey {
//! #         self.key.clone().or(self.default_key())
//! #     }
//! }
//! ```
//!
//! [`Preference::Reference`](crate::prelude::Preference::Reference) reads a shared value.
//! The key must exist and match the field's type. Use
//! [`Preference::Specific`](crate::prelude::Preference::Specific) for a fixed value.
//!
//! Example: [a themed status badge](https://github.com/marc2332/freya/blob/main/examples/theme_definition.rs).
