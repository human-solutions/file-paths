use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{serde_impl, AbsoluteFilePath, AnyFolderPath, AnyPath, RelativeFilePath};
use crate::{AbsoluteFolderPath, Result};
use either::Either;

#[derive(Clone)]
pub struct AnyFilePath(pub(crate) PathInner<CurrentOS>);

all_paths!(AnyFilePath);
try_from!(AnyFilePath);
serde_impl!(AnyFilePath);

impl AnyFilePath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_file()?;
        Ok(self)
    }

    pub fn to_concrete(&self) -> Either<AbsoluteFilePath, RelativeFilePath> {
        match self.0.is_absolute() {
            true => Either::Left(AbsoluteFilePath(self.0.clone())),
            false => Either::Right(RelativeFilePath(self.0.clone())),
        }
    }

    pub fn to_absolute_file(&self, root: &AbsoluteFolderPath) -> AbsoluteFilePath {
        match self.0.is_absolute() {
            true => AbsoluteFilePath(self.0.clone()),
            false => root.with_file(&RelativeFilePath(self.0.clone())),
        }
    }

    pub fn drop_file(&self) -> AnyFolderPath {
        AnyFolderPath(self.0.drop_file())
    }
}

impl From<RelativeFilePath> for AnyFilePath {
    fn from(value: RelativeFilePath) -> Self {
        Self(value.0)
    }
}

impl From<AbsoluteFilePath> for AnyFilePath {
    fn from(value: AbsoluteFilePath) -> Self {
        Self(value.0)
    }
}

impl TryFrom<AnyPath> for AnyFilePath {
    type Error = crate::PathError;

    fn try_from(value: AnyPath) -> std::result::Result<Self, Self::Error> {
        value.0.as_str().try_into()
    }
}
