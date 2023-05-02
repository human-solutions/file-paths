use crate::os::CurrentOS;
use crate::{all_dirs, AbsoluteFolderPath, RelativeFilePath};
use crate::{all_paths, inner::PathInner, try_from};
use crate::{PathError, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelativeFolderPath(pub(crate) PathInner<CurrentOS>);

all_paths!(RelativeFolderPath);
all_dirs!(RelativeFolderPath);
try_from!(RelativeFolderPath);

impl RelativeFolderPath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_relative()?;
        self.0.ensure_folder()?;
        Ok(self)
    }

    pub fn with_root<P>(&self, root: P) -> Result<AbsoluteFolderPath>
    where
        P: TryInto<AbsoluteFolderPath, Error = PathError>,
    {
        let root: AbsoluteFolderPath = root.try_into()?;
        let path = self.0.path.clone() + &root.0.path;
        path.try_into()
    }

    pub fn with_file<F>(&self, file: F) -> Result<RelativeFilePath>
    where
        F: TryInto<RelativeFilePath, Error = PathError>,
    {
        let file: RelativeFilePath = file.try_into()?;
        let path = self.0.path.clone() + &file.0.path;
        path.try_into()
    }
}
