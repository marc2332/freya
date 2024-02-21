Specify how you want the automatic (e.g `width: auto`) bounds in the cross axis to be constrained for the inner elements.

Accepted values:

- `normal` (default): Uses parent bounds.
- `fit`: Uses parent bounds but later shrunks to the size of the biggest element inside.

The `fit` mode will allow the inner elements using `width: fill-min` to expand to the biggest element inside this element.

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            content: "fit",
            height: "100%",
            rect {
                width: "fill-min", // Will have a width of 300px
                height: "25%",
                background: "red",
            }
            rect {
                width: "150",  // Will have a width of 150px
                height: "25%",
                background: "green",
            }
            rect {
                width: "fill-min",  // Will have a width of 300px
                height: "25%",
                background: "blue",
            }
            rect {
                width: "300",  // Biggest element, will have a width of 300px
                height: "25%",
                background: "black",
            }
        }
    )
}
```
