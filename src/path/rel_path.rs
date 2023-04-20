use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelPath(pub(crate) PathInner<CurrentOS>);

all_paths!(RelPath);
try_from!(RelPath);

impl RelPath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_relative()?;
        Ok(self)
    }
}
