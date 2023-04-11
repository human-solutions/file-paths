use std::{
    fmt::{Debug, Display},
    path::Path,
    str::Chars,
};

use anyhow::{ensure, Result};

use crate::{
    ext::{PathExt, StrExt},
    iter::{InnerSegmentIter, Segments},
    SEP, SLASH,
};

use super::{envs::contract_envs, expand_envs};

pub(crate) struct PathInner {
    /// an absolute path is guaranteed to start with
    /// - on win: `<drive-letter>:\` or `\`
    /// - on *nix: `/`
    /// a path is guaranteed to have one and only one
    /// path separator (win: `\`, otherwise: `/`) per segment
    pub(crate) path: String,
}

impl AsRef<Path> for PathInner {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}

impl Display for PathInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (chr, path) = self.as_contracted(!f.alternate());
        if let Some(chr) = chr {
            write!(f, "{chr}{SEP}")?;
        }
        write!(f, "{path}")
    }
}

impl Debug for PathInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (chr, path) = self.as_contracted(!f.alternate());
        #[cfg(windows)]
        let path = path.replace('/', "\\");
        if let Some(chr) = chr {
            write!(f, "{chr}{SEP}")?;
        }
        write!(f, "{path}")
    }
}
impl PathInner {
    pub(crate) fn empty() -> Self {
        Self {
            path: String::new(),
        }
    }

    pub(crate) fn new(path: &str) -> Result<Self> {
        let mut inner = PathInner::empty();

        let path = expand_envs(path)?;
        let mut path = if let Some(_drive) = win_drive(&path) {
            #[cfg(windows)]
            {
                inner.path.push(_drive);
                inner.path.push(':');
            }
            &path[2..]
        } else {
            &path
        };

        if path.starts_with(SLASH) {
            inner.path.push(SEP);
            path = &path[1..];
        }
        let mut iter = InnerSegmentIter::new(path);

        while let Some(segment) = iter.next() {
            inner.push_segment(segment)?;
        }
        Ok(inner)
    }

    pub(crate) fn new_from_path(path: &Path) -> Result<Self> {
        Self::new(path.try_to_str()?)
    }

    fn as_contracted(&self, do_contract: bool) -> (Option<char>, &str) {
        if do_contract && self.is_absolute() {
            match contract_envs(&self.path) {
                Ok(s) => s,
                Err(_) => (None, &self.path),
            }
        } else {
            (None, self.path.as_str())
        }
    }

    pub(crate) fn is_absolute(&self) -> bool {
        #[cfg(windows)]
        return self.path.starts_with('\\') || (self.path.len() > 3 && &self.path[1..3] == ":\\");
        #[cfg(not(windows))]
        return self.path.starts_with('/');
    }

    pub(crate) fn relative_part(&self) -> &str {
        #[cfg(windows)]
        {
            if self.path.starts_with(SEP) {
                &self.path[1..]
            } else if self.path.len() > 3 && &self.path[1..3] == ":\\" {
                &self.path[3..]
            } else {
                self.path.as_ref()
            }
        }
        #[cfg(not(windows))]
        if self.path.starts_with(SEP) {
            &self.path[1..]
        } else {
            self.path.as_ref()
        }
    }

    fn push_segment(&mut self, segment: &str) -> Result<()> {
        segment.assert_allowed_path_component()?;
        if !self.path.is_empty() && !(self.path.len() == 1 && self.path.starts_with(SEP)) {
            self.path.push(crate::SEP);
        }
        self.path.push_str(segment);
        ensure!(
            segment.len() <= u8::MAX as usize,
            "path segments must be less than 255 characters, not: {segment}"
        );
        Ok(())
    }

    pub fn chars(&self) -> Chars {
        self.path.chars()
    }

    pub fn segments(&self) -> Segments {
        Segments::new(self)
    }
}

pub fn win_drive(path: &str) -> Option<char> {
    let found = path.starts_with(|c: char| c.is_ascii_alphabetic())
        && path.len() >= 2
        && &path[1..2] == ":";

    if found {
        Some(path.chars().next().unwrap().to_ascii_uppercase())
    } else {
        None
    }
}

#[test]
fn test_abs_path_inner() {
    let p1 = PathInner::new("/home/dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    #[cfg(not(windows))]
    assert_eq!(p1.path, "/home/dir");
    #[cfg(windows)]
    assert_eq!(p1.path, r"C:\home\dir");

    assert_eq!(segs, vec!["home", "dir"]);
    assert_eq!(format!("{p1}"), "/home/dir");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_no_path_inner() {
    let p1 = PathInner::new("").unwrap();
    let segs: Vec<&str> = p1.segments().collect();
    assert_eq!(p1.path, "");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "");
    assert_eq!(p1.is_absolute(), false);
}

#[test]
fn test_root_lin() {
    let p1 = PathInner::new("/").unwrap();
    let segs: Vec<&str> = p1.segments().collect();
    assert_eq!(p1.path, "/");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "/");
    assert_eq!(p1.is_absolute(), true);
}

#[cfg(windows)]
#[test]
fn test_root_win() {
    // a windows-formatted c:/ path is kept on win.
    let p1 = PathInner::new("c:/").unwrap();
    let segs: Vec<&str> = p1.segments().collect();
    assert_eq!(p1.path, r"c:\");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), r"c:\");
    assert_eq!(p1.is_absolute(), true);
}

#[cfg(not(windows))]
#[test]
fn test_root_win() {
    // a windows-formatted c:/ path is replaced with
    // / on lin.
    let p1 = PathInner::new("c:/").unwrap();
    let segs: Vec<&str> = p1.segments().collect();
    assert_eq!(p1.path, "/");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "/");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_home_path_inner() {
    // when running tests '~' = /home/test
    let p1 = PathInner::new("~/dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/home/test/dir");
    assert_eq!(segs, vec!["home", "test", "dir"]);
    assert_eq!(format!("{p1}"), "~/dir");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_cwd_path_inner() {
    // when running tests '.' = /var/test
    let p1 = PathInner::new("./dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/var/test/dir");
    assert_eq!(segs, vec!["var", "test", "dir"]);
    assert_eq!(format!("{p1}"), "./dir");
    assert_eq!(p1.is_absolute(), true);
}

#[cfg(not(windows))]
#[test]
fn test_display() {
    let p1 = PathInner::new("./dir/hi").unwrap();

    assert_eq!(format!("{p1}"), "./dir/hi");
    assert_eq!(format!("{p1:#}"), "/var/test/dir/hi");
}

#[cfg(windows)]
#[test]
fn test_display() {
    let p1 = PathInner::new(r".\dir\hi").unwrap();

    assert_eq!(format!("{p1}"), r".\dir\hi");
    assert_eq!(format!("{p1:#}"), r"\var\test\dir\hi");
}

#[test]
fn test_debug() {
    let p1 = PathInner::new("./dir/hi").unwrap();

    assert_eq!(format!("{p1:?}"), "./dir/hi");
    assert_eq!(format!("{p1:#?}"), "/var/test/dir/hi");
}
