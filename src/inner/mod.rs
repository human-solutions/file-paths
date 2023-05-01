#[cfg(test)]
mod test;

mod path;
mod public;
mod traits;

pub(crate) use path::PathInner;
pub use traits::{PathValues, StrValues, TryExist};

use crate::{ext::PathStrExt, SLASH};
use anyhow::Result;

/// expects a path without drive letters
pub(crate) fn str_segments(path: &str) -> Result<Vec<&str>> {
    let mut segs = Vec::new();
    for seg in path.split(SLASH) {
        seg.assert_allowed_path_component()?;
        if seg == ".." {
            match segs.last() {
                None => segs.push(seg),
                Some(last) if *last == ".." => segs.push(seg),
                Some(_) => drop(segs.pop()),
            };
        } else if !seg.is_empty() {
            segs.push(seg)
        }
    }
    Ok(segs)
}

#[test]
fn test_path_iter() {
    assert_eq!(str_segments("/dir1//dir2/").unwrap(), vec!["dir1", "dir2"]);
    assert_eq!(str_segments("dir1/../dir2/").unwrap(), vec!["dir2"]);
    assert_eq!(
        str_segments("dir1/../../dir2/").unwrap(),
        vec!["..", "dir2"]
    );
    assert_eq!(str_segments("dir1/dir2/dir3/../../").unwrap(), vec!["dir1"]);
    assert_eq!(str_segments("./dir1/..").unwrap(), vec!["."]);
    assert_eq!(str_segments("").unwrap(), Vec::<&str>::new());
    assert_eq!(str_segments("s").unwrap(), vec!["s"]);
}
