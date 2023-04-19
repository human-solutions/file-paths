#![allow(dead_code)]

pub(super) fn remove_win_drive(path: &str) -> &str {
    if has_drive(path) {
        &path[2..]
    } else {
        path
    }
}

pub(super) fn add_win_drive<'a>(path: &'a str, drive: char, to: &mut String) -> &'a str {
    let (path, drive) = match win_drive(path) {
        Some(drive) => (&path[2..], drive),
        None => (path, drive),
    };
    to.push(drive);
    to.push(':');
    path
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
