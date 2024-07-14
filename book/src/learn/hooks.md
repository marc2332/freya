# Hooks

Hooks are special functions to be used inside of Components. They are usually prefixed with `use`, e.g `use_signal`, `use_effect`

# Rules of Hooks

## 1. They cannot be called conditionally

You cannot do the following because hooks need to maintain their order.

❌:
```rs
#[component]
fn MyComponent(value: bool) -> Element {
    let is_enabled = if value {
        use_signal(|| value)
    } else {
        true
    };

    rsx!(...)
}
```

✅:
```rs
#[component]
fn MyComponent(initial_value: bool) -> Element {
    let is_enabled = use_signal(|| initial_value)

    rsx!(...)
}
```

## 2. They can only be called inside of Component functions

You cannot call them inside of event handlers, futures, etc.

❌:
```rs
#[component]
fn MyComponent() -> Element {
    let onclick = |_| {
         let state = use_signal(|| false);
    };

    rsx!(
        label {
            onclick,
            "Hello, World!"
        }
    )
}
```

✅:
```rs
#[component]
fn MyComponent() -> Element {
    let state = use_signal(|| false);

    rsx!(...)
}
```

## 3. They cannot be called in loops


❌:
```rs
#[component]
fn MyComponent() -> Element {
    for i in 0..5 {
        let state = use_signal(|| i);
    }

    rsx!(...)
}
```

✅:
```rs
#[component]
fn MyComponent() -> Element {
    let state = use_signal(|| (0..5).iter().collect::<Vec<_>>());

    rsx!(...)
}
```