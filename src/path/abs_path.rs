use crate::{all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbsPath(pub(crate) PathInner);

all_paths!(AbsPath);
try_from!(AbsPath);
try_exist!(AbsPath);
serde_exist!(AbsPath);
serde_expanded!(AbsPath);

impl AbsPath {
    pub fn exists(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure!(p.exists(), "path doesn't exist: {}", self.0);
        Ok(())
    }
}
