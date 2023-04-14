use std::path::Path;

use crate::{iter::Segments, os::OsGroup};

use super::PathInner;

impl<OS: OsGroup> PathInner<OS> {
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
