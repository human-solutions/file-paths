use crate::os::CurrentOS;
use crate::{all_dirs, with_file, AbsoluteFolderPath, RelativeFilePath};
use crate::{all_paths, inner::PathInner, try_from};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelativeFolderPath(pub(crate) PathInner<CurrentOS>);

all_paths!(RelativeFolderPath);
all_dirs!(RelativeFolderPath);
try_from!(RelativeFolderPath);
with_file!(RelativeFolderPath, RelativeFilePath);

impl RelativeFolderPath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_relative()?;
        self.0.ensure_folder()?;
        Ok(self)
    }

    pub fn with_root(&self, root: AbsoluteFolderPath) -> AbsoluteFolderPath {
        let path = self.0.path.clone() + &root.0.path;
        let p = PathInner { path, t: self.0.t };
        AbsoluteFolderPath(p)
    }

    pub fn with_root_str(&self, root: &str) -> Result<AbsoluteFolderPath> {
        Ok(self.with_root(root.try_into()?))
    }
}
