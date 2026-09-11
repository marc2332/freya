use freya::{
    html::*,
    prelude::*,
};

use crate::showcases::heading;

const HTML: &str = r#"
<html>
  <head>
    <style>
      body {
        margin: 0;
        font-family: sans-serif;
        color: #1a1a2e;
      }
      .card {
        margin-top: 16px;
        padding: 16px;
        border-radius: 12px;
        background: #f2f2f7;
      }
      .badge {
        display: inline-block;
        padding: 4px 10px;
        border-radius: 999px;
        background: linear-gradient(90deg, #e94560, #903749);
        color: white;
        font-size: 13px;
      }
      button {
        padding: 10px 20px;
        border: none;
        border-radius: 8px;
        background: #e94560;
        color: white;
        font-size: 15px;
        cursor: pointer;
      }
      button:hover { background: #c9314b; }
      button:active { background: #a02036; }
    </style>
  </head>
  <body>
    <h1 style="color: #e94560; margin: 0 0 8px 0;">Hello from Blitz</h1>
    <p style="font-size: 16px; line-height: 1.5;">
    Blitz in the browser!
    </p>

    <div class="card">
      <h2 style="margin: 0 0 12px 0;">Form controls</h2>
      <p>
        <label><input type="checkbox" checked> Check 1 </label>
        <label><input type="checkbox"> Check 2 </label>
      </p>
      <button>A button</button>
    </div>
  </body>
</html>
"#;

#[derive(PartialEq)]
pub struct HtmlShowcase;

impl Component for HtmlShowcase {
    fn render(&self) -> impl IntoElement {
        let handle = use_html_handle(|| HtmlSource::html(HTML));

        rect()
            .expanded()
            .spacing(20.)
            .child(heading("HTML", "HTML + CSS rendered by Blitz"))
            .child(HtmlViewer::new(handle).expanded())
    }
}
