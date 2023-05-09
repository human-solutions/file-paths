use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{serde_impl, AnyPath, Result};
use crate::{RelativeFilePath, RelativeFolderPath};
use either::Either;

#[derive(Clone)]
pub struct RelativePath(pub(crate) PathInner<CurrentOS>);

all_paths!(RelativePath);
try_from!(RelativePath);
serde_impl!(RelativePath);

impl RelativePath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_relative()?;
        Ok(self)
    }

    pub fn to_concrete(&self) -> Either<RelativeFolderPath, RelativeFilePath> {
        match self.0.is_folder() {
            true => Either::Left(RelativeFolderPath(self.0.clone())),
            false => Either::Right(RelativeFilePath(self.0.clone())),
        }
    }

    pub fn to_any(&self) -> AnyPath {
        AnyPath(self.0.clone())
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

#[test]
fn test_convert_to_abstract() {
    let p: RelativePath = "dir1/dir2/".try_into().unwrap();

    let any_path = p.to_any();
    assert_eq!(format!("{any_path:#?}"), "AnyPath(dir1/dir2/)");
}

#[test]
fn test_convert_to_concrete() {
    let p: RelativePath = "dir1/dir2/".try_into().unwrap();
    let abs_fold = p.to_concrete().unwrap_left();
    assert_eq!(format!("{abs_fold:#?}"), "RelativeFolderPath(dir1/dir2/)");

    let p: RelativePath = "dir1/file".try_into().unwrap();
    let abs_file = p.to_concrete().unwrap_right();
    assert_eq!(format!("{abs_file:#?}"), "RelativeFilePath(dir1/file)");
}

#[test]
fn test_from_concrete() {
    let rel_file: RelativeFilePath = "dir/file".try_into().unwrap();
    let _p: RelativePath = rel_file.into();

    let rel_file: RelativeFolderPath = "dir/dir/".try_into().unwrap();
    let _p: RelativePath = rel_file.into();
}
