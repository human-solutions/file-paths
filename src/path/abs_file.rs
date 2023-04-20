use crate::os::CurrentOS;
use crate::{
    all_files, all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbsFile(pub(crate) PathInner<CurrentOS>);

all_paths!(AbsFile);
all_files!(AbsFile);
try_from!(AbsFile);
try_exist!(AbsFile);
serde_exist!(AbsFile);
serde_expanded!(AbsFile);

impl AbsFile {
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
}
