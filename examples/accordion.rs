#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(
        Accordion {
            summary: rsx!(AccordionSummary {
                label {
                    "Accordion Summary"
                }
            }),
            AccordionBody {
                label {
                    "Accordion Body"
                }
            }
        }
    )
}
