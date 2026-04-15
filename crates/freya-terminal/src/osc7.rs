//! OSC 7 (current working directory) extraction.
//!
//! alacritty's vte dispatcher silently drops OSC 7, so we run a second
//! `vte::Parser` whose `Perform` impl intercepts only that sequence.

use std::path::PathBuf;

use alacritty_terminal::vte::Perform;

/// Captures the payload of OSC 7 sequences seen on the byte stream.
#[derive(Default)]
pub(crate) struct CwdSink {
    latest: Option<String>,
}

impl CwdSink {
    pub(crate) fn take(&mut self) -> Option<String> {
        self.latest.take()
    }
}

impl Perform for CwdSink {
    fn osc_dispatch(&mut self, params: &[&[u8]], _bell_terminated: bool) {
        if params.len() >= 2
            && params[0] == b"7"
            && let Ok(url) = std::str::from_utf8(params[1])
        {
            self.latest = Some(url.to_owned());
        }
    }
}

/// Parse the URL payload of an OSC 7 sequence into a filesystem path.
/// Accepts `file:///path`, `file://host/path`, and bare paths.
pub(crate) fn parse_cwd_url(url: &str) -> PathBuf {
    let Some(stripped) = url.strip_prefix("file://") else {
        return PathBuf::from(url);
    };
    match stripped.split_once('/') {
        Some((_, path)) => PathBuf::from(format!("/{path}")),
        None => PathBuf::from(stripped),
    }
}

#[cfg(test)]
mod tests {
    use alacritty_terminal::vte::Parser as VteParser;

    use super::*;

    fn sniff(chunks: &[&[u8]]) -> Option<String> {
        let mut parser = VteParser::new();
        let mut sink = CwdSink::default();
        for chunk in chunks {
            parser.advance(&mut sink, chunk);
        }
        sink.take()
    }

    #[test]
    fn osc7_bel_in_one_chunk() {
        assert_eq!(
            sniff(&[b"prefix\x1b]7;file:///home/marc\x07tail"]).as_deref(),
            Some("file:///home/marc"),
        );
    }

    #[test]
    fn osc7_st_in_one_chunk() {
        assert_eq!(
            sniff(&[b"\x1b]7;file:///tmp\x1b\\"]).as_deref(),
            Some("file:///tmp"),
        );
    }

    #[test]
    fn osc7_split_across_chunks() {
        assert_eq!(
            sniff(&[b"\x1b]7;file://", b"/var", b"/log\x07"]).as_deref(),
            Some("file:///var/log"),
        );
    }

    #[test]
    fn ignores_other_oscs() {
        assert_eq!(sniff(&[b"\x1b]0;hello\x07"]), None);
        assert_eq!(sniff(&[b"\x1b]70;nope\x07"]), None);
    }
}
