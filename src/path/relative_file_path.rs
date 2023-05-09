use crate::os::CurrentOS;
use crate::{all_files, serde_impl, AbsoluteFilePath, AbsoluteFolderPath, RelativeFolderPath};
use crate::{all_paths, inner::PathInner, try_from};
use crate::{RelativePath, Result};

#[derive(Clone)]
pub struct RelativeFilePath(pub(crate) PathInner<CurrentOS>);

all_paths!(RelativeFilePath);
all_files!(RelativeFilePath);
try_from!(RelativeFilePath);
serde_impl!(RelativeFilePath);

impl RelativeFilePath {
    pub(crate) fn validate(self) -> Result<Self> {
        self.0.ensure_relative()?;
        self.0.ensure_file()?;
        Ok(self)
    }

    pub fn dropping_file(&self) -> RelativeFolderPath {
        RelativeFolderPath(self.0.drop_file())
    }

    pub fn with_root(&self, root: &AbsoluteFolderPath) -> AbsoluteFilePath {
        AbsoluteFilePath(self.0.with_root(root.as_str()))
    }

    pub fn to_relative(&self) -> RelativePath {
        self.clone().into()
    }
}

impl AsRef<RelativeFilePath> for RelativeFilePath {
    fn as_ref(&self) -> &RelativeFilePath {
        self
    }
}

#[test]
fn test_convert_to_abstract() {
    let p: RelativeFilePath = "dir/file.txt".try_into().unwrap();

    let abs_path = p.to_relative();
    assert_eq!(format!("{abs_path:#?}"), "RelativePath(dir/file.txt)");
}

#[test]
fn test_convert_to_concrete() {
    let p: RelativeFilePath = "dir/file.txt".try_into().unwrap();

    let rel_file = p.with_root(&"/dir/".try_into().unwrap());
    assert_eq!(
        format!("{rel_file:#?}"),
        "AbsoluteFilePath(/dir/dir/file.txt)"
    );

    let abs_fold = p.dropping_file();
    assert_eq!(format!("{abs_fold:#?}"), "RelativeFolderPath(dir/)");
}
