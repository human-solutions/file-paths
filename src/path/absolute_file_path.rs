use crate::os::CurrentOS;
use crate::{
    all_files, all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from,
};
use crate::{AbsoluteFolderPath, RelativeFolderPath};
use anyhow::{ensure, Result};
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

    pub fn exists(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure!(p.exists(), "file doesn't exist: {}", self.0);
        ensure!(p.is_file(), "file is not a file: {}", self.0);
        Ok(())
    }

    pub fn drop_file(&self) -> AbsoluteFolderPath {
        AbsoluteFolderPath(self.0.drop_file())
    }

    pub fn remove_root(&self, root: AbsoluteFolderPath) -> Option<RelativeFolderPath> {
        self.0.remove_root(root.as_str()).map(RelativeFolderPath)
    }

    pub fn relative_from(&self, segment: usize) -> RelativeFolderPath {
        RelativeFolderPath(self.0.relative_from(segment))
    }
}
