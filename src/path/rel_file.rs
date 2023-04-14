use crate::{all_paths, inner::PathInner, try_from};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelFile(pub(crate) PathInner);

all_paths!(RelFile);
try_from!(RelFile);

impl RelFile {
    pub(crate) fn validate(self) -> Result<Self> {
        ensure!(!self.0.is_absolute(), "file path is not relative: {self}");
        Ok(self)
    }
}
