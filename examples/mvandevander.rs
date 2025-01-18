#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

// Replica of https://x.com/martin_cohen/status/1878476739317280901

fn main() {
    launch_with_props(
        app,
        "https://x.com/martin_cohen/status/1878476739317280901",
        (400.0, 350.0),
    );
}

fn app() -> Element {
    rsx!(
        rect {
            height: "100%",
            width: "100%",
            background: "white",
            border: "2 solid black",
            spacing: "2",
            padding: "5",
            content: "flex",
            rect {
                height: "70",
                width: "100%",
                background: "white",
                border: "2 solid black",
                content: "flex",
                spacing: "2",
                padding: "5",
                direction: "horizontal",
                rect {
                    height: "100%",
                    width: "200",
                    background: "white",
                    border: "2 solid black",
                    spacing: "2",
                    padding: "5",
                    direction: "horizontal",
                    rect {
                        height: "50",
                        width: "50",
                        background: "white",
                        border: "2 solid black"
                    }
                    rect {
                        height: "50",
                        width: "fill",
                        background: "white",
                        border: "2 solid black"
                    }
                }

                rect {
                    height: "100%",
                    width: "200",
                    background: "white",
                    border: "2 solid black",
                    spacing: "2",
                    padding: "5",
                    direction: "horizontal",
                    rect {
                        height: "50",
                        width: "50",
                        background: "white",
                        border: "2 solid black"
                    }
                    rect {
                        height: "50",
                        width: "fill",
                        background: "white",
                        border: "2 solid black"
                    }
                }

                rect {
                    height: "100%",
                    width: "200",
                    background: "white",
                    border: "2 solid black",
                    spacing: "2",
                    padding: "5",
                    direction: "horizontal",
                    rect {
                        height: "50",
                        width: "50",
                        background: "white",
                        border: "2 solid black"
                    }
                    rect {
                        height: "50",
                        width: "fill",
                        background: "white",
                        border: "2 solid black"
                    }
                }

                rect {
                    height: "100%",
                    width: "200",
                    background: "white",
                    border: "2 solid black",
                    spacing: "2",
                    padding: "5",
                    direction: "horizontal",
                    rect {
                        height: "50",
                        width: "50",
                        background: "white",
                        border: "2 solid black"
                    }
                    rect {
                        height: "50",
                        width: "fill",
                        background: "white",
                        border: "2 solid black",
                    }
                }

                rect {
                    height: "100%",
                    width: "flex(1)",
                    background: "white",
                    border: "2 solid black"
                }

                rect {
                    height: "100%",
                    width: "auto",
                    background: "white",
                    border: "2 solid black",
                    padding: "5",
                    rect {
                        height: "50",
                        width: "50",
                        background: "white",
                        border: "2 solid black"
                    }
                }

                rect {
                    height: "100%",
                    width: "auto",
                    background: "white",
                    border: "2 solid black",
                    padding: "5",
                    rect {
                        height: "50",
                        width: "50",
                        background: "white",
                        border: "2 solid black",
                        padding: "5",
                    }
                }

                rect {
                    height: "100%",
                    width: "auto",
                    background: "white",
                    border: "2 solid black",
                    padding: "5",
                    direction: "horizontal",
                    rect {
                        height: "50",
                        width: "50",
                        background: "white",
                        border: "2 solid black",
                    }
                }

                rect {
                    height: "100%",
                    width: "auto",
                    background: "white",
                    border: "2 solid black",
                    padding: "5",
                    direction: "horizontal",
                    rect {
                        height: "50",
                        width: "50",
                        background: "white",
                        border: "2 solid black",
                    }
                }
            }

            rect {
                height: "flex(1)",
                width: "100%",
                background: "white",
                border: "2 solid black",
                spacing: "2",
                padding: "4",
                direction: "horizontal",
                rect {
                    height: "100%",
                    width: "100%",
                    background: "white",
                    border: "2 solid black",
                    spacing: "2",
                    padding: "5",
                    direction: "horizontal",
                    rect {
                        height: "100%",
                        width: "70",
                        background: "white",
                        border: "2 solid black",
                        content: "flex",
                        spacing: "2",
                        padding: "5",

                        rect {
                            height: "auto",
                            width: "100%",
                            background: "white",
                            border: "2 solid black",
                            padding: "5",
                            rect {
                                height: "50",
                                width: "50",
                                background: "white",
                                border: "2 solid black",
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            background: "white",
                            border: "2 solid black",
                            padding: "5",
                            rect {
                                height: "50",
                                width: "50",
                                background: "white",
                                border: "2 solid black",
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            background: "white",
                            border: "2 solid black",
                            padding: "5",
                            rect {
                                height: "50",
                                width: "50",
                                background: "white",
                                border: "2 solid black",
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            background: "white",
                            border: "2 solid black",
                            padding: "5",
                            rect {
                                height: "50",
                                width: "50",
                                background: "white",
                                border: "2 solid black",
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            background: "white",
                            border: "2 solid black",
                            padding: "5",
                            rect {
                                height: "50",
                                width: "50",
                                background: "white",
                                border: "2 solid black",
                            }
                        }
                        rect {
                            height: "flex(1)",
                            width: "100%",
                            background: "white",
                            border: "2 solid black",
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            background: "white",
                            border: "2 solid black",
                            padding: "5",
                            rect {
                                height: "50",
                                width: "50",
                                background: "white",
                                border: "2 solid black"
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            background: "white",
                            border: "2 solid black",
                            padding: "5",
                            direction: "horizontal",
                            rect {
                                height: "50",
                                width: "50",
                                background: "white",
                                border: "2 solid black",
                            }
                        }
                    }
                    rect {
                        height: "100%",
                        width: "300",
                        background: "white",
                        border: "2 solid black",
                        spacing: "2",
                        padding: "5",
                        rect {
                            height: "100%",
                            width: "100%",
                            background: "white",
                            border: "2 solid black",
                            content: "flex",
                            spacing: "2",
                            padding: "5",
                            direction: "horizontal",
                            rect {
                                height: "50",
                                width: "50",
                                background: "white",
                                border: "2 solid black"
                            }
                            rect {
                                height: "50",
                                width: "flex(1)",
                                background: "white",
                                border: "2 solid black"
                            }
                            rect {
                                height: "50",
                                width: "50",
                                background: "white",
                                border: "2 solid black"
                            }
                        }
                    }
                    rect {
                        height: "100%",
                        width: "fill",
                        background: "white",
                        border: "2 solid black",
                    }
                }

            }
            rect {
                height: "auto",
                width: "100%",
                background: "white",
                border: "2 solid black",
                spacing: "2",
                padding: "5",
                direction: "horizontal",
                rect {
                    height: "auto",
                    width: "auto",
                    background: "white",
                    border: "2 solid black",
                    padding: "5",
                    rect {
                        height: "4",
                        width: "140",
                        background: "black",
                    }
                }
                rect {
                    height: "auto",
                    width: "auto",
                    background: "white",
                    border: "2 solid black",
                    padding: "5",
                    rect {
                        height: "4",
                        width: "140",
                        background: "black",
                    }
                }
                rect {
                    height: "auto",
                    width: "auto",
                    background: "white",
                    border: "2 solid black",
                    padding: "5",
                    rect {
                        height: "4",
                        width: "140",
                        background: "black",
                    }
                }

            }
        }
    )
}
