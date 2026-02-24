use std::sync::atomic::{
    AtomicI32,
    Ordering,
};

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn writable_read_subscribes_state() {
    static READER_RENDER_COUNT: AtomicI32 = AtomicI32::new(0);

    #[derive(PartialEq)]
    struct WriterComponent {
        counter: State<i32>,
    }

    impl Component for WriterComponent {
        fn render(&self) -> impl IntoElement {
            let mut counter = self.counter;
            Button::new()
                .child("Increment")
                .on_press(move |_| *counter.write() += 1)
        }
    }

    #[derive(PartialEq)]
    struct ReaderComponent {
        value: Writable<i32>,
    }

    impl Component for ReaderComponent {
        fn render(&self) -> impl IntoElement {
            READER_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let value = self.value.read();
            label().text(format!("Value: {}", *value))
        }
    }

    fn app() -> impl IntoElement {
        let counter = use_state(|| 0);

        rect()
            .child(WriterComponent { counter })
            .child(ReaderComponent {
                value: counter.into_writable(),
            })
    }

    READER_RENDER_COUNT.store(0, Ordering::Relaxed);
    let mut test = launch_test(app);

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 2);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 3);
}

#[test]
fn writable_peek_does_not_subscribe_state() {
    static READER_RENDER_COUNT: AtomicI32 = AtomicI32::new(0);

    #[derive(PartialEq)]
    struct WriterComponent {
        counter: State<i32>,
    }

    impl Component for WriterComponent {
        fn render(&self) -> impl IntoElement {
            let mut counter = self.counter;
            Button::new()
                .child("Increment")
                .on_press(move |_| *counter.write() += 1)
        }
    }

    #[derive(PartialEq)]
    struct ReaderComponent {
        value: Writable<i32>,
    }

    impl Component for ReaderComponent {
        fn render(&self) -> impl IntoElement {
            READER_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let value = self.value.peek();
            label().text(format!("Value: {}", *value))
        }
    }

    fn app() -> impl IntoElement {
        let counter = use_state(|| 0);

        rect()
            .child(WriterComponent { counter })
            .child(ReaderComponent {
                value: counter.into_writable(),
            })
    }

    READER_RENDER_COUNT.store(0, Ordering::Relaxed);
    let mut test = launch_test(app);

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);
}

#[test]
fn writable_write_notifies_subscribers_state() {
    static READER1_RENDER_COUNT: AtomicI32 = AtomicI32::new(0);
    static READER2_RENDER_COUNT: AtomicI32 = AtomicI32::new(0);

    #[derive(PartialEq)]
    struct WriterComponent {
        value: Writable<i32>,
    }

    impl Component for WriterComponent {
        fn render(&self) -> impl IntoElement {
            let mut value = self.value.clone();
            Button::new()
                .child("Increment")
                .on_press(move |_| *value.write() += 1)
        }
    }

    #[derive(PartialEq)]
    struct ReaderComponent1 {
        value: Writable<i32>,
    }

    impl Component for ReaderComponent1 {
        fn render(&self) -> impl IntoElement {
            READER1_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let value = self.value.read();
            label().text(format!("Reader1: {}", *value))
        }
    }

    #[derive(PartialEq)]
    struct ReaderComponent2 {
        value: Writable<i32>,
    }

    impl Component for ReaderComponent2 {
        fn render(&self) -> impl IntoElement {
            READER2_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let value = self.value.read();
            label().text(format!("Reader2: {}", *value))
        }
    }

    fn app() -> impl IntoElement {
        let counter = use_state(|| 0);
        let writable = counter.into_writable();

        rect()
            .child(WriterComponent {
                value: writable.clone(),
            })
            .child(ReaderComponent1 {
                value: writable.clone(),
            })
            .child(ReaderComponent2 { value: writable })
    }

    READER1_RENDER_COUNT.store(0, Ordering::Relaxed);
    READER2_RENDER_COUNT.store(0, Ordering::Relaxed);
    let mut test = launch_test(app);

    assert_eq!(READER1_RENDER_COUNT.load(Ordering::Relaxed), 1);
    assert_eq!(READER2_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER1_RENDER_COUNT.load(Ordering::Relaxed), 2);
    assert_eq!(READER2_RENDER_COUNT.load(Ordering::Relaxed), 2);
}

