use std::path::Path;

use crate::iter::Segments;

use super::PathInner;

impl PathInner {
    pub fn segments(&self) -> Segments {
        Segments::new(self.relative_part())
    }

    pub fn as_str(&self) -> &str {
        &self.path
    }

    pub fn as_path(&self) -> &Path {
        Path::new(&self.path)
    }
}
