use crate::SLASH;

use super::CharExt;

use anyhow::{bail, ensure, Result};

pub(crate) trait PathStrExt {
    fn is_absolute(&self) -> bool;
    fn assert_allowed_path_component(&self) -> Result<()>;
    fn assert_allowed_file_name(&self) -> Result<()>;
    fn after_last_slash_from(&self, pos: usize) -> usize;
    fn first_dot_from(&self, pos: usize) -> usize;
}

impl PathStrExt for str {
    fn after_last_slash_from(&self, pos: usize) -> usize {
        pos + self[pos..].rfind(SLASH).map(|i| i + 1).unwrap_or(0)
    }

    fn first_dot_from(&self, pos: usize) -> usize {
        self[pos..]
            .find('.')
            .map(|i| i + pos)
            .unwrap_or_else(|| self.len())
    }

    fn is_absolute(&self) -> bool {
        self.starts_with(SLASH) || (self.len() >= 3 && [":/", ":\\"].contains(&&self[1..3]))
    }

    fn assert_allowed_path_component(&self) -> Result<()> {
        ensure!(
            self.len() <= 255,
            "path components can have a maximum length of 255 characters but this was {}: {self}",
            self.len(),
        );
        for c in self.chars() {
            if c.is_forbidden_in_path() {
                if c.is_ascii_control() {
                    bail!(
                        "forbidden ascii control character {:#x} in path segment: {self}",
                        c as i32
                    );
                } else {
                    bail!("forbidden ascii character {c} in path segment: {self}")
                }
            }
        }
        Ok(())
    }

    fn assert_allowed_file_name(&self) -> Result<()> {
        ensure!(!self.is_empty(), "An empty filename is not valid");
        ensure!(
            self.find(SLASH).is_none(),
            "A file name cannot contain slashes: {self}"
        );
        Ok(())
    }
}
