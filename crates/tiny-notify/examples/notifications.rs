use futures_lite::future::{
    block_on,
    zip,
};
use tiny_notify::{
    ArcNotify,
    BroadcastNotify,
    Notify,
};

fn main() {
    block_on(async {
        let notify = Notify::new();
        zip(notify.notified(), async { notify.notify() }).await;

        let notify = ArcNotify::new();
        notify.notify();
        notify.notified().await;

        let broadcast = BroadcastNotify::new();
        let first = broadcast.notified();
        let second = broadcast.notified();
        zip(zip(first, second), async { broadcast.notify() }).await;
    });
}
