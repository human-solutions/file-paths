use crate::os::CurrentOS;
use crate::{all_dirs, AbsoluteFolderPath, AnyFilePath, AnyPath, RelativeFolderPath};
use crate::{all_paths, inner::PathInner, try_from};
use crate::{PathError, Result};
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyFolderPath(pub(crate) PathInner<CurrentOS>);

all_paths!(AnyFolderPath);
all_dirs!(AnyFolderPath);
try_from!(AnyFolderPath);

impl AnyFolderPath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_folder()?;
        Ok(self)
    }

    pub fn to_concrete(self) -> Either<AbsoluteFolderPath, RelativeFolderPath> {
        match self.0.is_absolute() {
            true => Either::Left(AbsoluteFolderPath(self.0)),
            false => Either::Right(RelativeFolderPath(self.0)),
        }
    }

    pub fn with_file<F>(&self, file: F) -> Result<AnyFilePath>
    where
        F: TryInto<AnyFilePath, Error = PathError>,
    {
        let file: AnyFilePath = file.try_into()?;
        let path = self.0.path.clone() + file.as_str();
        path.try_into()
    }
}

impl From<RelativeFolderPath> for AnyFolderPath {
    fn from(value: RelativeFolderPath) -> Self {
        Self(value.0)
    }
}

impl From<AbsoluteFolderPath> for AnyFolderPath {
    fn from(value: AbsoluteFolderPath) -> Self {
        Self(value.0)
    }
}

impl TryFrom<AnyPath> for AnyFolderPath {
    type Error = crate::PathError;

    fn try_from(value: AnyPath) -> std::result::Result<Self, Self::Error> {
        value.0.as_str().try_into()
    }
}
