use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{AbsoluteFolderPath, RelativeFolderPath};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelativeFilePath(pub(crate) PathInner<CurrentOS>);

all_paths!(RelativeFilePath);
try_from!(RelativeFilePath);

impl RelativeFilePath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_relative()?;
        self.0.ensure_file()?;
        Ok(self)
    }

    pub fn drop_file(&mut self) -> RelativeFolderPath {
        RelativeFolderPath(self.0.drop_file())
    }

    pub fn with_root(&self, root: AbsoluteFolderPath) -> AbsoluteFolderPath {
        AbsoluteFolderPath(self.0.with_root(root.as_str()))
    }
}
