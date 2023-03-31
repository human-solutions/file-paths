use std::{borrow::Cow, fmt::Display};

use anyhow::{Context, Result};

use crate::ext::CharExt;
use crate::SEP;

pub(crate) enum Drive {
    Win(char),
    None,
}

impl Drive {
    fn win(ch: char) -> Self {
        Drive::Win(ch.to_ascii_lowercase())
    }

    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, Drive::None)
    }
}

impl Display for Drive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => Ok(()),
            Self::Win(_c) => {
                #[cfg(windows)]
                return write!(f, "{_c}:{SEP}");
                #[cfg(not(windows))]
                return write!(f, "{SEP}");
            }
        }
    }
}

pub(crate) fn windows_drive(path: &str) -> Result<(Drive, Cow<str>)> {
    let mut chars = path.chars().peekable();
    let (c1, c2) = match (chars.next(), chars.next()) {
        (Some(c1), Some(c2)) => (c1, c2),
        _ => return Ok((Drive::None, Cow::Borrowed(path))),
    };
    if c1.is_ascii_alphabetic() && c2 == ':' {
        let c1 = c1.to_ascii_lowercase();
        if chars.next_if(char::is_slash).is_some() {
            // absolute c:/ path
            Ok((Drive::win(c1), Cow::Borrowed(&path[2..])))
        } else if chars.peek().is_some() {
            // relative c: path that needs to be expanded ()
            let mut path = current_dir().context(
                "could not get the current directory for expanding {c1}: in path {self}",
            )?;
            path.extend(chars);
            Ok((Drive::win(c1), Cow::Owned(path)))
        } else {
            // a 'c:' path is not expanded now
            Ok((Drive::win(c1), Cow::Owned(String::new())))
        }
    } else {
        Ok((Drive::None, Cow::Borrowed(path)))
    }
}

#[cfg(not(test))]
fn current_dir() -> Result<String> {
    use crate::ext::PathBufExt;
    { std::env::current_dir()?.try_to_string() }
        .context("could not resolve current dir environment variable")
}

#[cfg(test)]
fn current_dir() -> Result<String> {
    Ok("cwd/".to_string())
}

#[test]
fn win_drive() {
    assert_eq!(extract("c"), "- c");
    assert_eq!(extract(":"), "- :");
    assert_eq!(extract("c:/dir"), "c - /dir");
    assert_eq!(extract("c:\\dir"), "c - \\dir");
    assert_eq!(extract("C:/"), "c - /");
    assert_eq!(extract("C:\\"), "c - \\");
    assert_eq!(extract("c:dir"), "c - cwd/dir");
    assert_eq!(extract("C:dir"), "c - cwd/dir");
}

#[cfg(test)]
fn extract(path: &str) -> String {
    let (d, s) = windows_drive(path).unwrap();
    if let Drive::Win(c) = d {
        format!("{c} - {s}")
    } else {
        format!("- {s}")
    }
}
