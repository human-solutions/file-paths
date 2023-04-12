#![allow(dead_code)]

use anyhow::Result;

pub(super) fn remove_win_drive<'a>(path: &'a str) -> &'a str {
    if has_drive(path) {
        &path[2..]
    } else {
        path
    }
}

pub(super) fn add_win_drive<'a>(path: &'a str, to: &mut String) -> Result<&'a str> {
    let (path, drive) = match win_drive(path) {
        Some(drive) => (&path[2..], drive),
        None => (path, current_drive()?),
    };
    to.push(drive);
    to.push(':');
    Ok(path)
}

pub fn has_drive(path: &str) -> bool {
    path.starts_with(|c: char| c.is_ascii_alphabetic()) && path.len() >= 2 && &path[1..2] == ":"
}

pub fn win_drive(path: &str) -> Option<char> {
    if has_drive(path) {
        Some(path.chars().next().unwrap().to_ascii_uppercase())
    } else {
        None
    }
}

#[cfg(not(windows))]
pub fn current_drive() -> Result<char> {
    Ok('C')
}

#[cfg(windows)]
pub fn current_drive() -> Result<char> {
    use crate::env::current_dir;
    use anyhow::bail;

    let cwd = current_dir()?;
    match win_drive(&cwd) {
        Some(drive) => Ok(drive),
        None => bail!("could not extract drive letter from {cwd}"),
    }
}
