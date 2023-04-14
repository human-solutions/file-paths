use super::drive::{add_win_drive, remove_win_drive};
use crate::os::OsGroup;
use anyhow::Result;

#[derive(Clone)]
pub struct LinTestOS {}

impl OsGroup for LinTestOS {
    const SEP: char = '/';

    fn current() -> anyhow::Result<String> {
        Ok(String::from("/var/test"))
    }

    fn home() -> anyhow::Result<String> {
        Ok(String::from("/home/test"))
    }

    fn is_absolute(path: &str) -> bool {
        super::is_absolute_lin(path)
    }
    fn relative_part(path: &str) -> &str {
        super::relative_part_lin(path)
    }
    fn process_drive_letter<'a>(path: &'a str, _inner: &mut String) -> Result<&'a str> {
        Ok(remove_win_drive(&path))
    }
}

#[derive(Clone)]
pub struct WinTestOS {}

impl OsGroup for WinTestOS {
    const SEP: char = '\\';

    fn is_absolute(path: &str) -> bool {
        super::is_absolute_win(path)
    }

    fn relative_part(path: &str) -> &str {
        super::relative_part_win(path)
    }
    fn process_drive_letter<'a>(path: &'a str, inner: &mut String) -> Result<&'a str> {
        add_win_drive(path, inner)
    }

    fn current() -> anyhow::Result<String> {
        Ok(String::from(r"C:\current"))
    }
    fn home() -> anyhow::Result<String> {
        Ok(String::from(r"C:\User\test\"))
    }
}
