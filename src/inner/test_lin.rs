use std::borrow::Cow;

use crate::inner::PathInner;

use super::expand_envs;

#[test]
fn test_abs_path_inner() {
    let p1 = PathInner::new("/home/dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/home/dir");
    assert_eq!(segs, vec!["home", "dir"]);
    assert_eq!(format!("{p1}"), "/home/dir");
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
    let p1 = PathInner::new("/").unwrap();
    let segs: Vec<&str> = p1.segments().collect();
    assert_eq!(p1.path, "/");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "/");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_root_win() {
    // a windows-formatted c:/ path is replaced with
    // / on lin.
    let p1 = PathInner::new("c:/").unwrap();
    let segs: Vec<&str> = p1.segments().collect();
    assert_eq!(p1.path, "/");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "/");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_home_path_inner() {
    // when running tests '~' = /home/test
    let p1 = PathInner::new("~/dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/home/test/dir");
    assert_eq!(segs, vec!["home", "test", "dir"]);
    assert_eq!(format!("{p1}"), "~/dir");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_cwd_path_inner() {
    // when running tests '.' = /var/test
    let p1 = PathInner::new("./dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/var/test/dir");
    assert_eq!(segs, vec!["var", "test", "dir"]);
    assert_eq!(format!("{p1}"), "./dir");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_display() {
    let p1 = PathInner::new("./dir/hi").unwrap();

    assert_eq!(format!("{p1}"), "./dir/hi");
    assert_eq!(format!("{p1:#}"), "/var/test/dir/hi");
}

#[test]
fn test_debug() {
    let p1 = PathInner::new("./dir/hi").unwrap();

    assert_eq!(format!("{p1:?}"), "./dir/hi");
    assert_eq!(format!("{p1:#?}"), "/var/test/dir/hi");
}

#[test]
fn exp_envs() {
    assert_eq!(exp_ok("$HI"), "$HI");

    assert_eq!(exp_ok("${HI}"), "=hi=");
    assert_eq!(exp_ok("/${HI}"), "/=hi=");
    assert_eq!(exp_ok("/${HI}/"), "/=hi=/");

    assert_eq!(exp_ok("%HI%"), "=hi=");
    assert_eq!(exp_ok("/%HI%"), "/=hi=");
    assert_eq!(exp_ok("/%HI%/"), "/=hi=/");

    // not expanded
    assert_eq!(exp_ok("/s$HI$"), "/s$HI$");
    assert_eq!(exp_ok("/%$HI"), "/%$HI");
    assert_eq!(exp_ok("/${HI"), "/${HI");
    assert_eq!(exp_ok("/${H-}"), "/${H-}");
    assert_eq!(exp_ok("/${H}s"), "/${H}s");
    assert_eq!(exp_ok("/%H%s"), "/%H%s");
    assert_eq!(exp_ok("/$"), "/$");

    assert_eq!(exp_ok("/dir1/./dir2"), "/dir1/./dir2");
    assert_eq!(exp_ok("dir1/./dir2"), "dir1/./dir2");

    assert_eq!(exp_ok("/dir1/~/dir2"), "/dir1/~/dir2");
    assert_eq!(exp_ok("dir1/~/dir2"), "dir1/~/dir2");

    // errors
    assert_eq!(exp_err("/%%"), "empty environment variable in path: /%%");

    assert_eq!(exp_err("/${}"), "empty environment variable in path: /${}");

    assert_eq!(
        exp_err("/${FAIL}"),
        "environment variable 'FAIL' is not defined"
    );

    assert_eq!(exp_ok("."), "/var/test/");
    assert_eq!(exp_ok("./"), "/var/test/");
    assert_eq!(exp_ok("./dir"), "/var/test/dir");

    assert_eq!(exp_ok("~"), "/home/test/");
    assert_eq!(exp_ok("~/"), "/home/test/");
    assert_eq!(exp_ok("~/dir"), "/home/test/dir");
}

fn exp_ok(path: &str) -> Cow<str> {
    expand_envs(path.into()).unwrap()
}

fn exp_err(path: &str) -> String {
    expand_envs(path.into())
        .unwrap_err()
        .to_string()
        .replace('\\', "/")
}
