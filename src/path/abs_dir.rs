use crate::os::CurrentOS;
use crate::{all_dirs, RelDir};
use crate::{
    all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from, AbsFile, RelFile,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbsDir(pub(crate) PathInner<CurrentOS>);

all_paths!(AbsDir);
all_dirs!(AbsDir);
try_from!(AbsDir);
try_exist!(AbsDir);
serde_exist!(AbsDir);
serde_expanded!(AbsDir);

impl AbsDir {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_absolute()?;
        self.0.ensure_dir()?;
        Ok(self)
    }

    pub fn exists(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure!(p.exists(), "dir doesn't exist: {}", self.0);
        ensure!(p.is_dir(), "not a directory: {}", self.0);
        Ok(())
    }

    pub fn with_file(self, file: RelFile) -> AbsFile {
        AbsFile(self.0.appending(&file.0.path))
    }

    pub fn remove_root(&self, root: AbsDir) -> Option<RelDir> {
        self.0.remove_root(&root.0.path).map(RelDir)
    }
}
