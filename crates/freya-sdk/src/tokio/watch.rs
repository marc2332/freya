use freya_core::prelude::{
    ReactiveContext,
    spawn,
    use_hook,
};
use tokio::sync::watch;

/// Subscribe this component to the given [watch::Receiver].
pub fn use_track_watcher<T: 'static>(watcher: &watch::Receiver<T>) {
    use_hook(|| {
        let mut watcher = watcher.clone();

        // No need to make this component rerun if it just got created
        watcher.mark_unchanged();

        let rc = ReactiveContext::current().unwrap();

        spawn(async move {
            while watcher.changed().await.is_ok() {
                rc.notify();
            }
        });
    })
}
