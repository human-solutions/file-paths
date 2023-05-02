use crate::os::CurrentOS;
use crate::Result;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{AbsoluteFilePath, AnyFolderPath, AnyPath, RelativeFilePath};
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyFilePath(pub(crate) PathInner<CurrentOS>);

all_paths!(AnyFilePath);
try_from!(AnyFilePath);

impl AnyFilePath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_file()?;
        Ok(self)
    }

    pub fn to_concrete(self) -> Either<AbsoluteFilePath, RelativeFilePath> {
        match self.0.is_absolute() {
            true => Either::Left(AbsoluteFilePath(self.0)),
            false => Either::Right(RelativeFilePath(self.0)),
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
