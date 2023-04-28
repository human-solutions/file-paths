use crate::os::CurrentOS;
use crate::{all_dirs, RelativeFolderPath};
use crate::{
    all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from,
    AbsoluteFilePath, RelativeFilePath,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbsoluteFolderPath(pub(crate) PathInner<CurrentOS>);

all_paths!(AbsoluteFolderPath);
all_dirs!(AbsoluteFolderPath);
try_from!(AbsoluteFolderPath);
try_exist!(AbsoluteFolderPath);
serde_exist!(AbsoluteFolderPath);
serde_expanded!(AbsoluteFolderPath);

impl AbsoluteFolderPath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_absolute()?;
        self.0.ensure_folder()?;
        Ok(self)
    }

    pub fn exists(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure!(p.exists(), "dir doesn't exist: {}", self.0);
        ensure!(p.is_dir(), "not a directory: {}", self.0);
        Ok(())
    }

    pub fn with_file(self, file: RelativeFilePath) -> AbsoluteFilePath {
        AbsoluteFilePath(self.0.appending(&file.0.path))
    }

    pub fn remove_root(&self, root: AbsoluteFolderPath) -> Option<RelativeFolderPath> {
        self.0.remove_root(&root.0.path).map(RelativeFolderPath)
    }
}
