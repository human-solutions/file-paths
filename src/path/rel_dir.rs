use crate::os::CurrentOS;
use crate::{all_dirs, AbsDir, RelFile};
use crate::{all_paths, inner::PathInner, try_from};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelDir(pub(crate) PathInner<CurrentOS>);

all_paths!(RelDir);
all_dirs!(RelDir);
try_from!(RelDir);

impl RelDir {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_relative()?;
        self.0.ensure_dir()?;
        Ok(self)
    }

    pub fn with_root(&self, root: AbsDir) -> AbsDir {
        let path = self.0.path.clone() + &root.0.path;
        let p = PathInner { path, t: self.0.t };
        AbsDir(p)
    }

    pub fn with_file(&self, file: RelFile) -> RelFile {
        let path = self.0.path.clone() + &file.0.path;
        let p = PathInner { path, t: self.0.t };
        RelFile(p)
    }
}
