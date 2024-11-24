#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use core::str;
use std::{
    fs,
    path::PathBuf,
};

use dioxus_router::{
    hooks::{
        use_navigator,
        use_route,
    },
    prelude::{
        Outlet,
        Routable,
        Router,
    },
};
use freya::prelude::*;
use rand::Rng;

fn main() {
    launch_with_props(app, "Documents Editor", (900.0, 750.0));
}

fn app() -> Element {
    let documents = use_context_provider(|| Signal::new(Vec::<Document>::new()));

    use_hook(move || {
        // Load the documents at launch
        spawn(async move {
            load_documents(documents).await;
        });
    });

    rsx!(Router::<Route> {})
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppSidebar)]
        #[route("/")]
        Home,
        #[nest("/document/:path")]
            #[layout(DocumentLayout)]
                #[route("/view")]
                DocumentView { path: String },
                #[route("/edit")]
                DocumentEdit { path: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[derive(Debug)]
struct Document(pub PathBuf);

impl Document {
    pub fn path(&self) -> String {
        self.0.to_str().unwrap().to_string()
    }

    pub fn route(&self, edit: bool) -> Route {
        if edit {
            Route::DocumentEdit { path: self.path() }
        } else {
            Route::DocumentView { path: self.path() }
        }
    }
}

async fn load_documents(mut documents: Signal<Vec<Document>>) {
    documents.clear();

    let Ok(mut paths) = tokio::fs::read_dir("./documents_example").await else {
        println!("Creating data folder...");
        tokio::fs::create_dir("./documents_example")
            .await
            .expect("Failed to create documents_example folder.");
        return;
    };

    println!("Loading documents from data folder...");

    while let Ok(Some(entry)) = paths.next_entry().await {
        let file_type = entry.file_type().await.unwrap();
        if file_type.is_file() {
            documents.push(Document(entry.path()))
        }
    }
}

#[component]
fn DocumentLayout(path: String) -> Element {
    let route = use_route::<Route>();
    let navigator = use_navigator();

    let (is_edit, path) = match route.clone() {
        Route::DocumentEdit { path } => (true, path),
        Route::DocumentView { path } => (false, path),
        _ => unreachable!(),
    };

    let switch_mode = move |_| {
        match &route {
            Route::DocumentEdit { path } => {
                navigator.push(Route::DocumentView { path: path.clone() })
            }
            Route::DocumentView { path } => {
                navigator.push(Route::DocumentEdit { path: path.clone() })
            }
            _ => unreachable!(),
        };
    };

    let note = if is_edit { "(editing)" } else { "" };

    rsx!(
        rect {
            padding: "20",
            spacing: "10",
            width: "fill",
            height: "fill",
            rect {
                direction: "horizontal",
                main_align: "space-between",
                cross_align: "center",
                width: "fill",
                label { "{path} {note}" }
                rect {
                    cross_align: "center",
                    spacing: "10",
                    Switch {
                        enabled: is_edit,
                        ontoggled: switch_mode
                    }
                    label {
                        "Toggle Mode"
                    }
                }
            }
            rect {
                corner_radius: "16",
                shadow: "0 0 6 0 rgb(0, 0, 0, 0.2)",
                padding: "24",
                width: "fill",
                height: "fill",
                Outlet::<Route> {  }
            }
        }
    )
}

#[component]
fn AppSidebar() -> Element {
    let documents: Signal<Vec<Document>> = use_context();

    rsx!(
        NativeRouter {
            Sidebar {
                sidebar: rsx!(
                    Link {
                        to: Route::Home,
                        ActivableRoute {
                            route: Route::Home,
                            exact: true,
                            SidebarItem {
                                label {
                                    "Home"
                                }
                            },
                        }
                    },
                    SidebarItem {
                        onclick: move |_| {
                            spawn(async move {
                                load_documents(documents).await;
                            });
                        },
                        label {
                            "Refresh documents"
                        }
                    },
                    ScrollView {
                        for document in documents.read().iter() {
                            Link {
                                key: "{document.path()}",
                                to: document.route(false),
                                ActivableRoute {
                                    route: document.route(false),
                                    exact: false,
                                    SidebarItem {
                                        label {
                                            "{document.path()}"
                                        }
                                    },
                                }
                            }
                        },
                    }
                ),
                Body {
                    rect {
                        main_align: "center",
                        cross_align: "center",
                        width: "100%",
                        height: "100%",
                        Outlet::<Route> {  }
                    }
                }
            }
        }
    )
}

#[component]
fn Home() -> Element {
    let navigator = use_navigator();

    let edit_blank = move |_| {
        let mut rng = rand::thread_rng();
        let path = format!("documents_example/doc-{}", rng.gen::<usize>());
        navigator.push(Route::DocumentEdit { path });
    };

    rsx!(
        Button {
            onclick: edit_blank,
            label {
                "Blank Document"
            }
        }
    )
}

#[component]
fn DocumentView(path: ReadOnlySignal<String>) -> Element {
    let content = use_resource(move || async move { fs::read_to_string(&*path.read()) });
    let content = content.read();

    if let Some(Ok(text)) = &*content {
        rsx!(
            label {
                "{text}"
            }
        )
    } else if content.is_none() {
        rsx!(
            label {
                "Loading document."
            }
        )
    } else {
        rsx!(
            label {
                "Error while loading document."
            }
        )
    }
}

static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[component]
fn DocumentEdit(path: String) -> Element {
    let documents: Signal<Vec<Document>> = use_context();
    let editable = use_editable(
        || {
            EditableConfig::new(
                fs::read_to_string(&path).unwrap_or_else(|_| LOREM_IPSUM.to_string()),
            )
            .with_allow_tabs(false)
        },
        EditableMode::MultipleLinesSingleEditor,
    );

    let save_file = {
        to_owned![path];
        move |_| {
            to_owned![path];
            spawn(async move {
                let file_content = editable.editor().read().rope().to_string();
                tokio::fs::write(path, file_content)
                    .await
                    .expect("Failed to save document.");
                load_documents(documents).await;
            });
        }
    };

    rsx!(
        rect {
            spacing: "10",
            rect {
                height: "calc(90% - 10)",
                DocumentEditor {
                    path,
                    editable
                }
            }
            rect {
                height: "calc(10% - 10)",
                width: "fill",
                Button {
                    theme: theme_with!(ButtonTheme {
                        width: "fill".into(),
                        padding: "10".into()
                    }),
                    onclick: save_file,
                    label {
                        width: "100%",
                        "Save"
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn PageNotFound() -> Element {
    rsx!(
        label {
            "404"
        }
    )
}

#[component]
fn DocumentEditor(path: String, mut editable: UseEditable) -> Element {
    let cursor_reference = editable.cursor_attr();
    let highlights = editable.highlights_attr(0);
    let editor = editable.editor().read();
    let cursor_char = editor.cursor_pos();

    let onmousedown = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseDown(e.data, 0));
    };

    let onmousemove = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseMove(e.data, 0));
    };

    let onclick = move |_: MouseEvent| {
        editable.process_event(&EditableEvent::Click);
    };

    let onglobalkeydown = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyDown(e.data));
    };

    let onglobalkeyup = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyUp(e.data));
    };

    rsx!(
        rect {
            width: "fill",
            height: "fill",
            cursor_reference,
            ScrollView {
                scroll_with_arrows: false,
                paragraph {
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_char}",
                    cursor_mode: "editable",
                    cursor_color: "black",
                    highlights,
                    onclick,
                    onmousemove,
                    onmousedown,
                    onglobalkeydown,
                    onglobalkeyup,
                    text {
                        "{editable.editor()}"
                    }
                }
            }
        }
    )
}
