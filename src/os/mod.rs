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

use anyhow::Result;
pub(crate) use env::contract;
pub(crate) use env::expand;

pub(crate) trait OsGroup {
    const SEP: char;

    fn is_absolute(path: &str) -> bool;
    fn relative_part(path: &str) -> &str;
    fn process_drive_letter<'a>(path: &'a str, inner: &mut String) -> Result<&'a str>;

    fn home() -> Result<String>;
    fn current() -> Result<String>;
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
pub(crate) fn relative_part_win(path: &str) -> &str {
    if path.starts_with('\\') {
        &path[1..]
    } else if path.len() >= 3 && &path[1..3] == ":\\" {
        &path[3..]
    } else if path.len() >= 2 && &path[1..2] == ":" {
        &path[2..]
    } else {
        path.as_ref()
    }
}

#[cfg(any(test, not(windows)))]
pub(crate) fn relative_part_lin(path: &str) -> &str {
    if path.starts_with('/') {
        &path[1..]
    } else {
        path.as_ref()
    }
}
