use dioxus::prelude::*;
use freya_elements::NodeLayout;
use tokio::sync::{mpsc::channel, mpsc::Sender};

/// Creates a reference to the desired node's layout size
pub fn use_node(cx: &ScopeState) -> (&UseRef<Sender<NodeLayout>>, &UseState<NodeLayout>) {
    let status = use_state::<NodeLayout>(&cx, || NodeLayout::default());
    let status_getter = status.current();
    let status_setter = status.setter();
    let node_ref = use_ref(&cx, || {
        let (tx, rx) = channel::<NodeLayout>(1);

        (tx, Some(rx))
    });
    let sender = use_ref(&cx, || node_ref.read().0.clone());

    use_effect(&cx, (), move |()| {
        let node_ref = node_ref.clone();
        let getter = status_getter.clone();

        async move {
            let rx = node_ref.write().1.take();
            let mut rx = rx.unwrap();
            let mut prev_status = (*getter).clone();

            loop {
                if let Some(status) = rx.recv().await {
                    if prev_status != status {
                        status_setter(status.clone());
                        prev_status = status;
                    }
                }
            }
        }
    });

    (sender, status)
}
