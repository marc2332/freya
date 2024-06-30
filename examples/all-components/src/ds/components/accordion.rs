use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsAccordion() -> Element {
    rsx!(
        Accordion {
            summary: rsx!(AccordionSummary {
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
    )
}
