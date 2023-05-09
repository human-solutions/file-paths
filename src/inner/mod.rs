#[cfg(test)]
mod test;

mod path;
mod public;
mod traits;

pub(crate) use path::PathInner;
pub use traits::{PathValues, StrValues, TryExist};

use crate::ext::PathStrExt;
use crate::Result;

/// expects a path without drive letters
pub(crate) fn str_segments<'a>(paths: impl Iterator<Item = &'a str>) -> Result<Vec<&'a str>> {
    let mut segs = Vec::new();

    // let path_segs = paths.into_iter().flat_map(|path| path.split(SLASH));
    for seg in paths {
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
    use crate::SLASH;

    assert_eq!(
        str_segments("/dir1//dir2/".split(SLASH)).unwrap(),
        vec!["dir1", "dir2"]
    );
    assert_eq!(
        str_segments("dir1/../dir2/".split(SLASH)).unwrap(),
        vec!["dir2"]
    );
    assert_eq!(
        str_segments("dir1/../../dir2/".split(SLASH)).unwrap(),
        vec!["..", "dir2"]
    );
    assert_eq!(
        str_segments("dir1/dir2/dir3/../../".split(SLASH)).unwrap(),
        vec!["dir1"]
    );
    assert_eq!(str_segments("./dir1/..".split(SLASH)).unwrap(), vec!["."]);
    assert_eq!(str_segments("".split(SLASH)).unwrap(), Vec::<&str>::new());
    assert_eq!(str_segments("s".split(SLASH)).unwrap(), vec!["s"]);
}
