use freya_clipboard::prelude::*;
use futures_util::FutureExt;

#[test]
fn clipboard_reads_multiple_formats() {
    let clipboard = MemoryClipboard::default();
    let data = TransferData::text("Hello")
        .with(DataFormat::Html, DataValue::Bytes(b"<b>Hello</b>".to_vec()))
        .with(DataFormat::Image, DataValue::Bytes(vec![1, 2, 3]));

    assert_eq!(clipboard.set(data).now_or_never().unwrap(), Ok(()));
    assert_eq!(
        clipboard.get(DataFormat::Text).now_or_never().unwrap(),
        Ok(DataValue::Text("Hello".into()))
    );
    assert_eq!(
        clipboard.get(DataFormat::Html).now_or_never().unwrap(),
        Ok(DataValue::Text("<b>Hello</b>".into()))
    );
    assert_eq!(
        clipboard.get(DataFormat::Image).now_or_never().unwrap(),
        Ok(DataValue::Bytes(vec![1, 2, 3]))
    );
}

#[test]
fn replacing_clipboard_removes_previous_formats() {
    let clipboard = MemoryClipboard::default();
    clipboard
        .set(TransferData::default().with(DataFormat::Html, DataValue::Text("<b>Old</b>".into())))
        .now_or_never()
        .unwrap()
        .unwrap();
    clipboard
        .set(TransferData::text("New"))
        .now_or_never()
        .unwrap()
        .unwrap();

    assert_eq!(
        clipboard.get(DataFormat::Html).now_or_never().unwrap(),
        Err(ClipboardError::UnsupportedFormat)
    );
    assert_eq!(
        clipboard.get(DataFormat::Text).now_or_never().unwrap(),
        Ok(DataValue::Text("New".into()))
    );
}
