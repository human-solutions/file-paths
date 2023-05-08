use std::path::Path;

use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{
    AbsoluteFilePath, AbsoluteFolderPath, AnyFilePath, AnyFolderPath, RelativeFilePath,
    RelativeFolderPath,
};
use crate::{PathError, Result};
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyPath(pub(crate) PathInner<CurrentOS>);

all_paths!(AnyPath);
try_from!(AnyPath);

#[derive(Debug)]
pub enum ConcretePath {
    AbsFolder(AbsoluteFolderPath),
    RelFolder(RelativeFolderPath),
    AbsFile(AbsoluteFilePath),
    RelFile(RelativeFilePath),
}

impl ConcretePath {
    fn new(path: &Path) -> Result<Self> {
        match (path.to_str(), path.is_file()) {
            (Some(file), true) => Self::new_file(file),
            (Some(folder), false) => Self::new_folder(folder),
            (None, _) => Err("".into()),
        }
    }
    fn new_file(file: &str) -> Result<Self> {
        let file: AnyFilePath = file.try_into()?;

        Ok(match file.to_concrete() {
            Either::Left(abs_file) => Self::AbsFile(abs_file),
            Either::Right(rel_file) => Self::RelFile(rel_file),
        })
    }

    fn new_folder(folder: &str) -> Result<Self> {
        let folder: AnyFolderPath = folder.try_into()?;

        Ok(match folder.to_concrete() {
            Either::Left(abs_folder) => Self::AbsFolder(abs_folder),
            Either::Right(rel_folder) => Self::RelFolder(rel_folder),
        })
    }
}

impl TryFrom<&Path> for ConcretePath {
    type Error = PathError;

    fn try_from(value: &Path) -> std::result::Result<Self, Self::Error> {
        ConcretePath::new(value)
    }
}

impl AnyPath {
    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    }

    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    pub fn is_folder(&self) -> bool {
        self.0.is_folder()
    }

    pub fn is_relative(&self) -> bool {
        !self.0.is_absolute()
    }

    pub fn to_concrete(self) -> ConcretePath {
        match (self.is_absolute(), self.is_folder()) {
            (true, true) => ConcretePath::AbsFolder(AbsoluteFolderPath(self.0)),
            (false, true) => ConcretePath::RelFolder(RelativeFolderPath(self.0)),
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

#[test]
fn test_convert_to_concrete() {
    let p: AnyPath = "/dir/file".try_into().unwrap();
    assert_eq!(format!("{:?}", p.to_concrete()), "AbsFile(/dir/file)");

    let p: AnyPath = "dir/file".try_into().unwrap();
    assert_eq!(format!("{:?}", p.to_concrete()), "RelFile(dir/file)");

    let p: AnyPath = "dir/".try_into().unwrap();
    assert_eq!(format!("{:?}", p.to_concrete()), "RelFolder(dir/)");
    let p: AnyPath = "/dir/".try_into().unwrap();
    assert_eq!(format!("{:?}", p.to_concrete()), "AbsFolder(/dir/)");
}
