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
        ScrollView {
            rect {
                label {
                    "Start Alignment"
                }
                rect {
                    width: "fill",
                    height: "100",
                    main_align: "start",
                    direction: "horizontal",
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                }
            }
            rect {
                label {
                    "Center Alignment"
                }
                rect {
                    width: "fill",
                    height: "100",
                    main_align: "center",
                    direction: "horizontal",
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                }
            }
            rect {
                label {
                    "End Alignment"
                }
                rect {
                    width: "fill",
                    height: "100",
                    main_align: "end",
                    direction: "horizontal",
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                }
            }
            rect {
                label {
                    "Space-between Alignment"
                }
                rect {
                    width: "fill",
                    height: "100",
                    main_align: "space-between",
                    direction: "horizontal",
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                }
            }

            rect {
                label {
                    "Space-around Alignment"
                }
                rect {
                    width: "fill",
                    height: "100",
                    main_align: "space-around",
                    direction: "horizontal",
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                }
            }
            rect {
                label {
                    "Space-evenly Alignment"
                }
                rect {
                    width: "fill",
                    height: "100",
                    main_align: "space-evenly",
                    direction: "horizontal",
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                    rect {
                        background: "black",
                        width: "50",
                        height: "50",
                        margin: "10",
                    }
                }
            }
        }
    )
}
