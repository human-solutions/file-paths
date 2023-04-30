/// implement TryExist\<String\>, TryFrom<&str>, TryFrom\<PathBuf\>
#[macro_export]
macro_rules! try_exist {
    ($struct:ident) => {
        impl $crate::TryExist<String> for $struct {
            fn try_exist(value: String) -> anyhow::Result<Self> {
                let me = Self(PathInner::new(&value)?);
                me.validate_fs()?;
                Ok(me)
            }
        }

        impl $crate::TryExist<&str> for $struct {
            fn try_exist(value: &str) -> anyhow::Result<Self> {
                let me = Self(PathInner::new(&value)?);
                me.validate_fs()?;
                Ok(me)
            }
        }

        impl $crate::TryExist<std::path::PathBuf> for $struct {
            fn try_exist(value: std::path::PathBuf) -> anyhow::Result<Self> {
                let me = Self(PathInner::new_from_path(&value)?);
                me.validate_fs()?;
                Ok(me)
            }
        }
    };
}
