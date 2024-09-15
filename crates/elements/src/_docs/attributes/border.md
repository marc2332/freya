### border

You can add borders to an element using the `border` attribute.
- `border` syntax: `[width] [width?] [width?] [width?] <inner | outer | center> [fill]`.

1-4 width values should be provided with the `border` attribute. Widths will be applied to different sides of a `rect` depending on the number of values provided:
- One value: `all`
- Two values: `vertical`, `horizontal`
- Three values: `top` `horizontal` `bottom`
- Four values: `top` `right` `bottom` `left`

*Border alignment* determines how the border is positioned relative to the element's edge. Alignment can be `inner`, `outer`, or `center`.

### Examples

A solid, black border with a width of 2 pixels on every side. Border is aligned to the inside of the rect's edge.

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            border: "2 inner black",
        }
    )
}
```

A solid, red border with different widths on each side. Border is aligned to the center of the rect's edge.

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            border: "1 2 3 4 center red",
        }
    )
}
```

Borders can take any valid fill type, including gradients.

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            border: "1 inner linear-gradient(red, green, yellow 40%, blue)",
        }
    )
}
```

Similarly to the `shadow` attribute, multiple borders can be drawn on a single element when separated by a comma. Borders specified later in the list are drawn on top of previous ones.

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            border: "6 outer red, 5 outer orange, 4 outer yellow, 3 outer green, 2 outer blue, 1 outer purple",
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