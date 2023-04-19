use crate::os::CurrentOS;
use crate::{
    all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from, AbsFile, RelFile,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbsDir(pub(crate) PathInner<CurrentOS>);

all_paths!(AbsDir);
try_from!(AbsDir);
try_exist!(AbsDir);
serde_exist!(AbsDir);
serde_expanded!(AbsDir);

impl AbsDir {
    pub(crate) fn validate(self) -> Result<Self> {
        ensure!(self.0.is_absolute(), "path is not absolute: {self}");
        Ok(self)
    }

    pub fn exists(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure!(p.exists(), "dir doesn't exist: {}", self.0);
        ensure!(p.is_dir(), "not a directory: {}", self.0);
        Ok(())
    }

    pub fn to_file(self, with_file: RelFile) -> Result<AbsFile> {
        AbsFile::try_from(self.0.path + &with_file.0.path)
    }
}
