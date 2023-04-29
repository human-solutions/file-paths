use anyhow::{ensure, Result};
use serde::Deserialize;
use std::ops::Range;
use std::{marker::PhantomData, path::Path};

use crate::RelativeFolderPath;
use crate::{
    ext::{PathExt, PathStrExt},
    iter::{Extensions, InnerSegmentIter},
    os::{self, OsGroup},
    SLASH,
};

use super::StringValues;

#[derive(Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub(crate) struct PathInner<OS> {
    /// an absolute path is guaranteed to start with
    /// - on win: `<drive-letter>:\` or `\`
    /// - on *nix: `/`
    /// a path is guaranteed to have one and only one
    /// path separator (win: `\`, otherwise: `/`) per segment
    pub(crate) path: String,
    pub(crate) t: PhantomData<OS>,
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
        !self.is_folder()
    }

    pub(crate) fn is_folder(&self) -> bool {
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
    pub(crate) fn ensure_folder(&self) -> Result<()> {
        ensure!(
            self.is_folder(),
            "path is not a folder (it doesn't end with a slash): {self}"
        );
        Ok(())
    }

    pub(crate) fn relative_from(&self, segments: usize) -> Self {
        let path = self
            .segments()
            .skip(segments)
            .collect::<Vec<_>>()
            .join(&OS::SEP.to_string());

        Self { path, t: self.t }
    }

    pub(crate) fn relative_part(&self) -> &str {
        &self.path[OS::start_of_relative_path(&self.path)..]
    }

    pub(crate) fn relative_start(&self) -> usize {
        OS::start_of_relative_path(&self.path)
    }

    pub(crate) fn join(&mut self, dir: &RelativeFolderPath) {
        if !self.path.ends_with(OS::SEP) {
            self.path.push(OS::SEP);
        }
        self.path.push_str(&dir.0.path);
    }

    pub(crate) fn joining(&self, dir: &RelativeFolderPath) -> Self {
        let mut me = self.clone();
        me.join(dir);
        me
    }

    pub(crate) fn extensions(&self) -> Extensions {
        Extensions::new(&self.path)
    }

    pub(crate) fn set_extensions<E: StringValues>(&mut self, extensions: E) {
        if let Some(last_slash_index) = self.path.rfind(SLASH) {
            if let Some(first_dot_index) = self.path[last_slash_index..].find('.') {
                self.path.truncate(last_slash_index + first_dot_index);
            }
        } else if let Some(first_dot_index) = self.path.find('.') {
            self.path.truncate(first_dot_index)
        }
        let ext = extensions.join_strings(".");
        if ext.is_empty() {
            return;
        }
        self.path.push('.');
        self.path.push_str(&extensions.join_strings("."))
    }

    pub(crate) fn push_segment(&mut self, segment: &str) -> Result<()> {
        segment.assert_allowed_path_component()?;
        self.path.push_str(segment);
        Ok(())
    }

    pub(crate) fn push_segments<S: StringValues>(&mut self, segments: S) -> Result<()> {
        for i in 0..segments.string_count() {
            self.push_segment(segments.string_at(i))?;
        }
        Ok(())
    }

    pub fn pushing_segments<S: StringValues>(&self, segments: S) -> Result<Self> {
        let mut me = self.clone();
        me.push_segments(segments)?;
        Ok(me)
    }

    pub(crate) fn pop_last_segment(&mut self) {
        let rel_start = self.relative_start();
        let end = if self.path[rel_start..].ends_with(OS::SEP) {
            self.path[rel_start..self.path.len() - 1].rfind(OS::SEP)
        } else {
            self.path[rel_start..].rfind(OS::SEP)
        };
        if let Some(end) = end {
            self.path.truncate(rel_start + end);
        }
    }

    pub(crate) fn popping_last_segment(&self) -> Self {
        let mut me = self.clone();
        me.pop_last_segment();
        me
    }

    fn file_name_start(&self) -> usize {
        let rel_start = self.relative_start();
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
        let rel = self.relative_start();
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

    pub(crate) fn drop_file(&self) -> Self {
        let start = self.file_name_start();
        PathInner {
            path: self.path[..start].to_owned(),
            t: self.t,
        }
    }

    pub(crate) fn with_file_stem(&self, file_stem: &str) -> Result<Self> {
        let mut me = self.clone();
        me.set_file_stem(file_stem)?;
        Ok(me)
    }

    pub(crate) fn with_root(&self, root: &str) -> Self {
        PathInner {
            path: root.to_owned() + &self.path,
            t: self.t,
        }
    }
    fn with_path(&self, path: &str) -> Self {
        PathInner {
            path: path.to_string(),
            t: self.t,
        }
    }

    pub(crate) fn appending(&self, path: &str) -> Self {
        PathInner {
            path: self.path.clone() + path,
            t: self.t,
        }
    }

    pub(crate) fn remove_root(&self, root: &str) -> Option<Self> {
        self.path.strip_prefix(root).map(|s| self.with_path(s))
    }

    pub(crate) fn parent(&self) -> Option<Self> {
        let start = self.relative_start();

        let ends_with_slash = self.path[start..].ends_with(OS::SEP);
        let end = self.path.len() - if ends_with_slash { 1 } else { 0 };

        self.path[start..end].rfind(OS::SEP).map(|prev_sep| {
            let path = self.path[..prev_sep + 2].to_string();
            Self { path, t: self.t }
        })
    }
}
