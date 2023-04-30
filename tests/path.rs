use x_path::AnyPath;

#[test]
fn some_tests() {
    let p = AnyPath::try_from("dir1/dir2").unwrap();

    let segs = p.segments().collect::<Vec<_>>();

    // let m = x_path::any_path::validated;
    assert_eq!(segs, vec!["dir1", "dir2"]);
    // p.push("")
}
