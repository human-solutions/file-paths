use std::borrow::Cow;

use crate::inner::PathInner;

use super::expand_envs;

#[test]
fn test_abs_path_inner() {
    let p1 = PathInner::new("/home/dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, r"C:\home\dir");
    assert_eq!(segs, vec!["home", "dir"]);
    assert_eq!(format!("{p1}"), r"C:\home\dir");
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
    let p1 = PathInner::new(r"\").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, r"\");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), r"\");
    assert_eq!(p1.is_absolute(), true);
}

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

#[test]
fn test_home_path_inner() {
    // when running tests '~' = /home/test
    let p1 = PathInner::new(r"~\dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, r"\home\test\dir");
    assert_eq!(segs, vec!["home", "test", "dir"]);
    assert_eq!(format!("{p1}"), "~\dir");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_cwd_path_inner() {
    // when running tests '.' = /var/test
    let p1 = PathInner::new(r".\dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, r"\var\test\dir");
    assert_eq!(segs, vec!["var", "test", "dir"]);
    assert_eq!(format!("{p1}"), ".\dir");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_display() {
    let p1 = PathInner::new(r".\dir\hi").unwrap();

    assert_eq!(format!("{p1}"), r".\dir\hi");
    assert_eq!(format!("{p1:#}"), r"\var\test\dir\hi");
}

#[test]
fn test_debug() {
    let p1 = PathInner::new(r".\dir\hi").unwrap();

    assert_eq!(format!("{p1:?}"), r".\dir\hi");
    assert_eq!(format!("{p1:#?}"), r"\var\test\dir\hi");
}

#[test]
fn exp_envs() {
    assert_eq!(exp_ok("$HI"), "$HI");

    assert_eq!(exp_ok("${HI}"), "=hi=");
    assert_eq!(exp_ok(r"\${HI}"), r"\=hi=");
    assert_eq!(exp_ok(r"\${HI}\"), r"\=hi=\");

    assert_eq!(exp_ok("%HI%"), "=hi=");
    assert_eq!(exp_ok(r"\%HI%"), r"\=hi=");
    assert_eq!(exp_ok(r"\%HI%\"), r"\=hi=\");

    // not expanded
    assert_eq!(exp_ok(r"\s$HI$"), r"\s$HI$");
    assert_eq!(exp_ok(r"\%$HI"), r"\%$HI");
    assert_eq!(exp_ok(r"\${HI"), r"\${HI");
    assert_eq!(exp_ok(r"\${H-}"), r"\${H-}");
    assert_eq!(exp_ok(r"\${H}s"), r"\${H}s");
    assert_eq!(exp_ok(r"\%H%s"), r"\%H%s");
    assert_eq!(exp_ok(r"\$"), r"\$");

    assert_eq!(exp_ok(r"\dir1\.\dir2"), r"\dir1\.\dir2");
    assert_eq!(exp_ok(r"dir1\.\dir2"), r"dir1\.\dir2");

    assert_eq!(exp_ok(r"\dir1\~\dir2"), r"\dir1\~\dir2");
    assert_eq!(exp_ok(r"dir1\~\dir2"), r"dir1\~\dir2");

    // errors
    assert_eq!(exp_err(r"\%%"), r"empty environment variable in path: \%%");

    assert_eq!(exp_err(r"\${}"), r"empty environment variable in path: \${}");

    assert_eq!(
        exp_err(r"\${FAIL}"),
        "environment variable 'FAIL' is not defined"
    );

    assert_eq!(exp_ok("."), r"C:\dir\test\");
    assert_eq!(exp_ok(r".\"), r"C:\dir\test\");
    assert_eq!(exp_ok(".\dir"), r"C:\dir\test\dir");

    assert_eq!(exp_ok("~"), r"C:\User\test\");
    assert_eq!(exp_ok(r"~\"), r"C:\User\test\");
    assert_eq!(exp_ok(r"~\dir"), r"C:\User\test\dir");
}

#[cfg(test)]
fn exp_ok(path: &str) -> String {
    expand_envs(path.into()).unwrap()
}

#[cfg(test)]
fn exp_err(path: &str) -> String {
    expand_envs(path.into()).unwrap_err().to_string()
}
