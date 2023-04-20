use std::{marker::PhantomData, path::Path};

use anyhow::Result;
use serde::Deserialize;

use crate::{
    ext::{PathExt, PathStrExt},
    iter::{Extensions, InnerSegmentIter},
    os::{self, OsGroup},
    SLASH,
};

#[derive(Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub(crate) struct PathInner<OS> {
    /// an absolute path is guaranteed to start with
    /// - on win: `<drive-letter>:\` or `\`
    /// - on *nix: `/`
    /// a path is guaranteed to have one and only one
    /// path separator (win: `\`, otherwise: `/`) per segment
    pub(crate) path: String,
    t: PhantomData<OS>,
}

impl<OS> Clone for PathInner<OS> {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            t: self.t,
        }
    }
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
        let iter = InnerSegmentIter::new(path);

        for (segment, has_more) in iter {
            inner.push_segment(segment)?;
            if has_more {
                inner.path.push(OS::SEP);
            }
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

    pub(crate) fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    pub(crate) fn relative_part(&self) -> &str {
        OS::relative_part(&self.path)
    }

    pub(crate) fn is_file(&self) -> bool {
        !self.is_dir()
    }

    pub(crate) fn is_dir(&self) -> bool {
        self.path.ends_with(SLASH) || self.path == "." || self.path == "~"
    }

    pub(crate) fn join<S: AsRef<str>>(&self, path: S) -> Result<Self> {
        let iter = InnerSegmentIter::new(path.as_ref());
        let mut me: Self = self.clone();

        for (segment, has_more) in iter {
            me.push_segment(segment)?;
            if has_more {
                me.path.push(OS::SEP);
            }
        }
        Ok(me)
    }

    pub(crate) fn extensions(&self) -> Extensions {
        Extensions::new(&self.path)
    }

    pub(crate) fn set_extensions<E: FileExtensions>(&mut self, extensions: E) {
        if let Some(last_slash_index) = self.path.rfind(SLASH) {
            if let Some(first_dot_index) = self.path[last_slash_index..].find('.') {
                self.path.truncate(last_slash_index + first_dot_index);
            }
        } else if let Some(first_dot_index) = self.path.find('.') {
            self.path.truncate(first_dot_index)
        }
        let ext = extensions.join_ext();
        if ext.is_empty() {
            return;
        }
        self.path.push('.');
        self.path.push_str(&extensions.join_ext())
    }

    pub(crate) fn add_extension(&mut self, extension: &str) {
        if !self.path.ends_with('.') && !extension.starts_with('.') {
            self.path.push('.');
        }
        self.path.push_str(extension);
    }

    pub(crate) fn push_segment(&mut self, segment: &str) -> Result<()> {
        segment.assert_allowed_path_component()?;
        self.path.push_str(segment);
        Ok(())
    }

    pub(crate) fn file_name(&mut self) -> Option<&str> {
        let start = if self.path.ends_with(SLASH) {
            return None;
        } else if let Some(last_slash_index) = self.path.rfind(SLASH) {
            last_slash_index + 1
        } else {
            0
        };
        if start < self.path.len() {
            Some(&self.path[start..])
        } else {
            None
        }
    }
}

pub trait FileExtensions {
    fn join_ext(&self) -> String;
}

impl FileExtensions for &[&str] {
    fn join_ext(&self) -> String {
        self.join(".")
    }
}

impl FileExtensions for Vec<String> {
    fn join_ext(&self) -> String {
        self.join(".")
    }
}

impl FileExtensions for Vec<&str> {
    fn join_ext(&self) -> String {
        self.join(".")
    }
}

impl FileExtensions for String {
    fn join_ext(&self) -> String {
        self.clone()
    }
}

impl FileExtensions for &str {
    fn join_ext(&self) -> String {
        self.to_string()
    }
}
