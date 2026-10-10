use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use notify::{
    EventKind,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
};

use crate::PersistedLocal;

/// File change notifications. Dropping this receiver stops watching.
pub struct PersistedLocalWatcher {
    events: async_channel::Receiver<notify::Result<()>>,
    _watcher: RecommendedWatcher,
}

impl PersistedLocal {
    /// Watch the file's parent directory to detect creation, removal and atomic replacements.
    /// Creates missing parent directories without writing the file.
    pub fn watch(&self) -> notify::Result<PersistedLocalWatcher> {
        let file_name = self
            .path()
            .file_name()
            .ok_or_else(|| notify::Error::generic("watching requires a file path"))?;
        let parent = self
            .path()
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)?;
        let parent = fs::canonicalize(parent)?;
        let watched_path = parent.join(file_name);
        let (sender, events) = async_channel::bounded(1);
        let mut watcher = notify::recommended_watcher(PersistedLocalWatcher::event_handler(
            watched_path,
            sender,
        ))?;
        watcher.watch(&parent, RecursiveMode::NonRecursive)?;

        Ok(PersistedLocalWatcher {
            events,
            _watcher: watcher,
        })
    }
}

impl PersistedLocalWatcher {
    fn event_handler(
        watched_path: PathBuf,
        sender: async_channel::Sender<notify::Result<()>>,
    ) -> impl Fn(notify::Result<notify::Event>) + Send + 'static {
        move |event| match event {
            Ok(event)
                if event.need_rescan()
                    || (matches!(
                        event.kind,
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                    ) && event.paths.contains(&watched_path)) =>
            {
                let _ = sender.try_send(Ok(()));
            }
            Err(error) => {
                let _ = sender.force_send(Err(error));
            }
            _ => {}
        }
    }

    /// Wait for a change without reading or modifying stored data.
    /// Pending changes coalesce, and watcher errors take priority over queued changes.
    pub async fn recv(&self) -> notify::Result<()> {
        self.events
            .recv()
            .await
            .map_err(|_| notify::Error::generic("file watcher stopped"))?
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use async_io::Timer;
    use futures_lite::future::{
        block_on,
        race,
    };

    use crate::{
        PersistedLocal,
        PersistedLocalWatcher,
        PersistenceTransport,
    };

    #[test]
    fn pending_changes_coalesce_without_hiding_errors() {
        let path = std::path::PathBuf::from("settings.json");
        let (sender, events) = async_channel::bounded(1);
        let callback = PersistedLocalWatcher::event_handler(path.clone(), sender);
        let change = notify::Event::new(notify::EventKind::Create(notify::event::CreateKind::File))
            .add_path(path);
        callback(Ok(change.clone()));
        callback(Ok(change.clone()));
        assert_eq!(events.len(), 1);

        callback(Err(notify::Error::generic("watch failed")));
        callback(Ok(change.clone()));
        assert!(
            events
                .try_recv()
                .unwrap()
                .unwrap_err()
                .to_string()
                .contains("watch failed")
        );
        assert!(events.is_empty());

        callback(Ok(change));
        assert!(events.try_recv().unwrap().is_ok());
    }

    #[test]
    fn watcher_notifies_on_creation_and_atomic_replacements() {
        for existing in [false, true] {
            let directory = tempfile::tempdir_in(".").unwrap();
            let transport = PersistedLocal::new(directory.path().join("nested/settings.bin"));
            if existing {
                transport.save(&[1]).unwrap();
            }
            let watcher = transport.watch().unwrap();

            transport.save(&[2]).unwrap();
            block_on(race(watcher.recv(), async {
                Timer::after(Duration::from_secs(3)).await;
                panic!("file change notification timed out");
            }))
            .unwrap();
            assert_eq!(transport.load().unwrap(), Some(vec![2]));
        }
    }
}
