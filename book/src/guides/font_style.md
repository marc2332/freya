# Font Style

Learn how the font style attributes work.

- [`color`](#color)
- [`font_family`](#font_family)
- [`font_size`](#font_size)
- [`align`](#align)
- [`font_style`](#font_style)
- [`font_weight`](#font_weight)
- [`font_width`](#font_width)
- [`line_height`](#line_height)
- [`max_lines`](#max_lines)


### color

The `color` attribute let's you specify the color of the text.

You can learn about the syntax of this attribute [here](/guides/style.html#color-syntax).

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            color: "green",
            "Hello, World!"
        }
    )
}
```

Another example showing [inheritance](#inheritance):

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            color: "blue",
            label {
                "Hello, World!"
            }
        }
    )
}

```

Compatible elements: [`label`](/guides/elements.html#label), [`paragraph`](/guides/elements.html#paragraph-and-text), [`text`](/guides/elements.html#paragraph-and-text)

### font_family

With the `font_family` you can specify what font do you want to use for the inner text.

Limitation: Only fonts installed in the system are supported for now.

Example: 

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_family: "Inter",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`label`](/guides/elements.html#label), [`paragraph`](/guides/elements.html#paragraph-and-text), 

### font_size

You can specify the size of the text using `font_size`.

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_size: "50",
            "Hellooooo!"
        }
    )
}
```

Compatible elements: [`label`](/guides/elements.html#label), [`paragraph`](/guides/elements.html#paragraph-and-text), [`text`](/guides/elements.html#paragraph-and-text)

### align

You can change the alignment of the text using the `align` attribute.

Accepted values: `center`, `end`, `justify`, `left`, `right`, `start`

Example

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            align: "right",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`label`](/guides/elements.html#label), [`paragraph`](/guides/elements.html#paragraph-and-text), 

### font_style

You can choose a style for a text using the `font_style` attribute.

Accepted values: `upright` (default), `italic` and `oblique`.

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_style: "italic",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).

### font_weight

You can choose a weight for a text using the `font_weight` attribute.

Accepted values:
- `invisible`
- `thin`
- `extra-light`
- `light`
- `normal` (default)
- `medium`
- `semi-bold`
- `bold`
- `extra-bold`
- `black`
- `extra-black`
- `50`
- `100`
- `200`
- `300`
- `400`
- `500`
- `600`
- `700`
- `800`
- `900`
- `950`

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_weight: "bold",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).

### font_width

You can choose a width for a text using the `font_width` attribute.

Accepted values:
- `ultra-condensed`
- `extra-condensed`
- `condensed`
- `normal` (default)
- `semi-expanded`
- `expanded`
- `extra-expanded`
- `ultra-expanded`

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_weight: "bold",
            "Hello, World!"
        }
    )
}
```


Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).


### line_height

Specify the height of the lines of the text.

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            lines_height: "3",
            "Hello, World! \n Hello, again!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`paragraph`](/guides/elements.html#paragraph-and-text).
