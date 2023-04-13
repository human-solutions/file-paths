use std::{
    fmt::{Debug, Display},
    path::Path,
};

use serde::Serialize;

use crate::SEP;

use super::PathInner;

impl AsRef<Path> for PathInner {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}

impl Display for PathInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (chr, path) = self.as_contracted(!f.alternate());

        if let Some(chr) = chr {
            write!(f, "{chr}{SEP}")?;
        }
        write!(f, "{path}")
    }
}

impl Debug for PathInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (chr, path) = self.as_contracted(!f.alternate());
        #[cfg(windows)]
        let path = super::drive::remove_win_drive(&path.replace('\\', "/"));
        if let Some(chr) = chr {
            write!(f, "{chr}{SEP}")?;
        }
        write!(f, "{path}")
    }
}

pub trait TryExist<T>: Sized {
    /// Performs the conversion.
    fn try_exist(value: T) -> anyhow::Result<Self>;
}

impl Serialize for PathInner {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ser.serialize_str(&format!("{self:?}"))
    }
}
