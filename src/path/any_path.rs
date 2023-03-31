use std::{path::PathBuf, str::Chars};

use crate::{inner::PathInner, iter::Segments};

pub struct AnyPath(PathInner);

impl AnyPath {
    pub fn segments(&self) -> Segments {
        self.0.segments()
    }

    pub fn chars(&self) -> Chars {
        self.0.chars()
    }
}

impl TryFrom<String> for AnyPath {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(PathInner::new(&value)?))
    }
}

impl TryFrom<&str> for AnyPath {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(PathInner::new(value)?))
    }
}

impl TryFrom<PathBuf> for AnyPath {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self(PathInner::new_from_path(&value)?))
    }
}
