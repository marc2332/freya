### border & border_align

You can add a border to an element using the `border` and `border_align` attributes.
- `border` syntax: `[width] [width?] [width?] [width?] <solid | none> [color]`.
- `border_align` syntax: `<inner | outer | center>`.

1-4 width values can be provided with the `border` attribute. Widths will be applied to different sides of a `rect` depending on the number of values:
- One value: `[all]`
- Two values: `[vertical]` `[horizontal]`
- Three values: `[top]` `[horizontal]` `[bottom]`
- Four values: `[top]` `[right]` `[bottom]` `[left]`

### Example

A solid, black border with a width of 2 pixels on every side. Border is aligned to the inside of the rect's bounding box.

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            border: "2 inner black",
            border_align: "inner"
        }
    )
}
```

Same as above, but with different border widths on each side.

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            border: "1 2 3 4 solid black",
            border_align: "inner"
        }
    )
}
```