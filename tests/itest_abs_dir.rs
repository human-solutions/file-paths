use serde::{Deserialize, Serialize};
use x_path::AbsoluteFolderPath;

#[derive(Serialize, Deserialize, Debug)]
struct PathTest {
    #[serde(with = "x_path::absolute_folder_path::exist")]
    path1: AbsoluteFolderPath,
    path2: AbsoluteFolderPath,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExpandPathTest {
    #[serde(with = "x_path::absolute_folder_path::expanded")]
    path1: AbsoluteFolderPath,
}

#[test]
fn itest_abs_dir() {
    let p = AbsoluteFolderPath::try_from("/dir1/dir2/").unwrap();

    let segs = p.segments().collect::<Vec<_>>();

    assert_eq!(segs, vec!["dir1", "dir2"]);
    assert_eq!(format!("{p:#?}"), "AbsoluteFolderPath(/dir1/dir2/)");

    assert!(!p.exists());

    let p_src = AbsoluteFolderPath::try_from("./src/").unwrap();
    assert!(p_src.exists());

    let p_not = AbsoluteFolderPath::try_from("some/rel/");

    let err_str = p_not.unwrap_err().to_string();

    assert!(err_str.starts_with("path is not absolute (it should start with a slash): "));
}

#[derive(Serialize)]
struct SomeData {
    path: AbsoluteFolderPath,
}

#[test]
fn i_abs_dir_json() {
    let path = AbsoluteFolderPath::try_from("/dir1/dir2/").unwrap();
    let d = SomeData { path };
    assert_eq!(
        serde_json::to_string(&d).unwrap(),
        r#"{"path":"/dir1/dir2/"}"#
    );

    let exp_p = ExpandPathTest {
        path1: ".".try_into().unwrap(),
    };
    assert!(serde_json::to_string(&exp_p).unwrap().len() > 10);

    let pt1 = PathTest {
        path1: AbsoluteFolderPath::try_from("./src/").unwrap(),
        path2: AbsoluteFolderPath::try_from("./dir1/").unwrap(),
    };
    insta::assert_snapshot!(serde_json::to_string_pretty(&pt1).unwrap(), @r###"
    {
      "path1": "./src/",
      "path2": "./dir1/"
    }
    "###);

    let val = err_json(r###" { "path1": "./doesntexist/", "path2": "./dir1/"  } "###);
    #[cfg(not(windows))]
    assert_eq!(
        val,
        "folder doesn't exist: ./doesntexist/ at line 1 column 28"
    );
    #[cfg(windows)]
    assert_eq!(
        val,
        "folder doesn't exist: .\\doesntexist\\ at line 1 column 28"
    );

    let val = err_json(r###" { "path1": "./Cargo.toml", "path2": "./dir1"  } "###);
    #[cfg(not(windows))]
    assert_eq!(val, "not a folder: ./Cargo.toml at line 1 column 26");
    #[cfg(windows)]
    assert_eq!(val, "not a folder: .\\Cargo.toml at line 1 column 26");
}

fn err_json(s: &str) -> String {
    serde_json::from_str::<PathTest>(s)
        .map_err(|e| e.to_string())
        .unwrap_err()
}
