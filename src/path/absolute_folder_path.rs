use crate::os::CurrentOS;
use crate::{all_dirs, with_file, RelativeFolderPath};
use crate::{
    all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from, AbsoluteFilePath,
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
with_file!(AbsoluteFolderPath, AbsoluteFilePath);
serde_exist!(AbsoluteFolderPath);
serde_expanded!(AbsoluteFolderPath);

impl AbsoluteFolderPath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_absolute()?;
        self.0.ensure_folder()?;
        Ok(self)
    }

    pub(crate) fn validate_fs(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure!(p.exists(), "folder doesn't exist: {}", self.0);
        ensure!(p.is_dir(), "not a folder: {}", self.0);
        Ok(())
    }

    pub fn exists(&self) -> bool {
        let p = self.0.as_path();
        p.exists() && p.is_dir()
    }

    pub fn removing_root(&self, root: AbsoluteFolderPath) -> Option<RelativeFolderPath> {
        self.0.remove_root(&root.0.path).map(RelativeFolderPath)
    }
}
