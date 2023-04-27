use crate::os::CurrentOS;
use crate::{all_dirs, AbsDir, AnyFile, AnyPath, RelDir};
use crate::{all_paths, inner::PathInner, try_from};
use anyhow::Result;
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyDir(pub(crate) PathInner<CurrentOS>);

all_paths!(AnyDir);
all_dirs!(AnyDir);
try_from!(AnyDir);

impl AnyDir {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_dir()?;
        Ok(self)
    }

    pub fn to_concrete(self) -> Either<AbsDir, RelDir> {
        match self.0.is_absolute() {
            true => Either::Left(AbsDir(self.0)),
            false => Either::Right(RelDir(self.0)),
        }
    }

    pub fn with_file(&self, file: AnyFile) -> AnyFile {
        AnyFile(self.0.appending(file.as_str()))
    }
}

impl From<RelDir> for AnyDir {
    fn from(value: RelDir) -> Self {
        Self(value.0)
    }
}

impl From<AbsDir> for AnyDir {
    fn from(value: AbsDir) -> Self {
        Self(value.0)
    }
}

impl TryFrom<AnyPath> for AnyDir {
    type Error = anyhow::Error;

    fn try_from(value: AnyPath) -> std::result::Result<Self, Self::Error> {
        AnyDir::try_from(value.0.as_str())
    }
}
