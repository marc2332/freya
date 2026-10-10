#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    persisted::*,
    prelude::*,
    radio::*,
};
use serde::{
    Deserialize,
    Serialize,
    de::Error as _,
};

#[derive(Clone, Deserialize, Serialize)]
struct Settings {
    schema_version: u32,
    step_size: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            schema_version: 1,
            step_size: 1,
        }
    }
}

impl Settings {
    fn migrate(document: serde_json::Value) -> serde_json::Result<serde_json::Value> {
        let mut settings: serde_json::Map<String, serde_json::Value> =
            serde_json::from_value(document)?;
        let version: u32 =
            serde_json::from_value(settings.get("schema_version").cloned().unwrap_or(0.into()))?;

        match version {
            0 => {
                settings.insert("schema_version".to_string(), 1.into());
                settings.entry("step_size".to_string()).or_insert(1.into());
            }
            1 => {}
            _ => return Err(serde_json::Error::custom("unsupported schema version")),
        }

        Ok(settings.into())
    }
}

struct AppState {
    settings: Settings,
    count: u32,
}

impl AppState {
    fn apply_settings(&mut self, next: Settings) -> Option<SettingsChannel> {
        let channel =
            (self.settings.step_size != next.step_size).then_some(SettingsChannel::Reloaded);
        self.settings = next;

        channel
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum SettingsChannel {
    Count,
    StepSize,
    Save,
    Reloaded,
}

impl RadioChannel<AppState> for SettingsChannel {
    fn derive_channel(self, _state: &AppState) -> Vec<Self> {
        match self {
            Self::StepSize => vec![Self::StepSize, Self::Save],
            Self::Reloaded => vec![Self::StepSize],
            channel => vec![channel],
        }
    }
}

struct PersistedWindow {
    title: &'static str,
    transport: PersistedLocal,
    settings: Settings,
}

impl PersistedWindow {
    fn open(title: &'static str, file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let transport = PersistedLocal::for_app("freya-persisted-per-window", file_name)?;
        tracing::info!(window = title, path = %transport.path().display(), "Opening settings");
        let settings = Persisted::new(transport.clone(), PersistedJson)
            .load_migrated(Settings::migrate)?
            .unwrap_or_default();

        Ok(Self {
            title,
            transport,
            settings,
        })
    }
}

impl App for PersistedWindow {
    fn render(&self) -> impl IntoElement {
        let title = self.title;
        let mut station = use_init_radio_station::<AppState, SettingsChannel>(|| AppState {
            settings: self.settings.clone(),
            count: 0,
        });
        let persisted = use_hook(|| Persisted::new(self.transport.clone(), PersistedJson));

        use_side_effect({
            let persisted = persisted.clone();
            move || {
                station.read_channel(SettingsChannel::Save);

                match persisted.save_radio(station, |state| &state.settings) {
                    Ok(()) => tracing::info!(window = title, "Autosaved settings"),
                    Err(error) => {
                        tracing::error!(window = title, %error, "Could not autosave settings")
                    }
                }
            }
        });

        use_hook(|| match self.transport.watch() {
            Ok(events) => {
                spawn(async move {
                    loop {
                        match events.recv().await {
                            Ok(()) => match persisted.reload_radio_migrated(
                                station,
                                AppState::apply_settings,
                                Settings::migrate,
                            ) {
                                Ok(Some(_)) => tracing::info!(window = title, "Reloaded settings"),
                                Ok(None) => {}
                                Err(error) => {
                                    tracing::error!(window = title, %error, "Could not reload settings")
                                }
                            },
                            Err(error) => {
                                tracing::error!(window = title, %error, "Could not watch settings");
                                break;
                            }
                        }
                    }
                });
            }
            Err(error) => tracing::error!(window = title, %error, "Could not watch settings"),
        });

        let state = station.read();

        rect()
            .expanded()
            .center()
            .spacing(12.0)
            .child(title)
            .child(format!("Count: {}", state.count))
            .child(format!("Step size: {}", state.settings.step_size))
            .child(
                Button::new()
                    .on_press(move |_| {
                        let mut state = station.write_channel(SettingsChannel::Count);
                        state.count = state.count.saturating_add(state.settings.step_size);
                    })
                    .child("Increment"),
            )
            .child(
                Button::new()
                    .on_press(move |_| {
                        let mut state = station.write_channel(SettingsChannel::StepSize);
                        state.settings.step_size = state.settings.step_size.saturating_add(1);
                    })
                    .child("Increase step size"),
            )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "state_persisted_per_window=info".into()),
        )
        .init();

    launch(
        LaunchConfig::new()
            .with_window(
                WindowConfig::new_app(PersistedWindow::open("Window 1", "window-1.json")?)
                    .with_title("Window 1"),
            )
            .with_window(
                WindowConfig::new_app(PersistedWindow::open("Window 2", "window-2.json")?)
                    .with_title("Window 2"),
            ),
    );
    Ok(())
}
