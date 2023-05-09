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
    assert_eq!(format!("{p1:#?}"), "./dir/hi");

    let p1 = PathInner::<WinTestOS>::new(".\\dir\\hi").unwrap();

    assert_eq!(p1.path, "C:\\current\\dir\\hi");
    assert_eq!(format!("{p1:?}"), "./dir/hi");
    assert_eq!(format!("{p1:#?}"), "./dir/hi");
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

#[test]
fn test_file_name() {
    let mut p = PathInner::<LinTestOS>::new("file.text").unwrap();
    assert_eq!(p.file_name(), "file.text");

    p.set_file_name("some.bin").unwrap();
    assert_eq!(format!("{p:?}"), "some.bin");

    let p = PathInner::<LinTestOS>::new("").unwrap();
    assert_eq!(p.file_name(), "");

    let p = PathInner::<LinTestOS>::new(".").unwrap();
    assert_eq!(p.file_name(), "");

    let p = PathInner::<LinTestOS>::new("/dir/").unwrap();
    assert_eq!(p.file_name(), "");

    let p = PathInner::<LinTestOS>::new("dir/file.text.zip").unwrap();
    assert_eq!(p.file_name(), "file.text.zip");

    let p = PathInner::<LinTestOS>::new("/root/dir/file.text").unwrap();
    assert_eq!(p.file_name(), "file.text");

    let mut p = PathInner::<WinTestOS>::new("file.text").unwrap();
    assert_eq!(p.file_name(), "file.text");

    p.set_file_name("some.bin").unwrap();
    assert_eq!(format!("{p:?}"), "some.bin");

    let p = PathInner::<WinTestOS>::new("dir\\file.text").unwrap();
    assert_eq!(p.file_name(), "file.text");

    let p = PathInner::<WinTestOS>::new("c:\\root\\dir\\file.text").unwrap();
    assert_eq!(p.file_name(), "file.text");
}

#[test]
fn test_file_stem() {
    let mut p = PathInner::<LinTestOS>::new("file.text").unwrap();
    assert_eq!(p.file_stem(), "file");

    p.set_file_name("some.bin").unwrap();
    assert_eq!(p.file_stem(), "some");
    assert_eq!(format!("{p:?}"), "some.bin");

    let p = PathInner::<LinTestOS>::new("dir/file.text").unwrap();
    assert_eq!(p.file_stem(), "file");

    let p = PathInner::<LinTestOS>::new("/root/dir/file.text").unwrap();
    assert_eq!(p.file_stem(), "file");

    let mut p = PathInner::<WinTestOS>::new("file.text").unwrap();
    assert_eq!(p.file_stem(), "file");

    p.set_file_stem("some.bin").unwrap();
    assert_eq!(format!("{p:?}"), "some.bin.text");

    let p = PathInner::<WinTestOS>::new("dir\\file.text").unwrap();
    assert_eq!(p.file_stem(), "file");

    let p = PathInner::<WinTestOS>::new("c:\\root\\dir\\file.text").unwrap();
    assert_eq!(p.file_stem(), "file");
}

#[test]
fn test_parent() {
    let p = PathInner::<LinTestOS>::new("/parent/dir").unwrap();
    let parent = p.parent().unwrap();
    assert_eq!(format!("{parent:?}"), "/parent/");

    let p = PathInner::<LinTestOS>::new("/parent/").unwrap();
    assert_eq!(p.parent(), None);

    let p = PathInner::<LinTestOS>::new("/parent/dir/").unwrap();

    let parent = p.parent().unwrap();
    assert_eq!(format!("{parent:?}"), "/parent/");
}

#[test]
fn test_pop() {
    let p = PathInner::<LinTestOS>::new("").unwrap();
    assert_eq!(format!("{:?}", p.popping(0)), "");
    assert_eq!(format!("{:?}", p.popping(1)), "");

    let p = PathInner::<LinTestOS>::new("dir1/dir2/file").unwrap();
    assert_eq!(format!("{:?}", p.popping(0)), "dir1/dir2/file");
    assert_eq!(format!("{:?}", p.popping(1)), "dir1/dir2/");
    assert_eq!(format!("{:?}", p.popping(2)), "dir1/");
    assert_eq!(format!("{:?}", p.popping(3)), "");
    assert_eq!(format!("{:?}", p.popping(4)), "");

    let p = PathInner::<LinTestOS>::new("/dir1").unwrap();
    assert_eq!(format!("{:?}", p.popping(0)), "/dir1");
    assert_eq!(format!("{:?}", p.popping(1)), "/");
    assert_eq!(format!("{:?}", p.popping(2)), "/");

    let p = PathInner::<WinTestOS>::new("c:dir1/dir2/file").unwrap();
    assert_eq!(format!("{}", p.popping(0)), "C:dir1\\dir2\\file");
    assert_eq!(format!("{:?}", p.popping(1)), "dir1/dir2/");
    assert_eq!(format!("{:?}", p.popping(2)), "dir1/");
    assert_eq!(format!("{:?}", p.popping(3)), "");
    assert_eq!(format!("{}", p.popping(4)), "C:");
}

#[test]
fn test_join() {
    let p = PathInner::<LinTestOS>::new("/dir1/").unwrap();

    let j = p.joining(vec!["dir2", "dir3/"]).unwrap();
    assert_eq!(format!("{j:?}"), "/dir1/dir2/dir3/");
}
