# tiny-notify

Let one part of your program wait until another has something ready, instead of repeatedly checking for changes.

For example, background work can wake a screen that needs updating, or several parts of an app can wait for the next frame.

- **`Notify`** wakes one waiting caller.
- **`ArcNotify`** does the same when the work runs on another thread.
- **`BroadcastNotify`** wakes several callers waiting for the same change.

## Wait until a result is ready

Keep the result somewhere shared, then notify the code waiting to use it.

```rust
use std::cell::RefCell;

use futures_lite::future::{block_on, zip};
use tiny_notify::Notify;

fn main() {
    block_on(async {
        let results = RefCell::new(Vec::new());
        let ready = Notify::new();

        zip(
            async {
                ready.notified().await;
                assert_eq!(results.borrow_mut().pop(), Some("Finished"));
            },
            async {
                results.borrow_mut().push("Finished");
                ready.notify();
            },
        )
        .await;
    });
}
```

The waiting code pauses until the result is ready. It does not need a loop that keeps asking whether the work has finished.

A notification only says that something may need attention. It does not carry the result or count how many changes happened.

This example uses `futures-lite` to run the waiting code. `tiny-notify` itself needs no extra libraries.

```sh
cargo run -p tiny-notify --example notifications
```
