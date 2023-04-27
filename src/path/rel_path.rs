use crate::os::CurrentOS;
use crate::{all_paths, inner::PathInner, try_from};
use crate::{RelDir, RelFile};
use anyhow::Result;
use either::Either;
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

    pub fn to_concrete(self) -> Either<RelDir, RelFile> {
        match self.0.is_dir() {
            true => Either::Left(RelDir(self.0)),
            false => Either::Right(RelFile(self.0)),
        }
    }
}
