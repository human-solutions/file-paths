use std::str::Chars;

use crate::iter::Segments;

use super::PathInner;

impl PathInner {
    pub fn chars(&self) -> Chars {
        self.path.chars()
    }

    pub fn segments(&self) -> Segments {
        Segments::new(self.relative_part())
    }

    pub fn as_str(&self) -> &str {
        &self.path
    }
}
