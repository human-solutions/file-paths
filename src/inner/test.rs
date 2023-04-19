use crate::{
    inner::PathInner,
    os::{LinTestOS, WinTestOS},
};

#[test]
fn test_abs_path_inner() {
    let p1 = PathInner::<LinTestOS>::new("/home/dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/home/dir");
    assert_eq!(segs, vec!["home", "dir"]);
    assert_eq!(format!("{p1}"), "/home/dir");
    assert!(p1.is_absolute());

    let p1 = PathInner::<WinTestOS>::new("/home/di").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "C:\\home\\di");
    assert_eq!(segs, vec!["home", "di"]);
    assert_eq!(format!("{p1}"), "C:\\home\\di");
    assert!(p1.is_absolute());
}

#[test]
fn test_no_path_inner() {
    let p1 = PathInner::<LinTestOS>::new("").unwrap();
    let segs: Vec<&str> = p1.segments().collect();
    assert_eq!(p1.path, "");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "");
    assert!(!p1.is_absolute());

    let p1 = PathInner::<WinTestOS>::new("").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "C:");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "C:");
    assert!(!p1.is_absolute());
}

#[test]
fn test_root_lin() {
    let p1 = PathInner::<LinTestOS>::new("/").unwrap();
    let segs: Vec<&str> = p1.segments().collect();
    assert_eq!(p1.path, "/");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "/");
    assert!(p1.is_absolute());

    let p1 = PathInner::<WinTestOS>::new("\\").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "C:\\");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "C:\\");
    assert!(p1.is_absolute());
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
    assert!(p1.is_absolute());

    // a windows-formatted c:/ path is kept on win.
    let p1 = PathInner::<WinTestOS>::new("c:/").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "C:\\");
    assert_eq!(segs, Vec::<&str>::new());
    assert_eq!(format!("{p1}"), "C:\\");
    assert!(p1.is_absolute());
}

#[test]
fn test_home_path_inner() {
    // when running tests '~' = /home/test
    let p1 = PathInner::<LinTestOS>::new("~/dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/home/test/dir");
    assert_eq!(segs, vec!["home", "test", "dir"]);
    assert_eq!(format!("{p1}"), "~/dir");
    assert!(p1.is_absolute());

    let p1 = PathInner::<WinTestOS>::new("~\\dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "C:\\User\\test\\dir");
    assert_eq!(segs, vec!["User", "test", "dir"]);
    assert_eq!(format!("{p1}"), "~\\dir");
    assert!(p1.is_absolute());
}

#[test]
fn test_cwd_path_inner() {
    // when running tests '.' = /var/test
    let p1 = PathInner::<LinTestOS>::new("./dir").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "/var/test/dir");
    assert_eq!(segs, vec!["var", "test", "dir"]);
    assert_eq!(format!("{p1}"), "./dir");
    assert!(p1.is_absolute());

    let p1 = PathInner::<WinTestOS>::new(".\\di").unwrap();
    let segs: Vec<&str> = p1.segments().collect();

    assert_eq!(p1.path, "C:\\current\\di");
    assert_eq!(segs, vec!["current", "di"]);
    assert_eq!(format!("{p1}"), ".\\di");
    assert!(p1.is_absolute());
}

#[test]
fn test_display() {
    let p1 = PathInner::<LinTestOS>::new("./dir/hi").unwrap();

    assert_eq!(format!("{p1}"), "./dir/hi");
    assert_eq!(format!("{p1:#}"), "/var/test/dir/hi");

    let p1 = PathInner::<WinTestOS>::new(".\\dir\\hi").unwrap();

    assert_eq!(format!("{p1}"), ".\\dir\\hi");
    assert_eq!(format!("{p1:#}"), "C:\\current\\dir\\hi");
}

#[test]
fn test_debug() {
    let p1 = PathInner::<LinTestOS>::new("./dir/hi").unwrap();

    assert_eq!(format!("{p1:?}"), "./dir/hi");
    assert_eq!(format!("{p1:#?}"), "/var/test/dir/hi");

    let p1 = PathInner::<WinTestOS>::new(".\\dir\\hi").unwrap();

    assert_eq!(p1.path, "C:\\current\\dir\\hi");
    assert_eq!(format!("{p1:?}"), "./dir/hi");
    assert_eq!(format!("{p1:#?}"), "/current/dir/hi");
}

#[test]
fn test_rel() {
    let p = PathInner::<WinTestOS>::new("some/rel").unwrap();

    assert_eq!(p.path, "C:some\\rel");
}

#[test]
fn test_extensions() {
    let mut p = PathInner::<LinTestOS>::new("some/file.with.ext").unwrap();
    let exts = p.extensions().collect::<Vec<_>>();

    assert_eq!(exts, vec!["with", "ext"]);
    p.set_extensions("hi");
    assert_eq!(format!("{p:?}"), "some/file.hi");

    let mut p = PathInner::<LinTestOS>::new("some/file.").unwrap();
    let exts = p.extensions().collect::<Vec<_>>();
    assert_eq!(exts, Vec::<String>::new());

    p.set_extensions(vec!["txt", "bz"]);
    assert_eq!(format!("{p:?}"), "some/file.txt.bz");

    p.set_extensions(Vec::<String>::new());
    assert_eq!(format!("{p:?}"), "some/file");

    p.set_extensions("txt");
    assert_eq!(format!("{p:?}"), "some/file.txt")
}
