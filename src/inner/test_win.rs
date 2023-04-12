use std::borrow::Cow;

use crate::inner::PathInner;

use super::expand_envs;

#[test]
fn test_abs_path_inner() {
    let p1 = PathInner::new("/home/di").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "C:\\home\\di");
    assert_eq!(segs, vec!["home", "di"]);
    assert_eq!(format!("{p1}"), "C:\\home\\di");
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
    let p1 = PathInner::new("\\").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "C:\\");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "C:\\");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_root_win() {
    // a windows-formatted c:/ path is kept on win.
    let p1 = PathInner::new("c:/").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "C:\\");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "C:\\");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_home_path_inner() {
    // when running tests '~' = /home/test
    let p1 = PathInner::new("~\\di").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "\\home\\test\\di");
    assert_eq!(segs, vec!["home", "test", "di"]);
    assert_eq!(format!("{p1}"), "~\\di");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_cwd_path_inner() {
    // when running tests '.' = /var/test
    let p1 = PathInner::new(".\\di").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "\\var\\test\\di");
    assert_eq!(segs, vec!["va", "test", "di"]);
    assert_eq!(format!("{p1}"), ".\\di");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_display() {
    let p1 = PathInner::new(".\\dir\\hi").unwrap();

    assert_eq!(format!("{p1}"), ".\\dir\\hi");
    assert_eq!(format!("{p1:#}"), "\\var\\test\\dir\\hi");
}

#[test]
fn test_debug() {
    let p1 = PathInner::new(".\\dir\\hi").unwrap();

    assert_eq!(format!("{p1:?}"), ".\\dir\\hi");
    assert_eq!(format!("{p1:#?}"), "\\var\\test\\dir\\hi");
}

#[test]
fn exp_envs() {
    assert_eq!(exp_ok("$HI"), "$HI");

    assert_eq!(exp_ok("${HI}"), "=hi=");
    assert_eq!(exp_ok("\\${HI}"), "\\=hi=");
    assert_eq!(exp_ok("\\${HI}\\"), "\\=hi=\\");

    assert_eq!(exp_ok("%HI%"), "=hi=");
    assert_eq!(exp_ok("\\%HI%"), "\\=hi=");
    assert_eq!(exp_ok("\\%HI%\\"), "\\=hi=\\");

    // not expanded
    assert_eq!(exp_ok("\\s$HI$"), "\\s$HI$");
    assert_eq!(exp_ok("\\%$HI"), "\\%$HI");
    assert_eq!(exp_ok("\\${HI"), "\\${HI");
    assert_eq!(exp_ok("\\${H-}"), "\\${H-}");
    assert_eq!(exp_ok("\\${H}s"), "\\${H}s");
    assert_eq!(exp_ok("\\%H%s"), "\\%H%s");
    assert_eq!(exp_ok("\\$"), "\\$");

    assert_eq!(exp_ok("\\dir1\\.\\dir2"), "\\dir1\\.\\dir2");
    assert_eq!(exp_ok("dir1\\.\\dir2"), "dir1\\.\\dir2");

    assert_eq!(exp_ok("\\dir1\\~\\dir2"), "\\dir1\\~\\dir2");
    assert_eq!(exp_ok("dir1\\~\\dir2"), "dir1\\~\\dir2");

    // errors
    assert_eq!(exp_err("\\%%"), "empty environment variable in path: \\%%");

    assert_eq!(
        exp_err("\\${}"),
        "empty environment variable in path: \\${}"
    );

    assert_eq!(
        exp_err("\\${FAIL}"),
        "environment variable 'FAIL' is not defined"
    );

    assert_eq!(exp_ok("."), "C:\\dir\\test\\");
    assert_eq!(exp_ok(".\\"), "C:\\dir\\test\\");
    assert_eq!(exp_ok(".\\di"), "C:\\dir\\test\\di");

    assert_eq!(exp_ok("~"), "C:\\User\\test\\");
    // the double ending slash is ok at expansion stage as it is removed later
    assert_eq!(exp_ok("~\\"), "C:\\User\\test\\\\");
    assert_eq!(exp_ok("~\\di"), "C:\\User\\test\\\\di");
}

#[cfg(test)]
fn exp_ok(path: &str) -> Cow<str> {
    expand_envs(path.into()).unwrap()
}

#[cfg(test)]
fn exp_err(path: &str) -> String {
    expand_envs(path.into()).unwrap_err().to_string()
}
