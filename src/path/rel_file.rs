use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{AbsDir, RelDir};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelFile(pub(crate) PathInner<CurrentOS>);

all_paths!(RelFile);
try_from!(RelFile);

impl RelFile {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_relative()?;
        self.0.ensure_file()?;
        Ok(self)
    }

    pub fn drop_file(&mut self) -> RelDir {
        RelDir(self.0.drop_file())
    }

    pub fn with_root(&self, root: AbsDir) -> AbsDir {
        AbsDir(self.0.with_root(root.as_str()))
    }
}
