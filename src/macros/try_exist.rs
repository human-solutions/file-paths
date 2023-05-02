/// implement TryExist\<String\>, TryFrom<&str>, TryFrom\<PathBuf\>
#[macro_export]
macro_rules! try_exist {
    ($struct:ident) => {
        impl<P: $crate::PathValues> $crate::TryExist<P> for $struct {
            fn try_exist(value: P) -> $crate::Result<Self> {
                let me = Self(PathInner::new(value)?);
                me.validate_fs()?;
                Ok(me)
            }
        }
    };
}
