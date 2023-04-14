use std::{marker::PhantomData, path::Path};

use anyhow::{ensure, Result};
use serde::Deserialize;

use crate::{
    ext::{PathExt, PathStrExt},
    iter::InnerSegmentIter,
    os::{self, OsGroup},
    SLASH,
};

#[derive(Clone, Deserialize)]
#[serde(transparent)]
pub(crate) struct PathInner<OS: OsGroup> {
    /// an absolute path is guaranteed to start with
    /// - on win: `<drive-letter>:\` or `\`
    /// - on *nix: `/`
    /// a path is guaranteed to have one and only one
    /// path separator (win: `\`, otherwise: `/`) per segment
    pub(crate) path: String,
    t: PhantomData<OS>,
}

impl<OS: OsGroup> PathInner<OS> {
    pub(crate) fn empty() -> Self {
        Self {
            path: String::new(),
            t: PhantomData,
        }
    }

    pub(crate) fn new(path: &str) -> Result<Self> {
        let mut inner = PathInner::empty();

        let path = os::expand::<OS>(path)?;

        let path = OS::process_drive_letter(&path, &mut inner.path)?;

        if path.starts_with(SLASH) {
            inner.path.push(OS::SEP)
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

    pub(super) fn as_contracted(&self, do_contract: bool) -> (Option<char>, &str) {
        if do_contract && self.is_absolute() {
            match os::contract::<OS>(&self.path) {
                Ok(s) => s,
                Err(_) => (None, &self.path),
            }
        } else {
            (None, self.path.as_str())
        }
    }

    pub(crate) fn is_absolute(&self) -> bool {
        OS::is_absolute(&self.path)
    }

    pub(crate) fn relative_part(&self) -> &str {
        OS::relative_part(&self.path)
    }

    pub(crate) fn push_segment(&mut self, segment: &str) -> Result<()> {
        segment.assert_allowed_path_component()?;
        if !self.path.is_empty() && !self.path.ends_with(OS::SEP) {
            self.path.push(OS::SEP);
        }
        self.path.push_str(segment);
        ensure!(
            segment.len() <= u8::MAX as usize,
            "path segments must be less than 255 characters, not: {segment}"
        );
        Ok(())
    }
}
