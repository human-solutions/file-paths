use crate::{all_paths, inner::PathInner, try_from};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyFile(pub(crate) PathInner);

all_paths!(AnyFile);
try_from!(AnyFile);

impl AnyFile {
    pub(crate) fn validate(self) -> Result<Self> {
        Ok(self)
    }
}
