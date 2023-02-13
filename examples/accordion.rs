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
        rect {
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
                }
            }
        }
    )
}

#[inline_props]
#[allow(non_snake_case)]
fn Accordion<'a>(cx: Scope<'a>, children: Element<'a>, summary: Element<'a>) -> Element<'a> {
    let open = use_state(cx, || false);
    let (node_ref, size) = use_node(cx);
    let height = if *open.get() {
        size.height + 15.0
    } else {
        0.0
    };
    render!(
        container {
            color: "white",
            padding: "20",
            radius: "3",
            width: "100%",
            height: "auto",
            background: "rgb(30, 30, 30)",
            onclick: |_| open.set(!*open.get()),
            summary
            container {
                width: "100%",
                height: "{height}",
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
    render!(
        children
    )
}

#[inline_props]
#[allow(non_snake_case)]
fn AccordionBody<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render!(
        rect {
            width: "100%",
            children
        }
    )
}