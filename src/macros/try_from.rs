/// implement TryFrom\<String\>, TryFrom<&str>, TryFrom\<PathBuf\>
#[macro_export]
macro_rules! try_from {
    // Cannot use PathValues here because it gives an error about conflicting
    // implementations with Into. See:
    // https://github.com/rust-lang/rust/issues/50133#issuecomment-64690839
    ($struct:ident) => {
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

        impl TryFrom<&std::path::Path> for $struct {
            type Error = $crate::PathError;

            fn try_from(value: &std::path::Path) -> std::result::Result<Self, Self::Error> {
                Self(PathInner::new(value)?).validate()
            }
        }

        impl TryFrom<&$struct> for $struct {
            type Error = $crate::PathError;

            fn try_from(value: &$struct) -> std::result::Result<Self, Self::Error> {
                Ok(value.clone())
            }
        }
    };
}
