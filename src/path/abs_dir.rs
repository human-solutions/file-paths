use crate::{all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbsDir(pub(crate) PathInner);

all_paths!(AbsDir);
try_from!(AbsDir);
try_exist!(AbsDir);
serde_exist!(AbsDir);
serde_expanded!(AbsDir);

impl AbsDir {
    pub fn exists(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure!(p.exists(), "dir doesn't exist: {}", self.0);
        ensure!(p.is_dir(), "not a directory: {}", self.0);
        Ok(())
    }
}
