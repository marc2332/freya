# Native Router

Even though Freya supports Dioxus Router, it might due to Freya being it's own platform it misses certain integrations that users might expect from routing in an app, like support for back and forward buttons from mouses.

In order to use the native router you simply need to wrap your `Router` content inside the `NativeRouter` component.

Example (based on the example from [router](../router.md)):
```rs
#[allow(non_snake_case)]
fn AppSidebar() -> Element {
    rsx!(
        NativeRouter {
            Body {
                Link {
                    to: Route::Home,
                    label {
                        "Home"
                    }
                },
                Link {
                    to: Route::Other,
                    label {
                        "Other"
                    }
                },
                ...
            }
        }
    )
}

```