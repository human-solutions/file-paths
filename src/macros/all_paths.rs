#[macro_export]
macro_rules! all_paths {
    ($struct:ident) => {
        impl $struct {
            pub fn segments(&self) -> $crate::iter::Segments {
                self.0.segments()
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn as_path(&self) -> &std::path::Path {
                self.0.as_path()
            }

            pub fn starts_with<S: AsRef<str>>(&self, base: S) -> bool {
                self.as_str().starts_with(base.as_ref())
            }

            pub fn ends_with<S: AsRef<str>>(&self, value: S) -> bool {
                self.as_str().ends_with(value.as_ref())
            }

            pub fn contains<S: AsRef<str>>(&self, value: S) -> bool {
                self.as_str().contains(value.as_ref())
            }
        }

        impl std::convert::AsRef<std::path::Path> for $struct {
            fn as_ref(&self) -> &std::path::Path {
                self.0.as_path()
            }
        }
        impl std::convert::AsRef<str> for $struct {
            fn as_ref(&self) -> &str {
                self.0.as_str()
            }
        }

        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::fmt::Debug for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    write!(f, "{}({:#?})", stringify!($struct), self.0)
                } else {
                    write!(f, "{}({:?})", stringify!($struct), self.0)
                }
            }
        }

        impl std::cmp::PartialEq<&str> for $struct {
            fn eq(&self, other: &&str) -> bool {
                match $struct::try_from(*other) {
                    Ok(other) => self.0.eq(&other.0),
                    Err(_) => false,
                }
            }
        }
        impl std::cmp::Eq for $struct {}

        impl std::cmp::PartialEq<$struct> for $struct {
            fn eq(&self, other: &$struct) -> bool {
                self.as_str().eq(other.as_str())
            }
        }

        impl std::cmp::PartialEq<$struct> for &str {
            fn eq(&self, other: &$struct) -> bool {
                match $struct::try_from(*self) {
                    Ok(me) => other.0.eq(&me.0),
                    Err(_) => false,
                }
            }
        }

        impl std::cmp::PartialOrd for $struct {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl std::cmp::Ord for $struct {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl std::hash::Hash for $struct {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.as_str().hash(state);
            }
        }
    };
}

#[test]
fn test_as_ref_str() {
    use crate::AbsoluteFolderPath;
    let dir1: AbsoluteFolderPath = "/dir1/dir2/".try_into().unwrap();
    let dir2: AbsoluteFolderPath = "/dir1/".try_into().unwrap();

    assert!(dir1.starts_with(&dir2));
    assert!(!dir2.starts_with(&dir1));
}