#[test]
fn writable_write_if_conditional_notification_state() {
    static READER_RENDER_COUNT: AtomicI32 = AtomicI32::new(0);
    static CLICK_COUNT: AtomicI32 = AtomicI32::new(0);

    #[derive(PartialEq)]
    struct WriterComponent {
        value: Writable<i32>,
    }

    impl Component for WriterComponent {
        fn render(&self) -> impl IntoElement {
            let mut value = self.value.clone();
            Button::new().child("Try Change").on_press(move |_| {
                let click = CLICK_COUNT.fetch_add(1, Ordering::Relaxed);
                value.write_if(|mut v| {
                    if click >= 2 {
                        *v += 1;
                        true
                    } else {
                        false
                    }
                });
            })
        }
    }

    #[derive(PartialEq)]
    struct ReaderComponent {
        value: Writable<i32>,
    }

    impl Component for ReaderComponent {
        fn render(&self) -> impl IntoElement {
            READER_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let value = self.value.read();
            label().text(format!("Value: {}", *value))
        }
    }

    fn app() -> impl IntoElement {
        let counter = use_state(|| 0);
        let writable = counter.into_writable();

        rect()
            .child(WriterComponent {
                value: writable.clone(),
            })
            .child(ReaderComponent { value: writable })
    }

    READER_RENDER_COUNT.store(0, Ordering::Relaxed);
    CLICK_COUNT.store(0, Ordering::Relaxed);
    let mut test = launch_test(app);

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 2);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 3);
}

#[test]
fn readable_read_subscribes_state() {
    static READER_RENDER_COUNT: AtomicI32 = AtomicI32::new(0);

    #[derive(PartialEq)]
    struct WriterComponent {
        counter: State<i32>,
    }

    impl Component for WriterComponent {
        fn render(&self) -> impl IntoElement {
            let mut counter = self.counter;
            Button::new()
                .child("Increment")
                .on_press(move |_| *counter.write() += 1)
        }
    }

    #[derive(PartialEq)]
    struct ReaderComponent {
        value: Readable<i32>,
    }

    impl Component for ReaderComponent {
        fn render(&self) -> impl IntoElement {
            READER_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let value = self.value.read();
            label().text(format!("Value: {}", *value))
        }
    }

    fn app() -> impl IntoElement {
        let counter = use_state(|| 0);

        rect()
            .child(WriterComponent { counter })
            .child(ReaderComponent {
                value: counter.into_readable(),
            })
    }

    READER_RENDER_COUNT.store(0, Ordering::Relaxed);
    let mut test = launch_test(app);

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 2);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 3);
}

#[test]
fn readable_peek_does_not_subscribe_state() {
    static READER_RENDER_COUNT: AtomicI32 = AtomicI32::new(0);

    #[derive(PartialEq)]
    struct WriterComponent {
        counter: State<i32>,
    }

    impl Component for WriterComponent {
        fn render(&self) -> impl IntoElement {
            let mut counter = self.counter;
            Button::new()
                .child("Increment")
                .on_press(move |_| *counter.write() += 1)
        }
    }

    #[derive(PartialEq)]
    struct ReaderComponent {
        value: Readable<i32>,
    }

    impl Component for ReaderComponent {
        fn render(&self) -> impl IntoElement {
            READER_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let value = self.value.peek();
            label().text(format!("Value: {}", *value))
        }
    }

    fn app() -> impl IntoElement {
        let counter = use_state(|| 0);

        rect()
            .child(WriterComponent { counter })
            .child(ReaderComponent {
                value: counter.into_readable(),
            })
    }

    READER_RENDER_COUNT.store(0, Ordering::Relaxed);
    let mut test = launch_test(app);

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);
}

