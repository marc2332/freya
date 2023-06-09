# Switch

The `switch` component, also known as `toggle` in other GUI libraries, represents two states, `enabled` and `disabled`.

Example:

```rust
// Every time the user clicks on the Switch,
// `ontoggled` will be called, `is_enabled` 
// will be updated with the opposite of the
// current value, and finally, the switch 
// will be shown with the new state

fn app() -> Element {
    let is_enabled = use_state(cx, || false);

    let ontoggled = |_| {
        is_enabled.set(!is_enabled.get());
    };

    render!(
        Switch {
            enabled: *is_enabled.get(),
            ontoggled: ontoggled,
        }
    )
}
```
