#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_os = "linux")]
use std::{
    env,
    path::PathBuf,
    process::Command,
};

#[cfg(target_os = "linux")]
use freya::winit::platform::x11::{
    WindowAttributesExtX11,
    WindowType,
};
#[cfg(target_os = "linux")]
use freya::{
    prelude::*,
    winit::{
        dpi::LogicalPosition,
        window::WindowLevel,
    },
};

#[cfg(target_os = "linux")]
fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("Dock")
                .with_size(50., 800.)
                .with_decorations(false)
                .with_resizable(false)
                .with_background(Color::TRANSPARENT)
                .with_transparency(true)
                .with_window_attributes(move |attributes, _| {
                    attributes
                        .with_x11_window_type(vec![WindowType::Dock])
                        .with_window_level(WindowLevel::AlwaysOnTop)
                        .with_position(LogicalPosition { x: 54, y: 32 })
                }),
        ),
    )
}

#[cfg(not(target_os = "linux"))]
fn main() {
    panic!("This example only runs on Linux");
}

#[cfg(target_os = "linux")]
fn app() -> impl IntoElement {
    let apps = use_hook(get_pinned_apps);

    rect()
        .expanded()
        .padding(8.)
        .spacing(8.)
        .background((30, 30, 30, 0.8))
        .children(apps.iter().filter_map(|app| {
            app.icon_path.as_ref().map(|icon_path| {
                DockIcon {
                    icon_path: icon_path.clone(),
                    name: app.name.clone(),
                }
                .into()
            })
        }))
}

#[cfg(target_os = "linux")]
#[derive(PartialEq)]
struct DockIcon {
    icon_path: PathBuf,
    name: String,
}

#[cfg(target_os = "linux")]
impl Component for DockIcon {
    fn render(&self) -> impl IntoElement {
        let mut hovered = use_state(|| false);

        let is_svg = self
            .icon_path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("svg"));

        let icon_element: Element = if is_svg {
            let icon_path = self.icon_path.clone();
            let svg_data = use_hook(|| {
                std::fs::read(&icon_path)
                    .map(Bytes::from)
                    .unwrap_or_default()
            });
            svg(svg_data)
                .width(Size::px(28.))
                .height(Size::px(28.))
                .into()
        } else {
            ImageViewer::new(self.icon_path.clone())
                .sampling_mode(SamplingMode::Trilinear)
                .width(Size::px(28.))
                .height(Size::px(28.))
                .into()
        };

        let name = self.name.clone();
        let background = if hovered() {
            Color::from_rgb(60, 60, 60)
        } else {
            Color::TRANSPARENT
        };

        rect()
            .center()
            .padding(4.)
            .corner_radius(8.)
            .background(background)
            .on_pointer_enter(move |_| hovered.set(true))
            .on_pointer_leave(move |_| hovered.set(false))
            .on_press(move |_| {
                println!("Clicked: {}", name);
            })
            .child(icon_element)
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.name)
    }
}

#[cfg(target_os = "linux")]
#[derive(Clone, Debug)]
struct AppInfo {
    name: String,
    icon_path: Option<PathBuf>,
}

#[cfg(target_os = "linux")]
fn get_pinned_apps() -> Vec<AppInfo> {
    let output = Command::new("dconf")
        .args(["read", "/org/gnome/shell/favorite-apps"])
        .output()
        .ok();

    let Some(output) = output else {
        return Vec::new();
    };

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse the GVariant array format: ['app1.desktop', 'app2.desktop', ...]
    let apps: Vec<String> = stdout
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(", ")
        .filter_map(|s| {
            let trimmed = s.trim().trim_matches('\'');
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect();

    apps.into_iter()
        .map(|desktop_id| {
            let (name, icon_name) = parse_desktop_file(&desktop_id);
            let icon_path = icon_name.and_then(|icon| find_icon_path(&icon));
            AppInfo { name, icon_path }
        })
        .collect()
}

#[cfg(target_os = "linux")]
fn parse_desktop_file(desktop_id: &str) -> (String, Option<String>) {
    let home = env::var("HOME").ok().map(PathBuf::from);

    let search_paths = [
        Some(PathBuf::from("/usr/share/applications")),
        Some(PathBuf::from("/var/lib/flatpak/exports/share/applications")),
        home.as_ref().map(|h| h.join(".local/share/applications")),
        home.as_ref()
            .map(|h| h.join(".local/share/flatpak/exports/share/applications")),
        Some(PathBuf::from("/var/lib/snapd/desktop/applications")),
    ];

    for path in search_paths.into_iter().flatten() {
        let desktop_path = path.join(desktop_id);
        if desktop_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&desktop_path) {
                let mut name = desktop_id.trim_end_matches(".desktop").to_string();
                let mut icon = None;

                for line in content.lines() {
                    if line.starts_with("Name=") && !line.contains('[') {
                        name = line.trim_start_matches("Name=").to_string();
                    } else if line.starts_with("Icon=") {
                        icon = Some(line.trim_start_matches("Icon=").to_string());
                    }
                }

                return (name, icon);
            }
        }
    }

    (desktop_id.trim_end_matches(".desktop").to_string(), None)
}

#[cfg(target_os = "linux")]
fn find_icon_path(icon_name: &str) -> Option<PathBuf> {
    // If it's already an absolute path, use it directly
    if icon_name.starts_with('/') {
        let path = PathBuf::from(icon_name);
        if path.exists() {
            return Some(path);
        }
    }

    let home = env::var("HOME").ok().map(PathBuf::from);

    let icon_dirs = [
        Some(PathBuf::from("/usr/share/icons/hicolor")),
        Some(PathBuf::from(
            "/var/lib/flatpak/exports/share/icons/hicolor",
        )),
        home.as_ref().map(|h| h.join(".local/share/icons/hicolor")),
        home.as_ref()
            .map(|h| h.join(".local/share/flatpak/exports/share/icons/hicolor")),
        Some(PathBuf::from("/usr/share/pixmaps")),
    ];

    // Prefer larger icons, and PNG over SVG (ImageViewer handles PNG better)
    let sizes = ["256x256", "128x128", "96x96", "64x64", "48x48", "scalable"];

    // First pass: look for PNG files only
    for dir in icon_dirs.iter().flatten() {
        for size in &sizes {
            let icon_path = dir
                .join(size)
                .join("apps")
                .join(format!("{}.png", icon_name));
            if icon_path.exists() {
                return Some(icon_path);
            }
        }
    }

    // Check pixmaps for PNG
    let pixmap_path = PathBuf::from("/usr/share/pixmaps").join(format!("{}.png", icon_name));
    if pixmap_path.exists() {
        return Some(pixmap_path);
    }

    // Second pass: fall back to SVG if no PNG found
    for dir in icon_dirs.into_iter().flatten() {
        for size in &sizes {
            let icon_path = dir
                .join(size)
                .join("apps")
                .join(format!("{}.svg", icon_name));
            if icon_path.exists() {
                return Some(icon_path);
            }
        }
    }

    // Check pixmaps for SVG
    let pixmap_path = PathBuf::from("/usr/share/pixmaps").join(format!("{}.svg", icon_name));
    if pixmap_path.exists() {
        return Some(pixmap_path);
    }

    None
}
