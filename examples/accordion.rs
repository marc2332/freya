#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        ScrollView {
            show_scrollbar: true,
            padding: "10",
            height: "100%",
            width: "100%",
            Accordion {
                summary: render!(AccordionSummary {
                    label {
                        "Accordion 1"
                    }
                }),
                AccordionBody {
                    label {
                        "This is the body"
                    }
                    label {
                        "This is the body"
                    }
                    label {
                        "This is the body"
                    }
                    label {
                        "This is the body"
                    }
                    label {
                        "This is the body"
                    }
                    label {
                        "This is the body"
                    }
                    label {
                        "This is the body"
                    }

                }
            }
            Accordion {
                summary: render!(AccordionSummary {
                    label {
                        "Accordion 2"
                    }
                }),
                AccordionBody {
                    label {
                        "This is the body"
                    }
                }
            }
            Accordion {
                summary: render!(AccordionSummary {
                    label {
                        "Accordion 3"
                    }
                }),
                AccordionBody {
                    label {
                        "This is the body"
                    }
                    label {
                        "This is the body"
                    }
                    label {
                        "This is the body"
                    }
                }
            }
        }
    )
}

#[inline_props]
#[allow(non_snake_case)]
fn Accordion<'a>(cx: Scope<'a>, children: Element<'a>, summary: Element<'a>) -> Element<'a> {
    let (start, set_value, value, animating) = use_animation_manager(cx, 0.0);
    let open = use_state(cx, || false);
    let (node_ref, size) = use_node(cx);

    // Adapt the accordtion if the body size changes
    use_effect(cx, &(size.width, size.height, animating), move |_| {
        if (size.height as f64) < value && !animating {
            set_value(size.height as f64 + 15.0);
        }
        async move {}
    });

    let onclick = move |_: MouseEvent| {
        let bodyHeight = size.height as f64 + 15.0;
        if *open.get() {
            start(AnimationMode::new_sine_in_out(bodyHeight..=0.0, 200));
        } else {
            start(AnimationMode::new_sine_in_out(0.0..=bodyHeight, 200));
        }
        open.set(!*open.get());
    };

    render!(
        container {
            color: "white",
            padding: "20",
            radius: "3",
            width: "100%",
            height: "auto",
            background: "rgb(30, 30, 30)",
            onclick: onclick,
            summary
            container {
                width: "100%",
                height: "{value}",
                rect {
                    height: "15"
                },
                rect {
                    reference: node_ref,
                    height: "auto",
                    width: "100%",
                    children
                }
            }
        }
    )
}

#[inline_props]
#[allow(non_snake_case)]
fn AccordionSummary<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render!(children)
}

#[inline_props]
#[allow(non_snake_case)]
fn AccordionBody<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render!(rect {
        width: "100%",
        children
    })
}
