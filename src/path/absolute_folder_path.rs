use crate::os::CurrentOS;
use crate::{all_dirs, AbsoluteFilePath, AbsolutePath, RelativeFilePath, RelativeFolderPath};
use crate::{all_paths, inner::PathInner, serde_exist, serde_expanded, try_exist, try_from};
use crate::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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

    pub fn removing_root(&self, root: &AbsoluteFolderPath) -> Result<RelativeFolderPath> {
        self.0.removing_root(&root.0.path).map(RelativeFolderPath)
    }

    /// Convert this [AbsoluteFolderPath] to an [AbsoluteFilePath] by providing a [RelativeFilePath].
    ///
    /// ```rust
    /// # use x_path::{AbsoluteFolderPath, RelativeFilePath};
    /// # fn main() -> Result<(), String> {
    /// let folder: AbsoluteFolderPath = "./src/".try_into()?;
    /// let cargo_location: RelativeFilePath = "../Cargo.toml".try_into()?;
    /// let cargo_path = folder.with_file(&cargo_location);
    ///
    /// // alternatively, you can convert on the fly into a RelativeFilePath like this:
    /// let lib_file = folder.with_file(&"lib.rs".try_into()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_file(&self, file: &RelativeFilePath) -> AbsoluteFilePath {
        AbsoluteFilePath(self.0.with_path_appended(file.as_str()))
    }

    pub fn with_folder(&self, folder: &RelativeFolderPath) -> Self {
        Self(self.0.with_path_appended(folder.as_str()))
    }

    pub fn to_absolute(&self) -> AbsolutePath {
        self.clone().into()
    }
}

#[test]
fn test_convert_to_abstract() {
    let p: AbsoluteFolderPath = "/dir1/dir2/".try_into().unwrap();

    let abs_path = p.to_absolute();
    assert_eq!(format!("{abs_path:?}"), "AbsolutePath(/dir1/dir2/)");
}

#[test]
fn test_convert_to_concrete() {
    let p: AbsoluteFolderPath = "/dir1/dir2/".try_into().unwrap();

    let rel_file = p.removing_root(&"/dir1/".try_into().unwrap()).unwrap();
    assert_eq!(format!("{rel_file:?}"), "RelativeFolderPath(dir2/)");
}
