use serde::{Deserialize, Serialize};
use x_path::AbsDir;

#[derive(Serialize, Deserialize, Debug)]
struct PathTest {
    #[serde(with = "x_path::abs_dir::exist")]
    path1: AbsDir,
    path2: AbsDir,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExpandPathTest {
    #[serde(with = "x_path::abs_dir::expanded")]
    path1: AbsDir,
}

#[test]
fn itest_abs_dir() {
    let p = AbsDir::try_from("/dir1/dir2/").unwrap();

    let segs = p.segments().collect::<Vec<_>>();

    assert_eq!(segs, vec!["dir1", "dir2"]);
    assert_eq!(format!("{p:?}"), "AbsDir(/dir1/dir2/)");

    #[cfg(not(windows))]
    const ERR_STR: &str = "dir doesn't exist: /dir1/dir2/";
    #[cfg(windows)] // looks like GitHub CI uses D:
    const ERR_STR: &str = "dir doesn't exist: D:\\dir1\\dir2\\";

    assert_eq!(p.exists().unwrap_err().to_string(), ERR_STR);

    let p_src = AbsDir::try_from("./src/").unwrap();
    assert!(p_src.exists().is_ok());

    let p_not = AbsDir::try_from("some/rel/");

    assert!(p_not
        .unwrap_err()
        .to_string()
        .starts_with("path is not absolute: "));
}

#[test]
fn i_abs_dir_json() {
    let p = AbsDir::try_from("/dir1/dir2/").unwrap();
    assert_eq!(serde_json::to_string(&p).unwrap(), r#""/dir1/dir2/""#);

    let exp_p = ExpandPathTest {
        path1: ".".try_into().unwrap(),
    };
    assert!(serde_json::to_string(&exp_p).unwrap().len() > 10);

    let pt1 = PathTest {
        path1: AbsDir::try_from("./src/").unwrap(),
        path2: AbsDir::try_from("./dir1/").unwrap(),
    };
    insta::assert_snapshot!(serde_json::to_string_pretty(&pt1).unwrap(), @r###"
    {
      "path1": "./src/",
      "path2": "./dir1/"
    }
    "###);

    let val = err_json(r###" { "path1": "./doesntexist/", "path2": "./dir1/"  } "###);
    #[cfg(not(windows))]
    assert_eq!(val, "dir doesn't exist: ./doesntexist/ at line 1 column 28");
    #[cfg(windows)]
    assert_eq!(
        val,
        "dir doesn't exist: .\\doesntexist\\ at line 1 column 28"
    );

    let val = err_json(r###" { "path1": "./Cargo.toml", "path2": "./dir1"  } "###);
    #[cfg(not(windows))]
    assert_eq!(val, "not a directory: ./Cargo.toml at line 1 column 26");
    #[cfg(windows)]
    assert_eq!(val, "not a directory: .\\Cargo.toml at line 1 column 26");
}

fn err_json(s: &str) -> String {
    serde_json::from_str::<PathTest>(s)
        .map_err(|e| e.to_string())
        .unwrap_err()
}
