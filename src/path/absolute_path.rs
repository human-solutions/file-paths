use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from};
use crate::{ensure, Result};
use crate::{AbsoluteFilePath, AbsoluteFolderPath};
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbsolutePath(pub(crate) PathInner<CurrentOS>);

all_paths!(AbsolutePath);
try_from!(AbsolutePath);
try_exist!(AbsolutePath);
serde_exist!(AbsolutePath);
serde_expanded!(AbsolutePath);

impl AbsolutePath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_absolute()?;
        Ok(self)
    }

    pub(crate) fn validate_fs(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure(p.exists(), || format!("path doesn't exist: {}", self.0))
    }

    pub fn exists(&self) -> bool {
        self.0.as_path().exists()
    }

    pub fn to_concrete(self) -> Either<AbsoluteFolderPath, AbsoluteFilePath> {
        match self.0.is_folder() {
            true => Either::Left(AbsoluteFolderPath(self.0)),
            false => Either::Right(AbsoluteFilePath(self.0)),
        }
    }
}

impl From<AbsoluteFolderPath> for AbsolutePath {
    fn from(value: AbsoluteFolderPath) -> Self {
        AbsolutePath(value.0)
    }
}

impl From<AbsoluteFilePath> for AbsolutePath {
    fn from(value: AbsoluteFilePath) -> Self {
        AbsolutePath(value.0)
    }
}
