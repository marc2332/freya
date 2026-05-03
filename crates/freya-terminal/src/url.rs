use alacritty_terminal::term::cell::{
    Cell,
    Flags,
};
use linkify::{
    LinkFinder,
    LinkKind,
};

thread_local! {
    static FINDER: LinkFinder = {
        let mut f = LinkFinder::new();
        f.kinds(&[LinkKind::Url]);
        f
    };
}

/// Column ranges `[start_col, end_col)` of plain-text URLs in `row`.
pub(crate) fn url_ranges(row: &[Cell]) -> Vec<(usize, usize)> {
    if !row_has_url_marker(row) {
        return Vec::new();
    }
    let (text, byte_to_col) = row_text(row);
    FINDER.with(|f| {
        f.links(&text)
            .map(|link| col_range_for(&link, &byte_to_col))
            .collect()
    })
}

/// URL at column `col` in `row`, if any.
pub(crate) fn url_at(row: &[Cell], col: usize) -> Option<String> {
    if !row_has_url_marker(row) {
        return None;
    }
    let (text, byte_to_col) = row_text(row);
    FINDER.with(|f| {
        f.links(&text).find_map(|link| {
            let (start, end) = col_range_for(&link, &byte_to_col);
            (col >= start && col < end).then(|| link.as_str().to_owned())
        })
    })
}

fn col_range_for(link: &linkify::Link<'_>, byte_to_col: &[usize]) -> (usize, usize) {
    let start_col = byte_to_col[link.start()];
    let end_col = byte_to_col[link.end() - 1] + 1;
    (start_col, end_col)
}

/// Cheap pre-scan: skips the row-text allocation when no `://` triplet exists in `row`.
fn row_has_url_marker(row: &[Cell]) -> bool {
    let mut chars = row
        .iter()
        .filter(|c| !c.flags.contains(Flags::WIDE_CHAR_SPACER))
        .map(|c| c.c);
    let Some(mut a) = chars.next() else {
        return false;
    };
    let Some(mut b) = chars.next() else {
        return false;
    };
    for c in chars {
        if a == ':' && b == '/' && c == '/' {
            return true;
        }
        a = b;
        b = c;
    }
    false
}

/// Visible text for a row paired with a byte→column map. Wide-char spacers
/// are skipped to mirror the renderer's text layout.
fn row_text(row: &[Cell]) -> (String, Vec<usize>) {
    let mut text = String::with_capacity(row.len());
    let mut byte_to_col = Vec::with_capacity(row.len());
    for (col, cell) in row.iter().enumerate() {
        if cell.flags.contains(Flags::WIDE_CHAR_SPACER) {
            continue;
        }
        let c = match cell.c {
            '\0' | '\t' => ' ',
            c => c,
        };
        let mut buf = [0u8; 4];
        let bytes = c.encode_utf8(&mut buf).len();
        for _ in 0..bytes {
            byte_to_col.push(col);
        }
        text.push(c);
    }
    (text, byte_to_col)
}
