use crate::{AbsoluteFolderPath, AbsolutePath};

#[test]
fn eq_test() {
    let p1 = AbsolutePath::try_from("./hi").unwrap();
    assert!(p1 == "./hi");

    assert!("./hi" == p1);

    let p1 = AbsoluteFolderPath::try_from("./hi/").unwrap();
    assert!(p1 == "./hi/");
    assert!("./hi/" == p1);

    let p1 = AbsoluteFolderPath::try_from("./hi");
    assert_eq!(
        p1.unwrap_err().to_string(),
        "path is not a folder (it doesn't end with a slash): ./hi"
    );
}
