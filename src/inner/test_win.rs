use std::borrow::Cow;

use crate::inner::PathInner;

use super::expand_envs;

#[test]
fn test_abs_path_inner() {
    let p1 = PathInner::new_win("/home/di").unwrap();
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

    assert_eq!(p1.path, "C:");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "C:");
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

    assert_eq!(p1.path, "C:\\User\\test\\di");
    assert_eq!(segs, vec!["User", "test", "di"]);
    assert_eq!(format!("{p1}"), "~\\di");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_cwd_path_inner() {
    // when running tests '.' = /var/test
    let p1 = PathInner::new(".\\di").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "C:\\current\\di");
    assert_eq!(segs, vec!["current", "di"]);
    assert_eq!(format!("{p1}"), ".\\di");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_display() {
    let p1 = PathInner::new(".\\dir\\hi").unwrap();

    assert_eq!(format!("{p1}"), ".\\dir\\hi");
    assert_eq!(format!("{p1:#}"), "C:\\current\\dir\\hi");
}

#[test]
fn test_debug() {
    let p1 = PathInner::new(".\\dir\\hi").unwrap();

    assert_eq!(format!("{p1:?}"), "./dir/hi");
    assert_eq!(format!("{p1:#?}"), "/current/dir/hi");
}
