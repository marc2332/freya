use freya::prelude::*;
use freya_radio::prelude::*;
use freya_testing::prelude::*;

#[derive(Default)]
struct Counter {
    count: i32,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash, PartialOrd, Ord)]
pub enum CounterChannel {
    Increment,
    Decrement,
    Reset,
}

impl RadioChannel<Counter> for CounterChannel {
    fn derive_channel(self, _radio: &Counter) -> Vec<Self> {
        match self {
            CounterChannel::Reset => vec![CounterChannel::Increment, CounterChannel::Decrement],
            c => vec![c],
        }
    }
}

#[test]
pub fn radio_basic_usage() {
    fn radio_app() -> impl IntoElement {
        use_init_radio_station::<Counter, CounterChannel>(Counter::default);
        let mut radio = use_radio::<Counter, CounterChannel>(CounterChannel::Increment);

        rect()
            .child(label().text(format!("Count: {}", radio.read().count)))
            .child(
                Button::new()
                    .on_press(move |_| {
                        radio.write().count += 1;
                    })
                    .child("Increment"),
            )
    }

    let mut test = launch_test(radio_app);
    test.sync_and_update();

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Count:"))
                .map(|_| node)
        })
        .unwrap();

    assert_eq!(
        Label::try_downcast(&*label.element()).unwrap().text,
        "Count: 0"
    );

    // Click the increment button
    test.click_cursor((25.0, 25.0));

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Count:"))
                .map(|_| node)
        })
        .unwrap();

    assert_eq!(
        Label::try_downcast(&*label.element()).unwrap().text,
        "Count: 1"
    );
}

#[test]
pub fn radio_multiple_subscribers_same_channel() {
    fn radio_app() -> impl IntoElement {
        use_init_radio_station::<Counter, CounterChannel>(Counter::default);

        rect()
            .child(IncrementButton {})
            .child(CounterDisplay {})
            .child(AnotherCounterDisplay {})
    }

    #[derive(PartialEq)]
    struct IncrementButton {}
    impl Render for IncrementButton {
        fn render(&self) -> impl IntoElement {
            let mut radio = use_radio::<Counter, CounterChannel>(CounterChannel::Increment);

            Button::new()
                .on_press(move |_| {
                    radio.write().count += 1;
                })
                .child("Increment")
        }
    }

    #[derive(PartialEq)]
    struct CounterDisplay {}
    impl Render for CounterDisplay {
        fn render(&self) -> impl IntoElement {
            let radio = use_radio::<Counter, CounterChannel>(CounterChannel::Increment);
            label().text(format!("Display 1: {}", radio.read().count))
        }
    }

    #[derive(PartialEq)]
    struct AnotherCounterDisplay {}
    impl Render for AnotherCounterDisplay {
        fn render(&self) -> impl IntoElement {
            let radio = use_radio::<Counter, CounterChannel>(CounterChannel::Increment);
            label().text(format!("Display 2: {}", radio.read().count))
        }
    }

    let mut test = launch_test(radio_app);
    test.sync_and_update();

    let labels = test.find_many(|node, element| {
        Label::try_downcast(element)
            .filter(|l| l.text.starts_with("Display"))
            .map(|_| node)
    });

    assert_eq!(labels.len(), 2);
    assert_eq!(
        Label::try_downcast(&*labels[0].element()).unwrap().text,
        "Display 1: 0"
    );
    assert_eq!(
        Label::try_downcast(&*labels[1].element()).unwrap().text,
        "Display 2: 0"
    );

    // Click the increment button - should notify both displays since they're on the Increment channel
    test.click_cursor((50.0, 20.0));

    let labels = test.find_many(|node, element| {
        Label::try_downcast(element)
            .filter(|l| l.text.starts_with("Display"))
            .map(|_| node)
    });

    // Both displays should update
    assert_eq!(
        Label::try_downcast(&*labels[0].element()).unwrap().text,
        "Display 1: 1"
    );
    assert_eq!(
        Label::try_downcast(&*labels[1].element()).unwrap().text,
        "Display 2: 1"
    );
}

