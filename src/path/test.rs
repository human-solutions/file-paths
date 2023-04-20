use crate::{AbsDir, AbsPath};

#[test]
fn eq_test() {
    let p1 = AbsPath::try_from("./hi").unwrap();
    assert!(p1 == "./hi");

    assert!("./hi" == p1);

    let p1 = AbsDir::try_from("./hi/").unwrap();
    assert!(p1 == "./hi/");
    assert!("./hi/" == p1);

    let p1 = AbsDir::try_from("./hi");
    assert_eq!(
        p1.unwrap_err().to_string(),
        "path is not a dir (doesn't end with a slash): ./hi"
    );
}
