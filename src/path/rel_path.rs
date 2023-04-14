use crate::{all_paths, inner::PathInner, try_from};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelPath(pub(crate) PathInner);

all_paths!(RelPath);
try_from!(RelPath);

impl RelPath {
    pub(crate) fn validate(self) -> Result<Self> {
        ensure!(!self.0.is_absolute(), "path is not relative: {self}");
        Ok(self)
    }
}
