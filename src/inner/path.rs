use anyhow::{ensure, Result};
use serde::Deserialize;
use std::ops::Range;
use std::{marker::PhantomData, path::Path};

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

    pub(crate) fn is_file(&self) -> bool {
        !self.is_dir()
    }

    pub(crate) fn is_dir(&self) -> bool {
        self.path.ends_with(SLASH) || self.path == "." || self.path == "~"
    }

    pub(crate) fn ensure_absolute(&self) -> Result<()> {
        ensure!(
            self.is_absolute(),
            "path is not absolute (it should start with a slash): {self}"
        );
        Ok(())
    }

    pub(crate) fn ensure_relative(&self) -> Result<()> {
        ensure!(
            self.is_relative(),
            "path is not relative (it should not start with a slash): {self}"
        );
        Ok(())
    }

    pub(crate) fn ensure_file(&self) -> Result<()> {
        ensure!(
            self.is_file(),
            "path is not a file (it should not end with a slash): {self}"
        );
        Ok(())
    }
    pub(crate) fn ensure_dir(&self) -> Result<()> {
        ensure!(
            self.is_dir(),
            "path is not a dir (it doesn't end with a slash): {self}"
        );
        Ok(())
    }

    pub(crate) fn relative_part(&self) -> &str {
        &self.path[OS::start_of_relative_path(&self.path)..]
    }

    pub(crate) fn relative_range(&self) -> usize {
        OS::start_of_relative_path(&self.path)
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

    fn file_name_start(&self) -> usize {
        let rel_start = self.relative_range();
        self.path.after_last_slash_from(rel_start)
    }

    pub(crate) fn file_name(&self) -> &str {
        &self.path[self.file_name_start()..]
    }

    pub(crate) fn set_file_name(&mut self, file_name: &str) -> Result<()> {
        file_name.assert_allowed_file_name()?;
        let file_start = self.file_name_start();
        self.path.truncate(file_start);
        self.path.push_str(file_name);
        Ok(())
    }

    pub(crate) fn with_file_name(&self, file_name: &str) -> Result<Self> {
        let mut me = self.clone();
        me.set_file_name(file_name)?;
        Ok(me)
    }

    pub(crate) fn file_stem_range(&self) -> Range<usize> {
        let rel = self.relative_range();
        let start = self.path.after_last_slash_from(rel);
        let end = self.path.first_dot_from(start);
        start..end
    }

    pub(crate) fn file_stem(&self) -> &str {
        &self.path[self.file_stem_range()]
    }

    pub(crate) fn set_file_stem(&mut self, file_stem: &str) -> Result<()> {
        ensure!(!file_stem.is_empty(), "An empty file stem is not valid");
        let range = self.file_stem_range();
        let mut path = self.path[..range.start].to_string();
        path.push_str(file_stem);
        path.push_str(&self.path[range.end..]);
        self.path = path;
        Ok(())
    }

    pub(crate) fn with_file_stem(&self, file_stem: &str) -> Result<Self> {
        let mut me = self.clone();
        me.set_file_stem(file_stem)?;
        Ok(me)
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
