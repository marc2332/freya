use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn parse_heading_and_paragraph() {
    fn app() -> impl IntoElement {
        rect().child(MarkdownViewer::new(
            "# Hello\n\nThis is **bold** and *italic* text.",
        ))
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    // Expect at least two paragraphs (heading and paragraph)
    let paragraphs = test.find_many(|node, element| Paragraph::try_downcast(element).map(|_| node));
    assert!(paragraphs.len() >= 2);

    // Check heading text
    assert!(paragraphs.iter().any(|p| {
        let para = Paragraph::try_downcast(&*p.element()).unwrap();
        para.spans
            .get(0)
            .map(|s| s.text == "Hello")
            .unwrap()
    }));

    // Check paragraph contains the text
    assert!(paragraphs.iter().any(|p| {
        let para = Paragraph::try_downcast(&*p.element()).unwrap();
        para.spans.iter().any(|s| s.text.contains("This is"))
    }));
}

#[test]
fn theme_font_sizes_applied() {
    let content = "# H1\n\n## H2\n\nParagraph text\n\n```rust\nlet x = 1;\n```\n\n- Item 1\n\n|H|C|\n|--|--|\n|Cell|Data|";

    fn app(content: &'static str) -> impl IntoElement {
        rect().child(
            MarkdownViewer::new(content)
                .heading_h1(40.0)
                .heading_h2(36.0)
                .paragraph_size(18.0)
                .code_font_size(13.0)
                .table_font_size(12.0),
        )
    }

    let mut test = launch_test(move || app(content));
    test.sync_and_update();

    // H1 heading
    let paragraphs = test.find_many(|node, element| Paragraph::try_downcast(element).map(|_| node));
    let h1 = paragraphs
        .iter()
        .find(|p| {
            let para = Paragraph::try_downcast(&*p.element()).unwrap();
            para.spans.get(0).map(|s| s.text == "H1").unwrap()
        })
        .expect("H1 not found");
    let para = Paragraph::try_downcast(&*h1.element()).unwrap();
    assert_eq!(
        para.text_style_data.font_size.map(|fs| fs.into()),
        Some(40.0)
    );

    // H2 heading
    let h2 = paragraphs
        .iter()
        .find(|p| {
            let para = Paragraph::try_downcast(&*p.element()).unwrap();
            para.spans.get(0).map(|s| s.text == "H2").unwrap()
        })
        .expect("H2 not found");
    let para_h2 = Paragraph::try_downcast(&*h2.element()).unwrap();
    assert_eq!(
        para_h2.text_style_data.font_size.map(|fs| fs.into()),
        Some(36.0)
    );

    // Paragraph font size
    let ptext = paragraphs
        .iter()
        .find(|p| {
            let para = Paragraph::try_downcast(&*p.element()).unwrap();
            para.spans.iter().any(|s| s.text.contains("Paragraph text"))
        })
        .expect("Paragraph text not found");
    let para_p = Paragraph::try_downcast(&*ptext.element()).unwrap();
    assert_eq!(
        para_p.text_style_data.font_size.map(|fs| fs.into()),
        Some(18.0)
    );

    // Code block (label)
    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));
    let code_label = labels
        .iter()
        .find(|l| {
            let lab = Label::try_downcast(&*l.element()).unwrap();
            lab.text.contains("let x = 1;")
        })
        .expect("code label not found");
    let lab = Label::try_downcast(&*code_label.element()).unwrap();
    assert_eq!(
        lab.text_style_data.font_size.map(|fs| fs.into()),
        Some(13.0)
    );

    // List bullet font size uses paragraph_size
    let bullet_label = labels
        .iter()
        .find(|l| Label::try_downcast(&*l.element()).unwrap().text == "•")
        .expect("bullet not found");
    let bullet = Label::try_downcast(&*bullet_label.element()).unwrap();
    assert_eq!(
        bullet.text_style_data.font_size.map(|fs| fs.into()),
        Some(18.0)
    );

    // Table header font size
    let header = paragraphs
        .iter()
        .find(|p| {
            let para = Paragraph::try_downcast(&*p.element()).unwrap();
            para.spans.get(0).map(|s| s.text == "H").unwrap()
        })
        .expect("table header not found");
    let para_h = Paragraph::try_downcast(&*header.element()).unwrap();
    assert_eq!(
        para_h.text_style_data.font_size.map(|fs| fs.into()),
        Some(12.0)
    );
}
