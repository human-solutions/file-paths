use std::{path::Path, str::Chars};

use anyhow::{ensure, Result};

use crate::{
    ext::{PathExt, StrExt},
    iter::{InnerSegmentIter, Segments},
};

use super::{expand_envs, windows_drive};

#[derive(Debug)]
pub(crate) struct PathInner {
    pub(crate) path: String,
    pub(crate) lengths: Vec<u8>,
    pub(crate) is_absolute: bool,
}

impl PathInner {
    pub(crate) fn empty() -> Self {
        Self {
            path: String::new(),
            lengths: Vec::new(),
            is_absolute: false,
        }
    }

    pub(crate) fn new(path: &str) -> Result<Self> {
        let mut inner = PathInner::empty();

        let path = expand_envs(path)?;
        let (drive, path) = windows_drive(&path)?;
        if !drive.is_empty() {
            inner.set_root(&drive.to_string());
        }
        let mut iter = InnerSegmentIter::new(&path);

        while let Some(segment) = iter.next() {
            inner.push_segment(segment)?;
        }
        Ok(inner)
    }

    pub(crate) fn new_from_path(path: &Path) -> Result<Self> {
        Self::new(path.try_to_str()?)
    }

    fn set_root(&mut self, root: &str) {
        debug_assert!(self.path.is_empty());
        self.is_absolute = true;
        self.path.push_str(root);
        self.lengths.push(root.len() as u8);
    }

    fn push_segment(&mut self, segment: &str) -> Result<()> {
        segment.assert_allowed_path_component()?;
        if !self.lengths.is_empty() {
            self.path.push(crate::SEP);
        }
        self.path.push_str(segment);
        ensure!(
            segment.len() <= u8::MAX as usize,
            "path segments must be less than 255 characters, not: {segment}"
        );
        self.lengths.push(segment.len() as u8);
        Ok(())
    }

    pub fn chars(&self) -> Chars {
        self.path.chars()
    }

    pub fn segments(&self) -> Segments {
        Segments::new(self)
    }
}

#[test]
fn test_path_inner() {}
