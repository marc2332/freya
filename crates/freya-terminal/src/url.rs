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

/// Column ranges `[start_col, end_col)` of clickable runs in `row`: OSC 8
/// hyperlinks attached by the terminal program plus plain-text URLs detected
/// by linkify.
pub(crate) fn link_ranges(row: &[Cell]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();

    let mut run_start: Option<usize> = None;
    for (col, cell) in row.iter().enumerate() {
        if cell.flags.contains(Flags::WIDE_CHAR_SPACER) {
            continue;
        }
        if cell.hyperlink().is_some() {
            run_start.get_or_insert(col);
        } else if let Some(start) = run_start.take() {
            ranges.push((start, col));
        }
    }
    if let Some(start) = run_start {
        ranges.push((start, row.len()));
    }

    if row_has_url_marker(row) {
        let (text, byte_to_col) = row_text(row);
        FINDER.with(|f| {
            for link in f.links(&text) {
                ranges.push((byte_to_col[link.start()], byte_to_col[link.end() - 1] + 1));
            }
        });
    }

    ranges
}

/// URL at column `col` in `row`, if any.
pub(crate) fn url_at(row: &[Cell], col: usize) -> Option<String> {
    if !row_has_url_marker(row) {
        return None;
    }
    let (text, byte_to_col) = row_text(row);
    FINDER.with(|f| {
        f.links(&text).find_map(|link| {
            let start = byte_to_col[link.start()];
            let end = byte_to_col[link.end() - 1] + 1;
            (col >= start && col < end).then(|| link.as_str().to_owned())
        })
    })
}

/// Cheap pre-scan: skips the row-text allocation when no `://` triplet exists in `row`.
fn row_has_url_marker(row: &[Cell]) -> bool {
    let (mut a, mut b) = ('\0', '\0');
    for cell in row
        .iter()
        .filter(|c| !c.flags.contains(Flags::WIDE_CHAR_SPACER))
    {
        if a == ':' && b == '/' && cell.c == '/' {
            return true;
        }
        a = b;
        b = cell.c;
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
        text.push(c);
        byte_to_col.resize(text.len(), col);
    }
    (text, byte_to_col)
}
