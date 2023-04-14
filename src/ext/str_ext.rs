use crate::SLASH;

use super::CharExt;

use anyhow::{bail, ensure, Result};

pub(crate) trait PathStrExt {
    fn is_absolute(&self) -> bool;
    fn assert_allowed_path_component(&self) -> Result<()>;
}

impl PathStrExt for str {
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
}
