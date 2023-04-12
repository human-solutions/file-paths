#[macro_export]
macro_rules! try_from {
    ($struct:ident) => {
        impl TryFrom<String> for $struct {
            type Error = anyhow::Error;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Ok(Self(PathInner::new(&value)?))
            }
        }

        impl TryFrom<&str> for $struct {
            type Error = anyhow::Error;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Ok(Self(PathInner::new(value)?))
            }
        }

        impl TryFrom<PathBuf> for $struct {
            type Error = anyhow::Error;

            fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
                Ok(Self(PathInner::new_from_path(&value)?))
            }
        }
    };
}

#[macro_export]
macro_rules! dirs {
    ($struct:ident) => {
        impl $struct {
            fn as_file
        }
    };
}
