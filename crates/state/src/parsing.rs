use std::str::CharIndices;

// FromStr but we own it so we can impl it on torin and skia_safe types.
pub trait Parse: Sized {
    type Err;

    fn parse(value: &str) -> Result<Self, Self::Err>;
}

pub trait ExtSplit {
    fn split_excluding_group(
        &self,
        delimiter: char,
        group_start: char,
        group_end: char,
    ) -> SplitExcludingGroup<'_>;
    fn split_ascii_whitespace_excluding_group(
        &self,
        group_start: char,
        group_end: char,
    ) -> SplitAsciiWhitespaceExcludingGroup<'_>;
}

impl ExtSplit for str {
    fn split_excluding_group(
        &self,
        delimiter: char,
        group_start: char,
        group_end: char,
    ) -> SplitExcludingGroup<'_> {
        SplitExcludingGroup {
            text: self,
            chars: self.char_indices(),
            delimiter,
            group_start,
            group_end,
            trailing_empty: true,
        }
    }

    fn split_ascii_whitespace_excluding_group(
        &self,
        group_start: char,
        group_end: char,
    ) -> SplitAsciiWhitespaceExcludingGroup<'_> {
        SplitAsciiWhitespaceExcludingGroup {
            text: self,
            chars: self.char_indices(),
            group_start,
            group_end,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SplitExcludingGroup<'a> {
    pub text: &'a str,
    pub chars: CharIndices<'a>,
    pub delimiter: char,
    pub group_start: char,
    pub group_end: char,
    pub trailing_empty: bool,
}

impl<'a> Iterator for SplitExcludingGroup<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let first = self.chars.next();

        let (start, mut prev) = match first {
            None => {
                if self.text.ends_with(self.delimiter) && self.trailing_empty {
                    self.trailing_empty = false;
                    return Some("");
                }
                return None;
            }
            Some((_, c)) if c == self.delimiter => return Some(""),
            Some(v) => v,
        };

        let mut in_group = false;
        let mut nesting = -1;

        loop {
            if prev == self.group_start {
                if nesting == -1 {
                    in_group = true;
                }
                nesting += 1;
            } else if prev == self.group_end {
                nesting -= 1;
                if nesting == -1 {
                    in_group = false;
                }
            }

            prev = match self.chars.next() {
                None => return Some(&self.text[start..]),
                Some((end, c)) if c == self.delimiter && !in_group => {
                    return Some(&self.text[start..end])
                }
                Some((_, c)) => c,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SplitAsciiWhitespaceExcludingGroup<'a> {
    pub text: &'a str,
    pub chars: CharIndices<'a>,
    pub group_start: char,
    pub group_end: char,
}

impl<'a> Iterator for SplitAsciiWhitespaceExcludingGroup<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let first = self.chars.next();

        let (start, mut prev) = match first {
            None => return None,
            Some((_, c)) if c.is_ascii_whitespace() => return self.next(),
            Some(v) => v,
        };

        let mut in_group = false;
        let mut nesting = -1;

        loop {
            if prev == self.group_start {
                if nesting == -1 {
                    in_group = true;
                }
                nesting += 1;
            } else if prev == self.group_end {
                nesting -= 1;
                if nesting == -1 {
                    in_group = false;
                }
            }

            prev = match self.chars.next() {
                None => return Some(&self.text[start..]),
                Some((end, c)) if c.is_ascii_whitespace() && !in_group => {
                    return Some(&self.text[start..end])
                }
                Some((_, c)) => c,
            }
        }
    }
}
