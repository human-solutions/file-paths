/// implement TryFrom\<String\>, TryFrom<&str>, TryFrom\<PathBuf\>
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

            pub fn join(&mut self, path: &str) -> anyhow::Result<Self> {
                Ok(Self(self.0.join(path)?))
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

        impl std::cmp::PartialEq<$struct> for &str {
            fn eq(&self, other: &$struct) -> bool {
                match $struct::try_from(*self) {
                    Ok(me) => other.0.eq(&me.0),
                    Err(_) => false,
                }
            }
        }
    };
}