#[test]
pub fn radio_channel_isolation() {
    fn radio_app() -> impl IntoElement {
        use_init_radio_station::<Counter, CounterChannel>(Counter::default);

        rect()
            .child(IncrementButton {})
            .child(DecrementButton {})
            .child(IncrementDisplay {})
            .child(DecrementDisplay {})
    }

    #[derive(PartialEq)]
    struct IncrementButton {}
    impl Render for IncrementButton {
        fn render(&self) -> impl IntoElement {
            let mut radio = use_radio::<Counter, CounterChannel>(CounterChannel::Increment);

            Button::new()
                .on_press(move |_| {
                    radio.write().count += 1;
                })
                .child("Increment")
        }
    }

    #[derive(PartialEq)]
    struct DecrementButton {}
    impl Render for DecrementButton {
        fn render(&self) -> impl IntoElement {
            let mut radio = use_radio::<Counter, CounterChannel>(CounterChannel::Decrement);

            Button::new()
                .on_press(move |_| {
                    radio.write().count -= 1;
                })
                .child("Decrement")
        }
    }

    #[derive(PartialEq)]
    struct IncrementDisplay {}
    impl Render for IncrementDisplay {
        fn render(&self) -> impl IntoElement {
            let radio = use_radio::<Counter, CounterChannel>(CounterChannel::Increment);
            label().text(format!("Inc: {}", radio.read().count))
        }
    }

    #[derive(PartialEq)]
    struct DecrementDisplay {}
    impl Render for DecrementDisplay {
        fn render(&self) -> impl IntoElement {
            let radio = use_radio::<Counter, CounterChannel>(CounterChannel::Decrement);
            label().text(format!("Dec: {}", radio.read().count))
        }
    }

    let mut test = launch_test(radio_app);
    test.sync_and_update();

    let inc_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Inc:"))
                .map(|_| node)
        })
        .unwrap();

    let dec_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Dec:"))
                .map(|_| node)
        })
        .unwrap();

    assert_eq!(
        Label::try_downcast(&*inc_label.element()).unwrap().text,
        "Inc: 0"
    );
    assert_eq!(
        Label::try_downcast(&*dec_label.element()).unwrap().text,
        "Dec: 0"
    );

    // Click increment button - only IncrementDisplay should update
    test.click_cursor((50.0, 20.0));

    let inc_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Inc:"))
                .map(|_| node)
        })
        .unwrap();

    let dec_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Dec:"))
                .map(|_| node)
        })
        .unwrap();

    // Only Inc should update to 1, Dec should stay at 0 (not notified)
    assert_eq!(
        Label::try_downcast(&*inc_label.element()).unwrap().text,
        "Inc: 1"
    );
    assert_eq!(
        Label::try_downcast(&*dec_label.element()).unwrap().text,
        "Dec: 0"
    );

    // Click decrement button - only DecrementDisplay should update
    test.click_cursor((50.0, 60.0));

    let inc_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Inc:"))
                .map(|_| node)
        })
        .unwrap();

    let dec_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Dec:"))
                .map(|_| node)
        })
        .unwrap();

    // Inc should stay at 1 (not notified), Dec should update to 0
    assert_eq!(
        Label::try_downcast(&*inc_label.element()).unwrap().text,
        "Inc: 1"
    );
    assert_eq!(
        Label::try_downcast(&*dec_label.element()).unwrap().text,
        "Dec: 0"
    );
}

#[test]
pub fn radio_derived_channels() {
    fn radio_app() -> impl IntoElement {
        use_init_radio_station::<Counter, CounterChannel>(Counter::default);

        rect()
            .child(ResetButton {})
            .child(IncrementDisplay {})
            .child(DecrementDisplay {})
    }

    #[derive(PartialEq)]
    struct ResetButton {}
    impl Render for ResetButton {
        fn render(&self) -> impl IntoElement {
            let mut radio = use_radio::<Counter, CounterChannel>(CounterChannel::Reset);

            Button::new()
                .on_press(move |_| {
                    radio.write().count = 0;
                })
                .child("Reset")
        }
    }

    #[derive(PartialEq)]
    struct IncrementDisplay {}
    impl Render for IncrementDisplay {
        fn render(&self) -> impl IntoElement {
            let radio = use_radio::<Counter, CounterChannel>(CounterChannel::Increment);
            label().text(format!("Inc: {}", radio.read().count))
        }
    }

    #[derive(PartialEq)]
    struct DecrementDisplay {}
    impl Render for DecrementDisplay {
        fn render(&self) -> impl IntoElement {
            let radio = use_radio::<Counter, CounterChannel>(CounterChannel::Decrement);
            label().text(format!("Dec: {}", radio.read().count))
        }
    }

    let mut test = launch_test(radio_app);
    test.sync_and_update();

    // Manually set count to something non-zero
    // We'll do this by clicking reset (which should set it to 0, but let's modify the test)
    // Actually, let's add increment/decrement buttons first

    let inc_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Inc:"))
                .map(|_| node)
        })
        .unwrap();

    let dec_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Dec:"))
                .map(|_| node)
        })
        .unwrap();

    assert_eq!(
        Label::try_downcast(&*inc_label.element()).unwrap().text,
        "Inc: 0"
    );
    assert_eq!(
        Label::try_downcast(&*dec_label.element()).unwrap().text,
        "Dec: 0"
    );

    // Click reset button - should notify both Increment and Decrement channels via derive_channel
    test.click_cursor((50.0, 20.0));

    let inc_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Inc:"))
                .map(|_| node)
        })
        .unwrap();

    let dec_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Dec:"))
                .map(|_| node)
        })
        .unwrap();

    // Both should still show 0 and both should have been notified (re-rendered)
    assert_eq!(
        Label::try_downcast(&*inc_label.element()).unwrap().text,
        "Inc: 0"
    );
    assert_eq!(
        Label::try_downcast(&*dec_label.element()).unwrap().text,
        "Dec: 0"
    );
}