#[test]
fn readable_from_writable_subscribes() {
    static READER_RENDER_COUNT: AtomicI32 = AtomicI32::new(0);

    #[derive(PartialEq)]
    struct WriterComponent {
        value: Writable<i32>,
    }

    impl Component for WriterComponent {
        fn render(&self) -> impl IntoElement {
            let mut value = self.value.clone();
            Button::new()
                .child("Increment")
                .on_press(move |_| *value.write() += 1)
        }
    }

    #[derive(PartialEq)]
    struct ReaderComponent {
        value: Readable<i32>,
    }

    impl Component for ReaderComponent {
        fn render(&self) -> impl IntoElement {
            READER_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let value = self.value.read();
            label().text(format!("Value: {}", *value))
        }
    }

    fn app() -> impl IntoElement {
        let counter = use_state(|| 0);
        let writable = counter.into_writable();
        let readable: Readable<i32> = writable.clone().into();

        rect()
            .child(WriterComponent { value: writable })
            .child(ReaderComponent { value: readable })
    }

    READER_RENDER_COUNT.store(0, Ordering::Relaxed);
    let mut test = launch_test(app);

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 2);

    test.click_cursor((50.0, 20.0));

    assert_eq!(READER_RENDER_COUNT.load(Ordering::Relaxed), 3);
}

#[test]
fn mixed_read_peek_subscription_behavior() {
    static SUBSCRIBED_RENDER_COUNT: AtomicI32 = AtomicI32::new(0);
    static UNSUBSCRIBED_RENDER_COUNT: AtomicI32 = AtomicI32::new(0);

    #[derive(PartialEq)]
    struct WriterComponent {
        counter: State<i32>,
    }

    impl Component for WriterComponent {
        fn render(&self) -> impl IntoElement {
            let mut counter = self.counter;
            Button::new()
                .child("Increment")
                .on_press(move |_| *counter.write() += 1)
        }
    }

    #[derive(PartialEq)]
    struct SubscribedComponent {
        value: Writable<i32>,
    }

    impl Component for SubscribedComponent {
        fn render(&self) -> impl IntoElement {
            SUBSCRIBED_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let value = self.value.read();
            label().text(format!("Subscribed: {}", *value))
        }
    }

    #[derive(PartialEq)]
    struct UnsubscribedComponent {
        value: Writable<i32>,
    }

    impl Component for UnsubscribedComponent {
        fn render(&self) -> impl IntoElement {
            UNSUBSCRIBED_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let value = self.value.peek();
            label().text(format!("Unsubscribed: {}", *value))
        }
    }

    fn app() -> impl IntoElement {
        let counter = use_state(|| 0);
        let writable = counter.into_writable();

        rect()
            .child(WriterComponent { counter })
            .child(SubscribedComponent {
                value: writable.clone(),
            })
            .child(UnsubscribedComponent { value: writable })
    }

    SUBSCRIBED_RENDER_COUNT.store(0, Ordering::Relaxed);
    UNSUBSCRIBED_RENDER_COUNT.store(0, Ordering::Relaxed);
    let mut test = launch_test(app);

    assert_eq!(SUBSCRIBED_RENDER_COUNT.load(Ordering::Relaxed), 1);
    assert_eq!(UNSUBSCRIBED_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(SUBSCRIBED_RENDER_COUNT.load(Ordering::Relaxed), 2);
    assert_eq!(UNSUBSCRIBED_RENDER_COUNT.load(Ordering::Relaxed), 1);

    test.click_cursor((50.0, 20.0));

    assert_eq!(SUBSCRIBED_RENDER_COUNT.load(Ordering::Relaxed), 3);
    assert_eq!(UNSUBSCRIBED_RENDER_COUNT.load(Ordering::Relaxed), 1);
}
