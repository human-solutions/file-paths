use crate::SLASH;

#[derive(Debug)]
pub struct Segments<'a> {
    path: &'a str,
    lengths: Vec<usize>,
    pos: Option<usize>,
    // start of the current str
    start: usize,
}

impl<'a> Segments<'a> {
    pub(crate) fn new(path: &'a str) -> Self {
        let lengths = path
            .split(SLASH)
            .filter(|s| !s.is_empty())
            .map(|s| s.len())
            .collect();
        Self {
            path,
            lengths,
            pos: None,
            start: 0,
        }
    }
}

impl<'a> Iterator for Segments<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = match self.pos {
            Some(pos) if pos + 1 >= self.lengths.len() => {
                self.pos = None;
                return None;
            }
            Some(pos) => pos + 1,
            None if self.start == 0 && !self.lengths.is_empty() => 0,
            None => return None,
        };
        self.pos = Some(pos);

        let start = self.start;
        let end = start + self.lengths[pos] as usize;
        self.start = end + 1;

        // println!(">> {start}..{end} - {}", self.path.len());
        Some(&self.path[start..end])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.lengths.len();
        let remaining = match self.pos {
            Some(pos) => len - pos,
            None => len,
        };
        (remaining, Some(remaining))
    }
}
impl<'a> ExactSizeIterator for Segments<'a> {}

impl<'a> DoubleEndedIterator for Segments<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let pos = match self.pos {
            Some(pos) if pos > 0 => pos - 1,
            Some(_) => return None,
            None if self.start == 0 => {
                self.pos = None;
                return None;
            }
            None => self.len() - 1,
        };

        self.pos = Some(pos);

        let end = self.start - 1;
        let start = end - self.lengths[pos] as usize;
        self.start = start;

        // println!("<< {start}..{end} - {}", self.path.path.len());
        Some(&self.path[start..end])
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
