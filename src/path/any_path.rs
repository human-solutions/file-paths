use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{
    AbsoluteFilePath, AbsoluteFolderPath, AnyFilePath, AnyFolderPath, RelativeFilePath,
    RelativeFolderPath,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyPath(pub(crate) PathInner<CurrentOS>);

all_paths!(AnyPath);
try_from!(AnyPath);

pub enum ConcretePath {
    AbsDir(AbsoluteFolderPath),
    RelDir(RelativeFolderPath),
    AbsFile(AbsoluteFilePath),
    RelFile(RelativeFilePath),
}

impl AnyPath {
    pub fn is_abs(&self) -> bool {
        self.0.is_absolute()
    }

    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    pub fn is_dir(&self) -> bool {
        self.0.is_folder()
    }

    pub fn is_rel(&self) -> bool {
        !self.0.is_absolute()
    }

    pub fn to_concrete(self) -> ConcretePath {
        match (self.is_abs(), self.is_dir()) {
            (true, true) => ConcretePath::AbsDir(AbsoluteFolderPath(self.0)),
            (false, true) => ConcretePath::RelDir(RelativeFolderPath(self.0)),
            (true, false) => ConcretePath::AbsFile(AbsoluteFilePath(self.0)),
            (false, false) => ConcretePath::RelFile(RelativeFilePath(self.0)),
        }
    }

    pub(crate) fn validate(self) -> Result<Self> {
        Ok(self)
    }
}

impl From<RelativeFolderPath> for AnyPath {
    fn from(value: RelativeFolderPath) -> Self {
        Self(value.0)
    }
}

impl From<AbsoluteFolderPath> for AnyPath {
    fn from(value: AbsoluteFolderPath) -> Self {
        Self(value.0)
    }
}

impl From<RelativeFilePath> for AnyPath {
    fn from(value: RelativeFilePath) -> Self {
        Self(value.0)
    }
}

impl From<AbsoluteFilePath> for AnyPath {
    fn from(value: AbsoluteFilePath) -> Self {
        Self(value.0)
    }
}

impl From<AnyFolderPath> for AnyPath {
    fn from(value: AnyFolderPath) -> Self {
        Self(value.0)
    }
}

impl From<AnyFilePath> for AnyPath {
    fn from(value: AnyFilePath) -> Self {
        Self(value.0)
    }
}
