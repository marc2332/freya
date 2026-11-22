#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

define_theme! {
    %[component]
    pub StatusBadge {
        %[fields]
        background: Color,
        color: Color,
        corner_radius: CornerRadius,
        padding: Gaps,
    }
}

#[derive(PartialEq)]
struct StatusBadge {
    theme: Option<StatusBadgeThemePartial>,
}

impl StatusBadge {
    fn new() -> Self {
        Self { theme: None }
    }
}

impl Component for StatusBadge {
    fn render(&self) -> impl IntoElement {
        let StatusBadgeTheme {
            background,
            color,
            corner_radius,
            padding,
        } = get_theme!(&self.theme, StatusBadgeThemePreference, "status_badge");

        rect()
            .background(background)
            .corner_radius(corner_radius)
            .padding(padding)
            .child(label().text("Active").color(color).font_size(12.))
    }
}

fn custom_theme() -> Theme {
    let mut theme = dark_theme();
    theme.name = "custom";
    theme.set_var(ColorRole::Primary.key(), Color::from_rgb(37, 52, 63));
    theme.set_var("app.badge_background", Color::from_rgb(255, 155, 81));
    theme.set_var("app.badge_radius", CornerRadius::new_all(99.));
    theme.set_theme(
        "status_badge",
        StatusBadgeThemePreference {
            background: Preference::Reference("app.badge_background"),
            color: Preference::Reference(ColorRole::TextInverse.key()),
            corner_radius: Preference::Reference("app.badge_radius"),
            padding: Gaps::new(4., 10., 4., 10.).into(),
        },
    );
    theme
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    use_init_theme(custom_theme);
    let toggled = use_state(|| false);

    rect()
        .theme_background()
        .theme_color()
        .center()
        .expanded()
        .spacing(12.)
        .child(Switch::new(toggled))
        .child(StatusBadge::new().background((123, 123, 123)))
}
