/// implement TryFrom\<String\>, TryFrom<&str>, TryFrom\<PathBuf\>
#[macro_export]
macro_rules! try_from {
    // Cannot use PathValues here because it gives an error about conflicting
    // implementations with Into. See:
    // https://github.com/rust-lang/rust/issues/50133#issuecomment-64690839
    ($struct:ident) => {
        impl From<&$struct> for $struct {
            fn from(value: &$struct) -> Self {
                value.clone()
            }
        }

        impl TryFrom<String> for $struct {
            type Error = $crate::PathError;

            fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
                Self(PathInner::new(value)?).validate()
            }
        }

        impl TryFrom<&str> for $struct {
            type Error = $crate::PathError;
            fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
                Self(PathInner::new(value)?).validate()
            }
        }

        impl TryFrom<&[&str]> for $struct {
            type Error = $crate::PathError;

            fn try_from(value: &[&str]) -> std::result::Result<Self, Self::Error> {
                Self(PathInner::new(value)?).validate()
            }
        }

        impl TryFrom<&[String]> for $struct {
            type Error = $crate::PathError;

            fn try_from(value: &[String]) -> std::result::Result<Self, Self::Error> {
                Self(PathInner::new(value)?).validate()
            }
        }

        impl TryFrom<Vec<String>> for $struct {
            type Error = $crate::PathError;

            fn try_from(value: Vec<String>) -> std::result::Result<Self, Self::Error> {
                Self(PathInner::new(value)?).validate()
            }
        }

        impl TryFrom<Vec<&str>> for $struct {
            type Error = $crate::PathError;

            fn try_from(value: Vec<&str>) -> std::result::Result<Self, Self::Error> {
                Self(PathInner::new(value)?).validate()
            }
        }

        impl TryFrom<&std::path::Path> for $struct {
            type Error = $crate::PathError;

            fn try_from(value: &std::path::Path) -> std::result::Result<Self, Self::Error> {
                Self(PathInner::new(value)?).validate()
            }
        }

        impl TryFrom<std::path::PathBuf> for $struct {
            type Error = $crate::PathError;

            fn try_from(value: std::path::PathBuf) -> std::result::Result<Self, Self::Error> {
                Self(PathInner::new(value.as_path())?).validate()
            }
        }
    };
}

#[cfg(test)]
use crate::AbsolutePath;

#[test]
fn test_from_strings() {
    let p: AbsolutePath = "/dir1/dir2/".try_into().unwrap();
    assert_eq!(format!("{p:?}"), "AbsolutePath(/dir1/dir2/)");

    let p: AbsolutePath = "/dir1/file".to_string().try_into().unwrap();
    assert_eq!(format!("{p:?}"), "AbsolutePath(/dir1/file)");

    let e: crate::PathError = AbsolutePath::try_from("dir1/dir2").unwrap_err();
    assert_eq!(
        e.to_string(),
        "path is not absolute (it should start with a slash): dir1/dir2"
    );
}

#[test]
fn test_from_str_vecs() {
    let vec: Vec<&str> = vec!["/dir1/", "/dir2/"];

    let p: AbsolutePath = vec.clone().try_into().unwrap();
    assert_eq!(format!("{p:?}"), "AbsolutePath(/dir1/dir2/)");

    let vec: &[&str] = &vec[..];
    let p: AbsolutePath = vec.try_into().unwrap();
    assert_eq!(format!("{p:?}"), "AbsolutePath(/dir1/dir2/)");
}

#[test]
fn test_from_string_vecs() {
    let vec: Vec<String> = vec!["/dir1/".into(), "/dir2/".into()];

    let p: AbsolutePath = vec.clone().try_into().unwrap();
    assert_eq!(format!("{p:?}"), "AbsolutePath(/dir1/dir2/)");
    let p: AbsolutePath = vec[..].try_into().unwrap();
    assert_eq!(format!("{p:?}"), "AbsolutePath(/dir1/dir2/)");
}

#[test]
fn test_from_path() {
    use std::path::PathBuf;
    let path = PathBuf::from("/dir1/dir2/");

    let p: AbsolutePath = path.as_path().try_into().unwrap();
    assert_eq!(format!("{p:?}"), "AbsolutePath(/dir1/dir2/)");

    let p: AbsolutePath = path.try_into().unwrap();
    assert_eq!(format!("{p:?}"), "AbsolutePath(/dir1/dir2/)");
}

#[test]
fn test_from_ref() {
    let p1: AbsolutePath = "/dir1/dir2/".try_into().unwrap();
    let p2: AbsolutePath = (&p1).try_into().unwrap();

    assert_eq!(format!("{p2:?}"), "AbsolutePath(/dir1/dir2/)");
}
