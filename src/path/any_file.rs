use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{AbsFile, AnyDir, AnyPath, RelFile};
use anyhow::Result;
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyFile(pub(crate) PathInner<CurrentOS>);

all_paths!(AnyFile);
try_from!(AnyFile);

impl AnyFile {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_file()?;
        Ok(self)
    }

    pub fn to_concrete(self) -> Either<AbsFile, RelFile> {
        match self.0.is_absolute() {
            true => Either::Left(AbsFile(self.0)),
            false => Either::Right(RelFile(self.0)),
        }
    }

    pub fn drop_file(&self) -> AnyDir {
        AnyDir(self.0.drop_file())
    }
}

impl From<RelFile> for AnyFile {
    fn from(value: RelFile) -> Self {
        Self(value.0)
    }
}

impl From<AbsFile> for AnyFile {
    fn from(value: AbsFile) -> Self {
        Self(value.0)
    }
}

impl TryFrom<AnyPath> for AnyFile {
    type Error = anyhow::Error;

    fn try_from(value: AnyPath) -> std::result::Result<Self, Self::Error> {
        AnyFile::try_from(value.0.as_str())
    }
}
