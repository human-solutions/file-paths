use std::path::{Path, PathBuf};

use anyhow::bail;

pub(crate) trait PathBufExt {
    fn try_to_string(&self) -> anyhow::Result<String>;
}

impl PathBufExt for PathBuf {
    fn try_to_string(&self) -> anyhow::Result<String> {
        let Some(s) = self.as_os_str().to_str() else {
            bail!("Non UTF-8 characters in path: {}", self.to_string_lossy())
        };
        Ok(s.to_string())
    }
}

pub(crate) trait PathExt {
    fn try_to_str<'a>(&'a self) -> anyhow::Result<&'a str>;
}

impl PathExt for Path {
    fn try_to_str<'a>(&'a self) -> anyhow::Result<&'a str> {
        let Some(s) = self.as_os_str().to_str() else {
            bail!("Non UTF-8 characters in path: {}", self.to_string_lossy())
        };
        Ok(s)
    }
}
