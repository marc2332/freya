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
    theme.colors = ColorsSheet {
        primary: Color::from_rgb(37, 52, 63),
        secondary: Color::from_rgb(255, 155, 81),
        tertiary: Color::from_rgb(81, 155, 255),
        ..DARK_COLORS
    };
    theme.set(
        "status_badge",
        StatusBadgeThemePreference {
            background: Preference::Reference("secondary"),
            color: Preference::Reference("text_inverse"),
            corner_radius: CornerRadius::new_all(99.).into(),
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
    let mut toggled = use_state(|| false);

    rect()
        .theme_background()
        .theme_color()
        .center()
        .expanded()
        .spacing(12.)
        .child(
            Switch::new()
                .toggled(toggled)
                .on_toggle(move |_| toggled.toggle()),
        )
        .child(StatusBadge::new().background((123, 123, 123)))
}
