use crate::Result;
use std::path::{Path, PathBuf};

pub(crate) trait PathBufExt {
    fn try_to_string(&self) -> Result<String>;
}

impl PathBufExt for PathBuf {
    fn try_to_string(&self) -> Result<String> {
        match self.as_os_str().to_str() {
            Some(s) => Ok(s.to_string()),
            None => Err(format!("non UTF-8 characters in path: {self:?}").into()),
        }
    }
}

pub(crate) trait PathExt {
    fn try_to_str(&self) -> Result<&str>;
}

impl PathExt for Path {
    fn try_to_str(&self) -> Result<&str> {
        match self.as_os_str().to_str() {
            Some(s) => Ok(s),
            None => Err(format!("non UTF-8 characters in path: {self:?}").into()),
        }
    }
}
