use crate::os::CurrentOS;
use crate::Result;
use crate::{all_dirs, AbsoluteFolderPath, RelativeFilePath, RelativePath};
use crate::{all_paths, inner::PathInner, try_from};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelativeFolderPath(pub(crate) PathInner<CurrentOS>);

all_paths!(RelativeFolderPath);
all_dirs!(RelativeFolderPath);
try_from!(RelativeFolderPath);

impl RelativeFolderPath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_relative()?;
        self.0.ensure_folder()?;
        Ok(self)
    }

    pub fn with_root(&self, root: &AbsoluteFolderPath) -> AbsoluteFolderPath {
        root.clone().with_folder(self)
    }

    pub fn with_file(&self, file: &RelativeFilePath) -> RelativeFilePath {
        RelativeFilePath(self.0.with_path_appended(file.as_str()))
    }

    pub fn with_folder(&self, folder: &RelativeFolderPath) -> RelativeFolderPath {
        RelativeFolderPath(self.0.with_path_appended(folder.as_str()))
    }

    pub fn to_relative(self) -> RelativePath {
        RelativePath(self.0)
    }
}

#[test]
fn test_convert_to_abstract() {
    let p: RelativeFolderPath = "dir1/dir2/".try_into().unwrap();

    let abs_path = p.to_relative();
    assert_eq!(format!("{abs_path:?}"), "RelativePath(dir1/dir2/)");
}

#[test]
fn test_convert_to_concrete() {
    let p: RelativeFolderPath = "dir/".try_into().unwrap();

    let abs_folder = p.with_root(&"/root/".try_into().unwrap());
    assert_eq!(format!("{abs_folder:?}"), "AbsoluteFolderPath(/root/dir/)");

    let rel_file = p.with_file(&"file".try_into().unwrap());
    assert_eq!(format!("{rel_file:?}"), "RelativeFilePath(dir/file)");

    let rel_folder = p.with_folder(&"fold/".try_into().unwrap());
    assert_eq!(format!("{rel_folder:?}"), "RelativeFolderPath(dir/fold/)");
}
