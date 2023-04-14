use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from, AbsPath, AnyDir, AnyFile, RelPath};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyPath(pub(crate) PathInner<CurrentOS>);

all_paths!(AnyPath);
try_from!(AnyPath);

pub enum PathStart {
    Abs(AbsPath),
    Rel(RelPath),
}

impl AnyPath {
    pub fn is_abs(&self) -> bool {
        self.0.is_absolute()
    }

    pub fn is_rel(&self) -> bool {
        !self.0.is_absolute()
    }

    pub fn to_abs_or_rel(self) -> PathStart {
        match self.is_abs() {
            true => PathStart::Abs(AbsPath(self.0)),
            false => PathStart::Rel(RelPath(self.0)),
        }
    }

    pub fn to_abs(self) -> Option<AbsPath> {
        match self.is_abs() {
            true => Some(AbsPath(self.0)),
            false => None,
        }
    }

    pub fn to_rel(self) -> Option<RelPath> {
        match self.is_abs() {
            true => None,
            false => Some(RelPath(self.0)),
        }
    }
    pub fn to_file(self) -> AnyFile {
        AnyFile(self.0)
    }

    pub fn to_dir(self) -> AnyDir {
        AnyDir(self.0)
    }

    pub(crate) fn validate(self) -> Result<Self> {
        Ok(self)
    }
}
