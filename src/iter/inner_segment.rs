use std::iter::Peekable;
use std::str::Split;

pub struct InnerSegmentIter<'a> {
    iter: Peekable<Split<'a, [char; 2]>>,
}

impl<'a> InnerSegmentIter<'a> {
    pub(crate) fn new(path: &'a str) -> Self {
        Self {
            iter: path.split(['/', '\\']).peekable(),
        }
    }
}

impl<'a> Iterator for InnerSegmentIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.iter.next() {
            if self.iter.next_if_eq(&"..").is_some() {
                //skip if next is .. (which is skipped as well)
            } else if !next.is_empty() {
                return Some(next);
            }
        }
        None
    }
}

impl<'a> InnerSegmentIter<'a> {}

#[test]
fn test_path_iter() {
    assert_eq!(segs("./dir1//dir2/"), vec![".", "dir1", "dir2"]);
    assert_eq!(segs("./dir1/../dir2/"), vec![".", "dir2"]);
    assert_eq!(segs("./dir1/../../dir2/"), vec![".", "..", "dir2"]);
    assert_eq!(segs("./dir1/.."), vec!["."]);
    assert_eq!(segs(""), Vec::<&str>::new());
    assert_eq!(segs("s"), vec!["s"]);

    assert_eq!(segs(".\\"), vec!["."]);

    assert_eq!(segs("~/"), vec!["~"]);
}

#[cfg(test)]
fn segs(path: &str) -> Vec<String> {
    InnerSegmentIter::new(&path)
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}
