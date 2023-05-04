use crate::error::ensure;
use crate::os::CurrentOS;
use crate::{
    all_files, all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from,
};
use crate::{AbsoluteFolderPath, AnyFilePath, RelativeFilePath, RelativeFolderPath};
use crate::{AbsolutePath, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbsoluteFilePath(pub(crate) PathInner<CurrentOS>);

all_paths!(AbsoluteFilePath);
all_files!(AbsoluteFilePath);
try_from!(AbsoluteFilePath);
try_exist!(AbsoluteFilePath);
serde_exist!(AbsoluteFilePath);
serde_expanded!(AbsoluteFilePath);

impl AbsoluteFilePath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_absolute()?;
        self.0.ensure_file()?;
        Ok(self)
    }

    pub(crate) fn validate_fs(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure(p.exists(), || format!("file doesn't exist: {}", self.0))?;
        ensure(p.is_file(), || format!("not a file: {}", self.0))?;
        Ok(())
    }

    pub fn exists(&self) -> bool {
        let p = self.0.as_path();
        p.exists() && p.is_file()
    }

    pub fn dropping_file(&self) -> AbsoluteFolderPath {
        AbsoluteFolderPath(self.0.drop_file())
    }

    pub fn removing_root(&self, root: &AbsoluteFolderPath) -> Option<RelativeFilePath> {
        self.0.removing_root(root.as_str()).map(RelativeFilePath)
    }

    pub fn to_relative(self, from_segment_index: usize) -> RelativeFolderPath {
        RelativeFolderPath(self.0.relative_from(from_segment_index))
    }

    pub fn to_absolute(self) -> AbsolutePath {
        self.into()
    }

    pub fn to_any_file(self) -> AnyFilePath {
        AnyFilePath(self.0)
    }
}

#[test]
fn test_convert_to_abstract() {
    let p: AbsoluteFilePath = "/dir/file.txt".try_into().unwrap();

    let abs_path = p.clone().to_absolute();
    assert_eq!(format!("{abs_path:?}"), "AbsolutePath(/dir/file.txt)");

    let abs_path = p.to_any_file();
    assert_eq!(format!("{abs_path:?}"), "AnyFilePath(/dir/file.txt)");
}

#[test]
fn test_convert_to_concrete() {
    let p: AbsoluteFilePath = "/dir/file.txt".try_into().unwrap();

    let rel_file = p.removing_root(&"/dir/".try_into().unwrap()).unwrap();
    assert_eq!(format!("{rel_file:?}"), "RelativeFilePath(file.txt)");

    let root: AbsoluteFolderPath = "/dir/".try_into().unwrap();
    let rel_file = p.removing_root(&root).unwrap();
    assert_eq!(format!("{rel_file:?}"), "RelativeFilePath(file.txt)");

    let rel_file = p.removing_root(&root).unwrap();
    assert_eq!(format!("{rel_file:?}"), "RelativeFilePath(file.txt)");

    let abs_fold = p.dropping_file();
    assert_eq!(format!("{abs_fold:?}"), "AbsoluteFolderPath(/dir/)");
}
