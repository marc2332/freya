# Switch

A controlled **Switch** to indicate a enabled/disabled state.

### Usage

```rust
fn app(cx: Scope) -> Element {
    let enabled = use_state(cx, || false);

    let is_enabled = if *enabled.get() { "Yes" } else { "No" };

    render!(
        rect {
            width: "100%",
            height: "100%",
            padding: "50",
            label {
                color: "black",
                "Is enabled? {is_enabled}"
            }
            Switch {
                enabled: *enabled.get(),
                ontoggled: |_| {
                    enabled.set(!enabled.get());
                }
            }
        }
    )
}
```
