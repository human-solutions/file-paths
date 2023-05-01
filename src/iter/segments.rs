use crate::SLASH;

#[derive(Debug)]
pub struct Segments<'a> {
    path: &'a str,
    lengths: Vec<usize>,
    pos: isize,
}

impl<'a> Segments<'a> {
    pub(crate) fn new(path: &'a str) -> Self {
        let path = if path.ends_with(SLASH) {
            &path[..path.len() - 1]
        } else {
            path
        };
        let mut lengths = Vec::new();
        path.split_inclusive(SLASH)
            .filter(|s| !s.is_empty())
            .map(|s| s.len())
            .fold(0, |acc, val| {
                lengths.push(acc + val);
                acc + val
            });
        Self::new_with_lengths(path, lengths)
    }

    pub(super) fn new_with_lengths(path: &'a str, lengths: Vec<usize>) -> Self {
        Self {
            path,
            lengths,
            pos: -1,
        }
    }

    fn str_at_pos(&self, pos: usize) -> &'a str {
        let start = if pos == 0 { 0 } else { self.lengths[pos - 1] };
        let end = if pos == self.lengths.len() - 1 {
            self.lengths[pos]
        } else {
            self.lengths[pos] - 1
        };
        // println!(">> {start}..{end} - {}", self.path.len());
        &self.path[start..end]
    }
}

impl<'a> Iterator for Segments<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.lengths.len();
        let pos = if len == 0 {
            return None;
        } else if self.pos < 0 {
            0
        } else if self.pos + 1 >= len as isize {
            self.pos = len as isize;
            return None;
        } else {
            self.pos as usize + 1
        };

        self.pos = pos as isize;

        Some(self.str_at_pos(pos))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.lengths.len();
        let remaining = match self.pos {
            pos if pos < 0 || pos >= len as isize => len,
            pos => len - pos as usize,
        };
        (remaining, Some(remaining))
    }
}
impl<'a> ExactSizeIterator for Segments<'a> {}

impl<'a> DoubleEndedIterator for Segments<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.lengths.len();
        let pos = if len == 0 {
            return None;
        } else if self.pos >= len as isize {
            len - 1
        } else if self.pos - 1 < 0 {
            self.pos = -1;
            return None;
        } else {
            self.pos as usize - 1
        };

        self.pos = pos as isize;

        Some(self.str_at_pos(pos))
    }
}

#[cfg(test)]
use crate::{inner::PathInner, os::LinTestOS};

#[test]
fn test_path_iter() {
    let path = PathInner::<LinTestOS>::new("var/some/paths").unwrap();

    let mut iter = path.segments();
    assert_eq!(iter.next(), Some("var"));
    assert_eq!(iter.next(), Some("some"));
    assert_eq!(iter.next(), Some("paths"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some("paths"));
    assert_eq!(iter.next_back(), Some("some"));
    assert_eq!(iter.next_back(), Some("var"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);
}

#[test]
fn test_abs_path_iter() {
    let path = PathInner::<LinTestOS>::new("/var/some/paths").unwrap();

    let mut iter = path.segments();
    assert_eq!(iter.next(), Some("var"));
    assert_eq!(iter.next(), Some("some"));
    assert_eq!(iter.next(), Some("paths"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some("paths"));
    assert_eq!(iter.next_back(), Some("some"));
    assert_eq!(iter.next_back(), Some("var"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);
}
