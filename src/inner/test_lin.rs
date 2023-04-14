use crate::{inner::PathInner, os::LinTestOS};

#[test]
fn test_abs_path_inner() {
    let p1 = PathInner::<LinTestOS>::new("/home/dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/home/dir");
    assert_eq!(segs, vec!["home", "dir"]);
    assert_eq!(format!("{p1}"), "/home/dir");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_no_path_inner() {
    let p1 = PathInner::<LinTestOS>::new("").unwrap();
    let segs: Vec<&str> = p1.segments().collect();
    assert_eq!(p1.path, "");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "");
    assert_eq!(p1.is_absolute(), false);
}

#[test]
fn test_root_lin() {
    let p1 = PathInner::<LinTestOS>::new("/").unwrap();
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
    let p1 = PathInner::<LinTestOS>::new("c:/").unwrap();
    let segs: Vec<&str> = p1.segments().collect();
    assert_eq!(p1.path, "/");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "/");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_home_path_inner() {
    // when running tests '~' = /home/test
    let p1 = PathInner::<LinTestOS>::new("~/dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/home/test/dir");
    assert_eq!(segs, vec!["home", "test", "dir"]);
    assert_eq!(format!("{p1}"), "~/dir");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_cwd_path_inner() {
    // when running tests '.' = /var/test
    let p1 = PathInner::<LinTestOS>::new("./dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/var/test/dir");
    assert_eq!(segs, vec!["var", "test", "dir"]);
    assert_eq!(format!("{p1}"), "./dir");
    assert_eq!(p1.is_absolute(), true);
}

#[test]
fn test_display() {
    let p1 = PathInner::<LinTestOS>::new("./dir/hi").unwrap();

    assert_eq!(format!("{p1}"), "./dir/hi");
    assert_eq!(format!("{p1:#}"), "/var/test/dir/hi");
}

#[test]
fn test_debug() {
    let p1 = PathInner::<LinTestOS>::new("./dir/hi").unwrap();

    assert_eq!(format!("{p1:?}"), "./dir/hi");
    assert_eq!(format!("{p1:#?}"), "/var/test/dir/hi");
}
