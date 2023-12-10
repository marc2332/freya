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
            theme: theme_with!(
                ScrollViewTheme { padding : "5".into(), height : "100%".into(), width : "100%"
                .into(), }
            ),
            Accordion { summary: render!(AccordionSummary { label { "Accordion 1" } }),
                AccordionBody {
                    label { "This is the body" }
                    label { "This is the body" }
                    label { "This is the body" }
                    label { "This is the body" }
                    label { "This is the body" }
                    label { "This is the body" }
                    label { "This is the body" }
                }
            }
            Accordion { summary: render!(AccordionSummary { label { "Accordion 2" } }),
                AccordionBody { label { "This is the body" } }
            }
            Accordion { summary: render!(AccordionSummary { label { "Accordion 3" } }),
                AccordionBody {
                    label { "This is the body" }
                    label { "This is the body" }
                    label { "This is the body" }
                }
            }
        }
    )
}
