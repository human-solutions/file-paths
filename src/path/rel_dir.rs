use crate::{all_paths, inner::PathInner, try_from};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelDir(pub(crate) PathInner);

all_paths!(RelDir);
try_from!(RelDir);

impl RelDir {
    pub(crate) fn validate(self) -> Result<Self> {
        ensure!(!self.0.is_absolute(), "directory is not relative: {self}");
        Ok(self)
    }
}
