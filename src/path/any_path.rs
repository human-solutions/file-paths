use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{AbsDir, AbsFile, AnyDir, AnyFile, RelDir, RelFile};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyPath(pub(crate) PathInner<CurrentOS>);

all_paths!(AnyPath);
try_from!(AnyPath);

pub enum ConcretePath {
    AbsDir(AbsDir),
    RelDir(RelDir),
    AbsFile(AbsFile),
    RelFile(RelFile),
}

impl AnyPath {
    pub fn is_abs(&self) -> bool {
        self.0.is_absolute()
    }

    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    pub fn is_rel(&self) -> bool {
        !self.0.is_absolute()
    }

    pub fn to_concrete(self) -> ConcretePath {
        match (self.is_abs(), self.is_dir()) {
            (true, true) => ConcretePath::AbsDir(AbsDir(self.0)),
            (false, true) => ConcretePath::RelDir(RelDir(self.0)),
            (true, false) => ConcretePath::AbsFile(AbsFile(self.0)),
            (false, false) => ConcretePath::RelFile(RelFile(self.0)),
        }
    }

    pub(crate) fn validate(self) -> Result<Self> {
        Ok(self)
    }
}

impl From<RelDir> for AnyPath {
    fn from(value: RelDir) -> Self {
        Self(value.0)
    }
}

impl From<AbsDir> for AnyPath {
    fn from(value: AbsDir) -> Self {
        Self(value.0)
    }
}

impl From<RelFile> for AnyPath {
    fn from(value: RelFile) -> Self {
        Self(value.0)
    }
}

impl From<AbsFile> for AnyPath {
    fn from(value: AbsFile) -> Self {
        Self(value.0)
    }
}

impl From<AnyDir> for AnyPath {
    fn from(value: AnyDir) -> Self {
        Self(value.0)
    }
}

impl From<AnyFile> for AnyPath {
    fn from(value: AnyFile) -> Self {
        Self(value.0)
    }
}
