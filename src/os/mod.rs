#[cfg(all(not(test), windows))]
mod win_os;

#[cfg(all(not(test), windows))]
pub(crate) use win_os::WinOS as CurrentOS;

#[cfg(all(not(test), not(windows)))]
mod lin_os;
#[cfg(all(not(test), not(windows)))]
pub(crate) use lin_os::LinOS as CurrentOS;
#[cfg(test)]
pub(crate) use test_os::LinTestOS as CurrentOS;

#[cfg(test)]
mod test_os;
#[cfg(test)]
pub use test_os::{LinTestOS, WinTestOS};

mod drive;
mod env;

use crate::Result;
pub(crate) use env::contract;
pub(crate) use env::expand;

pub(crate) trait OsGroup {
    const SEP: char;
    const SEP_STR: &'static str;

    fn is_absolute(path: &str) -> bool;
    fn start_of_relative_path(path: &str) -> usize;
    /// if a drive letter is found, then it is added to the inner path and the
    /// returns the path string without the drive
    fn process_drive_letter<'a>(path: &'a str, inner: &mut String) -> Result<&'a str>;

    fn home() -> Result<String>;
    fn current() -> Result<String>;
    fn drive_letter() -> Result<char>;

    fn contract(path: &str) -> Result<(Option<char>, &str)> {
        let home_rel = Self::remove_abs_start(path, &Self::home()?);
        let cwd_rel = Self::remove_abs_start(path, &Self::current()?);
        Ok(match (home_rel, cwd_rel) {
            (Some(home), Some(cwd)) if home.len() < cwd.len() => (Some('~'), home),
            (Some(_), Some(cwd)) => (Some('.'), cwd),
            (Some(home), None) => (Some('~'), home),
            (None, Some(cwd)) => (Some('.'), cwd),
            (None, None) => (None, path),
        })
    }
    fn remove_abs_start<'a>(path: &'a str, start: &str) -> Option<&'a str> {
        if path.starts_with(start) {
            let mut pos = start.len();
            if path[pos..].starts_with(Self::SEP) {
                pos += 1;
            }
            Some(&path[pos..])
        } else {
            None
        }
    }

    fn as_contracted(path: &str) -> (Option<char>, &str) {
        if Self::is_absolute(path) {
            match Self::contract(path) {
                Ok(s) => s,
                Err(_) => (None, path),
            }
        } else {
            (None, path)
        }
    }
    fn debug_fmt(path: &str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (chr, path) = Self::as_contracted(path);

        let path = drive::remove_win_drive(path).replace('\\', "/");
        if let Some(chr) = chr {
            write!(f, "{chr}/")?;
        }
        write!(f, "{path}")
    }
}

#[cfg(any(test, not(windows)))]
pub(crate) fn is_absolute_lin(path: &str) -> bool {
    path.starts_with('/')
}

#[cfg(any(test, windows))]
pub(crate) fn is_absolute_win(path: &str) -> bool {
    path.starts_with('\\') || (path.len() >= 3 && &path[1..3] == ":\\")
}

#[cfg(any(test, windows))]
pub(crate) fn start_of_relative_part_win(path: &str) -> usize {
    if path.starts_with('\\') {
        1
    } else if path.len() >= 3 && &path[1..3] == ":\\" {
        3
    } else if path.len() >= 2 && &path[1..2] == ":" {
        2
    } else {
        0
    }
}

#[cfg(any(test, not(windows)))]
pub(crate) fn start_of_relative_part_lin(path: &str) -> usize {
    if path.starts_with('/') {
        1
    } else {
        0
    }
}
