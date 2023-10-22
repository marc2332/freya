# Layout

Learn how the layout attributes work.

- [`width & height`](#width--height)
- [`min_width & min_height`](#min_width--min_height)
- [`max_width & max_height`](#max_width--max_height)
- [`Size units`](#size_units)
  - [`Logical pixels`](#logical-pixels)
  - [`Percentages`](#percentages)
  - [`calc()`](#calc)
- [`direction`](#direction)
- [`padding`](#padding)
- [`margin`](#margin)
- [`main_align & cross_align`](#main_align--cross_align)

> ⚠️ Freya's layout is still somewhat limited.

### width & height
All elements support both `width` and `height` attributes.

See syntax for [`Size Units`](#size-units).

##### Usage

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "red",
            width: "15",
            height: "50",
        }
    )
}
```

### min_width & min_height

`rect` supports specifying a minimum width and height, this can be useful if you use it alongside a percentage for the target size.

See syntax for [`Size Units`](#size-units).

##### Usage

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "red",
            min_width: "100",
            min_height: "100",
            width: "50%",
            height: "50%",
        }
    )
}
```

### max_width & max_height

`rect` supports specifying a maximum width and height.

See syntax for [`Size Units`](#size-units).

##### Usage

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "red",
            max_width: "50%",
            max_height: "50%",
            width: "500",
            height: "500",
        }
    )
}
```

### Size Units

#### Logical pixels

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "50",
            height: "33"
        }
    )
}
```

#### Percentages
Relative percentage to the parent equivalent value.

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "50%", // Half the parent
            height: "75%" // 3/4 the parent
        }
    )
}
```

#### `calc()`

For more complex logic you can use the `calc()` function.

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "calc(33% - 60 + 15%)", // (1/3 of the parent minus 60) plus 15% of parent
            height: "calc(100% - 10)" // 100% of the parent minus 10
        }
    )
}
```

### direction

Control how the inner elements will be stacked, possible values are `vertical` (default) and `horizontal`.

##### Usage

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100%",
            height: "100%",
            direction: "vertical",
            rect {
                width: "100%",
                height: "50%",
                background: "red"
            },
            rect {
                width: "100%",
                height: "50%",
                background: "green"
            }
        }
    )
}
```

### padding

Specify the inner paddings of an element. You can do so by three different ways, just like in CSS.

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            padding: "25" // 25 in all sides
            padding: "100 50" // 100 in top and bottom, and 50 in left and right
            padding: "5 7 3 9" // 5 in top, 7 in right, 3 in bottom and 9 in left
        }
    )
}

```

### main_align & cross_align

Control how the inner elements are positioned inside the element. You can combine it with the `direction` attribute to create complex flows.

Possible values for both attributes are:
- `start` (default): At the begining of the axis
- `center`: At the center of the axis
- `end`: At the end of the axis

When using the `vertical` direction, `main_align` will be the Y axis and `cross_align` will be the X axis. But when using the `horizontal` direction, the
`main_align` will be the X axis and the `cross_align` will be the Y axis.

Example on how to center the inner elements in both axis:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",
            rect {
                width: "50%",
                height: "50%",
                background: "red"
            },
        }
    )
}
```

### margin

Specify the margin of an element. You can do so by three different ways, just like in CSS.

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            margin: "25" // 25 in all sides
            margin: "100 50" // 100 in top and bottom, and 50 in left and right
            margin: "5 7 3 9" // 5 in top, 7 in right, 3 in bottom and 9 in left
        }
    )
}
```