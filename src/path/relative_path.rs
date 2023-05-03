use crate::os::CurrentOS;
use crate::Result;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{RelativeFilePath, RelativeFolderPath};
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelativePath(pub(crate) PathInner<CurrentOS>);

all_paths!(RelativePath);
try_from!(RelativePath);

impl RelativePath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_relative()?;
        Ok(self)
    }

    pub fn to_concrete(self) -> Either<RelativeFolderPath, RelativeFilePath> {
        match self.0.is_folder() {
            true => Either::Left(RelativeFolderPath(self.0)),
            false => Either::Right(RelativeFilePath(self.0)),
        }
    }
}

impl From<RelativeFilePath> for RelativePath {
    fn from(value: RelativeFilePath) -> Self {
        RelativePath(value.0)
    }
}

impl From<RelativeFolderPath> for RelativePath {
    fn from(value: RelativeFolderPath) -> Self {
        RelativePath(value.0)
    }
}
