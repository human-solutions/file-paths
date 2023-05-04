use crate::os::CurrentOS;
use crate::Result;
use crate::{
    all_dirs, AbsoluteFolderPath, AnyFilePath, AnyPath, RelativeFilePath, RelativeFolderPath,
};
use crate::{all_paths, inner::PathInner, try_from};
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

    pub fn to_concrete(&self) -> Either<AbsoluteFolderPath, RelativeFolderPath> {
        match self.0.is_absolute() {
            true => Either::Left(AbsoluteFolderPath(self.0.clone())),
            false => Either::Right(RelativeFolderPath(self.0.clone())),
        }
    }

    /// Converts an AnyFolderPath to AbsoluteFolderPath. If the path is already
    /// absolute then it is used otherwise it is appended to the root.
    pub fn to_absolute(&self, root: &AbsoluteFolderPath) -> AbsoluteFolderPath {
        match self.0.is_absolute() {
            true => AbsoluteFolderPath(self.0.clone()),
            false => root.with_folder(&RelativeFolderPath(self.0.clone())),
        }
    }

    pub fn with_file(&self, file: &RelativeFilePath) -> AnyFilePath {
        AnyFilePath(self.0.with_path_appended(file.as_str()))
    }

    pub fn with_folder(&self, folder: &RelativeFolderPath) -> Self {
        Self(self.0.with_path_appended(folder.as_str()))
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
