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

#[derive(Deserialize, Serialize)]
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
                tracing::info!(
                    from_version = 0,
                    to_version = 1,
                    "Migrated configuration in memory"
                );
            }
            1 => {}
            _ => return Err(serde_json::Error::custom("unsupported schema version")),
        }
        Ok(settings.into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "state_persisted=info".into()),
        )
        .init();

    // Create a local persist transport
    let transport = PersistedLocal::for_app("freya-persisted-example", "settings.json")?;
    tracing::info!(path = %transport.path().display(), "Opening configuration");
    let events = transport.watch()?;

    // Create a persist with the local transport and json format
    let persisted = Persisted::new(transport, PersistedJson);

    // Load the current persit data and migrate it if needed
    let settings = persisted
        .load_migrated(Settings::migrate)?
        .unwrap_or_default();

    // Create the radio with persisted settings and a runtime-only count
    let station =
        RadioStation::<AppState, SettingsChannel>::create_global(AppState { settings, count: 0 });

    launch(
        LaunchConfig::new()
            .with_task(move |_| async move {
                // Save the latest changes of the radio
                Effect::create_global({
                    let persisted = persisted.clone();
                    move || {
                        station.read_channel(SettingsChannel::Save);

                        match persisted.save_radio(station, |state| &state.settings) {
                            Ok(()) => tracing::info!("Autosaved settings"),
                            Err(error) => tracing::error!(%error, "Could not autosave settings"),
                        }
                    }
                });

                loop {
                    // If the file is modified outside this app we update the radio and try to migrate
                    match events.recv().await {
                        Ok(()) => match persisted.reload_radio_migrated(
                            station,
                            AppState::apply_settings,
                            Settings::migrate,
                        ) {
                            Ok(Some(_)) => tracing::info!("Reloaded settings after file change"),
                            Ok(None) => {}
                            Err(error) => {
                                tracing::error!(%error, "Could not reload changed settings file")
                            }
                        },
                        Err(error) => {
                            tracing::error!(%error, "Could not watch settings file");
                            break;
                        }
                    }
                }
            })
            .with_window(WindowConfig::new(app).with_root_context(station)),
    );
    Ok(())
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .spacing(12.0)
        .child(CountDisplay)
        .child(SettingsControls)
}

#[derive(PartialEq)]
struct CountDisplay;

impl Component for CountDisplay {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<AppState, SettingsChannel>(SettingsChannel::Count);
        tracing::info!("Rendering count subscriber");
        format!("Count: {}", radio.read().count)
    }
}

#[derive(PartialEq)]
struct SettingsControls;

impl Component for SettingsControls {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio::<AppState, SettingsChannel>(SettingsChannel::StepSize);
        tracing::info!("Rendering step-size subscriber");

        rect()
            .spacing(12.0)
            .child(format!("Step size: {}", radio.read().settings.step_size))
            .child(
                Button::new()
                    .on_press(move |_| {
                        let mut state = radio.write_channel(SettingsChannel::Count);
                        state.count = state.count.saturating_add(state.settings.step_size);
                    })
                    .child("Increment"),
            )
            .child(
                Button::new()
                    .on_press(move |_| {
                        let mut state = radio.write_channel(SettingsChannel::StepSize);
                        state.settings.step_size = state.settings.step_size.saturating_add(1);
                    })
                    .child("Increase step size"),
            )
    }
}
