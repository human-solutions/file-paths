/// implement TryFrom\<String\>, TryFrom<&str>, TryFrom\<PathBuf\>
#[macro_export]
macro_rules! all_paths {
    ($struct:ident) => {
        impl $struct {
            pub fn segments(&self) -> crate::iter::Segments {
                self.0.segments()
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn as_path(&self) -> &std::path::Path {
                self.0.as_path()
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
                    write!(f, "{}({:#?})", stringify!($struct), self.0)
                }
            }
        }
    };
}
