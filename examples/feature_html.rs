#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_html::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(900., 700.)))
}

const HTML: &str = r#"
<html>
  <head>
    <style>
      body {
        margin: 0;
        font-family: sans-serif;
        background: #ffffff;
        color: #1a1a2e;
      }
      .card {
        margin-top: 20px;
        padding: 16px;
        border-radius: 12px;
        background: #f2f2f7;
      }
      button, .button {
        display: inline-block;
        padding: 10px 20px;
        border: none;
        border-radius: 8px;
        background: #e94560;
        color: white;
        font-size: 15px;
        text-decoration: none;
        cursor: pointer;
      }
      button:hover, .button:hover {
        background: #c9314b;
      }
      button:active, .button:active {
        background: #a02036;
      }
      input[type="text"] {
        padding: 10px;
        border: 1px solid #c5c5d2;
        border-radius: 8px;
        font-size: 15px;
        width: 300px;
      }
    </style>
  </head>
  <body>
    <div style="padding: 32px;">
      <h1 style="color: #e94560; margin-bottom: 8px;">Hello from Blitz</h1>
      <p style="font-size: 16px; line-height: 1.5;">
        Everything below is plain HTML and CSS rendered by <b>Blitz</b> straight
        into Freya's canvas and without JavaScript.
      </p>

      <div class="card">
        <h2 style="margin: 0 0 12px 0;">Search the web</h2>
        <form action="https://duckduckgo.com/" method="get">
          <input type="text" name="q" placeholder="Type and press the button">
          <button type="submit">Search</button>
        </form>
        <p style="margin: 12px 0 0 0; color: #6e6e80;">
          Submitting navigates this view to the results page.
        </p>
      </div>

      <div class="card">
        <h2 style="margin: 0 0 12px 0;">Form controls</h2>
        <p>
          <label><input type="checkbox" checked> Check 1 </label>
          <label><input type="checkbox">  Check 2 </label>
        </p>
        <p>
          <label><input type="radio" name="engine" checked> Radio 1</label>
          <label><input type="radio" name="engine"> Radio 2</label>
        </p>
      </div>

      <div class="card">
        <h2 style="margin: 0 0 12px 0;">Visit</h2>
        <a class="button" href="https://freyaui.dev">Visit freyaui.dev</a>
        <a class="button" href="https://blitz.is/">Visit blitz.is/</a>
      </div>
    </div>
  </body>
</html>
"#;

fn app() -> impl IntoElement {
    let handle = use_html_handle(|| HtmlSource::html(HTML));

    rect()
        .expanded()
        .center()
        .background((30, 30, 40))
        .child(HtmlViewer::new(handle))
}
