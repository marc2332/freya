# Lifecycle

Dioxus components can use hooks to manage certain lifecycle situations.

## Component creation
You can run certain logic when the component is created for the first time by using the `use_hook` hook.

```rs
fn app() -> Element {

    use_hook(|| {
        println!("Component running for the first time!");
    });

    rsx!(...)
}
```

## Component destroyment

Run some logic when the component is being destroyed.

```rs
fn app() -> Element {

    use_drop(|| {
        println!("Component is being dropped.");
    });

    rsx!(...)
}
```

## Side effects

Run some logic when a signal is changed.

```rs
fn app() -> Element {
    let mut signal = use_signal(|| 1);

    use_effect(move || {
        // Because we are reading this signal now the effect is subscribed to any change
        let value = signal();
        println!("Value of signal is {value}");
    });

    rsx!(...)
}
```

## Side effects with dependencies

Run some logic when a signal is changed.

```rs
fn app() -> Element {
    let mut signal = use_signal(|| 1);
    let mut other_signal = use_signal(|| 1);

    // Manually specify non-signal values that we might want to react to
    use_effect(use_reactive(&signal, |value| {
        println!("Value of signal is {value}");
    }));

    // When you need multiple values you can pass a tuple
    use_effect(use_reactive(&(signal, other_signal), |(value, other_signal)| {
        println!("Value of signals are {value} and {other_signal}");
    }));

    rsx!(...)
}
```