use anyhow::Result;

use crate::os::OsGroup;

pub(crate) fn contract<'a, OS: OsGroup>(path: &'a str) -> Result<(Option<char>, &'a str)> {
    let home_rel = remove_abs_start::<OS>(path, &OS::home()?);
    let cwd_rel = remove_abs_start::<OS>(path, &OS::current()?);
    Ok(match (home_rel, cwd_rel) {
        (Some(home), Some(cwd)) if home.len() < cwd.len() => (Some('~'), home),
        (Some(_), Some(cwd)) => (Some('.'), cwd),
        (Some(home), None) => (Some('~'), home),
        (None, Some(cwd)) => (Some('.'), cwd),
        (None, None) => (None, path),
    })
}

fn remove_abs_start<'a, OS: OsGroup>(path: &'a str, start: &str) -> Option<&'a str> {
    if path.starts_with(start) {
        let mut pos = start.len();
        if path[pos..].starts_with(OS::SEP) {
            pos += 1;
        }
        Some(&path[pos..])
    } else {
        None
    }
}
