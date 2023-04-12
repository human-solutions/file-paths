use x_path::AnyPath;

#[test]
fn some_tests() {
    let p = AnyPath::try_from("dir1/dir2").unwrap();

    let segs = p.segments().collect::<Vec<_>>();

    assert_eq!(segs, vec!["dir1", "dir2"]);

    // p.push("")
}
