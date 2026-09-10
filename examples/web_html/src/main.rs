use freya::{
    html::*,
    prelude::*,
};

const NOTO_SANS: &[u8] = include_bytes!("../../../crates/freya-edit/tests/NotoSans-Regular.ttf");

const START_PAGE: &str = r#"
<html>
  <head>
    <style>
      body { margin: 0; padding: 32px; font-family: sans-serif; color: #1a1a2e; }
      code {
        padding: 2px 6px;
        border-radius: 6px;
        background: #f2f2f7;
      }
    </style>
  </head>
  <body>
    <h1 style="color: #e94560;">A browser inside a browser</h1>
    <p style="font-size: 16px; line-height: 1.5;">
      This page is rendered by <b>Blitz</b> into Freya's canvas, running as
      WebAssembly. Type a URL above to navigate, requests are fetched by your
      actual browser, so only sites that allow cross-origin requests will load.
    </p>
    <p style="font-size: 16px;">
      Sites that block cross-origin requests can be reached through this
      example's dev server, which relays them. Try
      <code>http://localhost:8772/proxy/https://news.ycombinator.com/</code>
    </p>
  </body>
</html>
"#;

fn main() {
    freya::web::launch(freya::web::WebConfig::new(app).with_font("Noto Sans", NOTO_SANS));
}

fn app() -> impl IntoElement {
    let mut handle = use_html_handle(|| HtmlSource::html(START_PAGE));
    let mut input = use_state(String::new);

    // Keep the input in sync when the page navigates.
    use_side_effect_with_deps(&handle.current_url(), move |url| {
        if let Some(url) = url {
            input.set(url.clone());
        }
    });

    rect()
        .expanded()
        .child(
            rect()
                .horizontal()
                .width(Size::fill())
                .cross_align(Alignment::Center)
                .spacing(4.)
                .padding(4.)
                .child(
                    Button::new()
                        .child("Back")
                        .enabled(handle.can_go_back())
                        .on_press(move |_| handle.back()),
                )
                .child(
                    Button::new()
                        .child("Forward")
                        .enabled(handle.can_go_forward())
                        .on_press(move |_| handle.forward()),
                )
                .child(
                    Button::new()
                        .child("Reload")
                        .on_press(move |_| handle.reload()),
                )
                .child(
                    Input::new(input)
                        .flat()
                        .width(Size::fill())
                        .placeholder("Enter a URL")
                        .on_submit(move |value: String| handle.navigate(value)),
                ),
        )
        .child(HtmlViewer::new(handle))
}
