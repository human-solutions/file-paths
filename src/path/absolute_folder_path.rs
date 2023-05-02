use crate::os::CurrentOS;
use crate::{all_dirs, PathError, RelativeFolderPath};
use crate::{
    all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from, AbsoluteFilePath,
};
use crate::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbsoluteFolderPath(pub(crate) PathInner<CurrentOS>);

all_paths!(AbsoluteFolderPath);
all_dirs!(AbsoluteFolderPath);
try_from!(AbsoluteFolderPath);
try_exist!(AbsoluteFolderPath);
serde_exist!(AbsoluteFolderPath);
serde_expanded!(AbsoluteFolderPath);

impl AbsoluteFolderPath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_absolute()?;
        self.0.ensure_folder()?;
        Ok(self)
    }

    pub(crate) fn validate_fs(&self) -> Result<()> {
        let p = self.0.as_path();
        ensure(p.exists(), || format!("folder doesn't exist: {}", self.0))?;
        ensure(p.is_dir(), || format!("not a folder: {}", self.0))
    }

    pub fn exists(&self) -> bool {
        let p = self.0.as_path();
        p.exists() && p.is_dir()
    }

    pub fn removing_root(&self, root: AbsoluteFolderPath) -> Option<RelativeFolderPath> {
        self.0.remove_root(&root.0.path).map(RelativeFolderPath)
    }

    pub fn with_file<F>(&self, file: F) -> std::result::Result<AbsoluteFilePath, PathError>
    where
        F: TryInto<AbsoluteFilePath, Error = PathError>,
    {
        let file: AbsoluteFilePath = file.try_into()?;
        let path = self.0.path.clone() + &file.0.path;
        path.try_into()
    }
}
