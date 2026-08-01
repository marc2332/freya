//! Crate-local snapshot of a terminal cell.
//!
//! rio-vt stores cells as packed `Square`s whose colors, attributes,
//! hyperlinks and zero-width characters live in per-grid side tables.
//! The renderer and URL scanner work on plain row buffers, so each visible
//! row is resolved into `TermCell`s once per frame (mirroring the row
//! cloning the previous backend required).

use rio_vt::{
    config::colors::{
        AnsiColor,
        ColorRgb,
        NamedColor,
    },
    crosswords::{
        Crosswords,
        grid::ExtrasTable,
        square::{
            ContentTag,
            Square,
            Wide,
        },
        style::{
            Style,
            StyleFlags,
        },
    },
    event::EventListener,
};

/// A resolved terminal cell: character, colors and the attributes the
/// renderer consumes.
#[derive(Clone)]
pub(crate) struct TermCell {
    pub c: char,
    pub fg: AnsiColor,
    pub bg: AnsiColor,
    pub inverse: bool,
    pub wide: bool,
    pub wide_spacer: bool,
    pub zerowidth: Vec<char>,
    /// URI of the OSC 8 hyperlink attached to this cell, if any.
    pub hyperlink: Option<String>,
}

impl TermCell {
    pub(crate) fn from_square(square: &Square, styles: &[Style], extras: &ExtrasTable) -> Self {
        let (fg, bg, flags) = match square.content_tag() {
            ContentTag::Codepoint => {
                let style = styles
                    .get(square.style_id() as usize)
                    .copied()
                    .unwrap_or_default();
                (style.fg, style.bg, style.flags)
            }
            ContentTag::BgPalette => (
                AnsiColor::Named(NamedColor::Foreground),
                AnsiColor::Indexed(square.bg_palette_index()),
                StyleFlags::empty(),
            ),
            ContentTag::BgRgb => {
                let (r, g, b) = square.bg_rgb();
                (
                    AnsiColor::Named(NamedColor::Foreground),
                    AnsiColor::Spec(ColorRgb { r, g, b }),
                    StyleFlags::empty(),
                )
            }
        };

        let (zerowidth, hyperlink) = square
            .extras_id()
            .and_then(|id| extras.get(id))
            .map(|extra| {
                (
                    extra.zerowidth.clone(),
                    extra.hyperlink.as_ref().map(|h| h.uri().to_owned()),
                )
            })
            .unwrap_or_default();

        TermCell {
            c: square.c(),
            fg,
            bg,
            inverse: flags.contains(StyleFlags::INVERSE),
            wide: matches!(square.wide(), Wide::Wide),
            wide_spacer: matches!(square.wide(), Wide::Spacer | Wide::LeadingSpacer),
            zerowidth,
            hyperlink,
        }
    }
}

/// Resolve the viewport row at `line` (0 = top of the visible area) into
/// `TermCell`s, reusing `buf`.
pub(crate) fn snapshot_row<T: EventListener>(
    term: &Crosswords<T>,
    line: usize,
    buf: &mut Vec<TermCell>,
) {
    buf.clear();
    let rows = term.visible_rows();
    let Some(row) = rows.get(line) else {
        return;
    };
    let styles = term.grid.style_set.styles();
    let extras = &term.grid.extras_table;
    let columns = term.columns();
    buf.extend(
        (0..columns)
            .map(|col| TermCell::from_square(&row[rio_vt::crosswords::pos::Column(col)], styles, extras)),
    );
}
