use super::PathInner;
use crate::os::OsGroup;
use crate::RelativeFolderPath;
use serde::Serialize;
use std::{
    fmt::{Debug, Display},
    path::Path,
};

impl<OS: OsGroup> AsRef<Path> for PathInner<OS> {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}

impl<OS: OsGroup> Display for PathInner<OS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (chr, path) = self.as_contracted(!f.alternate());

        if let Some(chr) = chr {
            write!(f, "{chr}{}", OS::SEP)?;
        }
        write!(f, "{path}")
    }
}

impl<OS: OsGroup> Debug for PathInner<OS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        OS::debug_fmt(&self.path, f)
    }
}

pub trait TryExist<T>: Sized {
    /// Performs the conversion.
    fn try_exist(value: T) -> anyhow::Result<Self>;
}

impl<OS: OsGroup> Serialize for PathInner<OS> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ser.serialize_str(&format!("{self:?}"))
    }
}

impl<T: StrValues> SegmentValues for T {
    fn segments(&self) -> Vec<&str> {
        self.str_vec()
    }
}

pub trait StrValues {
    fn str_vec(&self) -> Vec<&str>;
}

impl StrValues for &[&str] {
    fn str_vec(&self) -> Vec<&str> {
        self.to_vec()
    }
}

impl StrValues for Vec<String> {
    fn str_vec(&self) -> Vec<&str> {
        self.iter().map(|s| s.as_str()).collect()
    }
}

impl StrValues for Vec<&str> {
    fn str_vec(&self) -> Vec<&str> {
        self.clone()
    }
}

impl StrValues for String {
    fn str_vec(&self) -> Vec<&str> {
        vec![self]
    }
}

impl StrValues for &String {
    fn str_vec(&self) -> Vec<&str> {
        vec![self.as_str()]
    }
}

impl StrValues for &str {
    fn str_vec(&self) -> Vec<&str> {
        vec![self]
    }
}

pub trait SegmentValues {
    fn segments(&self) -> Vec<&str>;
}

impl SegmentValues for RelativeFolderPath {
    fn segments(&self) -> Vec<&str> {
        self.segments().collect()
    }
}
