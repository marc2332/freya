#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use std::path::PathBuf;
use std::process::Command;

use freya::{
    prelude::*,
    winit::{dpi::LogicalPosition, window::WindowLevel},
};

fn main() {
    let (width, height) = (64, 800);
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("Dock")
                .with_size(width as f64, height as f64)
                .with_decorations(false)
                .with_resizable(false)
                .with_background(Color::from_rgb(30, 30, 30))
                .with_window_attributes(move |attributes, el| {
                    let attributes = attributes.with_window_level(WindowLevel::AlwaysOnTop);

                    // Position on left side of screen, centered vertically
                    if let Some(monitor) = el
                        .primary_monitor()
                        .or_else(|| el.available_monitors().next())
                    {
                        let size = monitor.size();
                        attributes.with_position(LogicalPosition {
                            x: 0,
                            y: size.height as i32 / 2 - height / 2,
                        })
                    } else {
                        attributes
                    }
                }),
        ),
    )
}

fn app() -> impl IntoElement {
    let apps = use_hook(get_pinned_apps);

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .padding(Gaps::new_all(8.))
        .spacing(8.)
        .background(Color::from_rgb(30, 30, 30))
        .corner_radius(8.)
        .children(apps.iter().filter_map(|app| {
            app.icon_path.as_ref().map(|icon_path| {
                dock_icon(icon_path.clone(), app.name.clone())
            })
        }))
}

fn dock_icon(icon_path: PathBuf, name: String) -> Element {
    let mut hovered = use_state(|| false);

    let background = if *hovered.read() {
        Color::from_rgb(60, 60, 60)
    } else {
        Color::TRANSPARENT
    };

    rect()
        .center()
        .padding(Gaps::new_all(4.))
        .corner_radius(8.)
        .background(background)
        .on_pointer_enter(move |_| hovered.set(true))
        .on_pointer_leave(move |_| hovered.set(false))
        .on_mouse_up(move |_| {
            println!("Clicked: {}", name);
        })
        .child(
            ImageViewer::new(icon_path)
                .width(Size::px(40.))
                .height(Size::px(40.)),
        )
        .into()
}

#[derive(Clone, Debug)]
struct AppInfo {
    name: String,
    icon_path: Option<PathBuf>,
}

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

fn parse_desktop_file(desktop_id: &str) -> (String, Option<String>) {
    let home = env::var("HOME").ok().map(PathBuf::from);

    let search_paths = [
        Some(PathBuf::from("/usr/share/applications")),
        Some(PathBuf::from("/var/lib/flatpak/exports/share/applications")),
        home.as_ref().map(|h| h.join(".local/share/applications")),
        home.as_ref().map(|h| h.join(".local/share/flatpak/exports/share/applications")),
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
        Some(PathBuf::from("/var/lib/flatpak/exports/share/icons/hicolor")),
        home.as_ref().map(|h| h.join(".local/share/icons/hicolor")),
        home.as_ref().map(|h| h.join(".local/share/flatpak/exports/share/icons/hicolor")),
        Some(PathBuf::from("/usr/share/pixmaps")),
    ];

    // Prefer larger icons
    let sizes = ["256x256", "128x128", "96x96", "64x64", "48x48", "scalable"];
    let extensions = ["png", "svg"];

    for dir in icon_dirs.into_iter().flatten() {
        for size in &sizes {
            for ext in &extensions {
                let icon_path = dir.join(size).join("apps").join(format!("{}.{}", icon_name, ext));
                if icon_path.exists() {
                    return Some(icon_path);
                }
            }
        }
    }

    // Check pixmaps directory (flat structure)
    for ext in &extensions {
        let pixmap_path = PathBuf::from("/usr/share/pixmaps").join(format!("{}.{}", icon_name, ext));
        if pixmap_path.exists() {
            return Some(pixmap_path);
        }
    }

    None
}
