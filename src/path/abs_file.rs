use crate::{all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbsFile(pub(crate) PathInner);

all_paths!(AbsFile);
try_from!(AbsFile);
try_exist!(AbsFile);
serde_exist!(AbsFile);
serde_expanded!(AbsFile);

impl AbsFile {
    pub fn exists(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure!(p.exists(), "file doesn't exist: {}", self.0);
        ensure!(p.is_file(), "file is not a file: {}", self.0);
        Ok(())
    }
}
