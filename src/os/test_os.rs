use super::drive::{add_win_drive, remove_win_drive};
use crate::os::OsGroup;
use anyhow::Result;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinTestOS {}

impl OsGroup for LinTestOS {
    const SEP: char = '/';

    fn current() -> anyhow::Result<String> {
        Ok(String::from("/var/test"))
    }

    fn home() -> anyhow::Result<String> {
        Ok(String::from("/home/test"))
    }

    fn drive_letter() -> Result<char> {
        Ok('C')
    }

    fn is_absolute(path: &str) -> bool {
        super::is_absolute_lin(path)
    }

    fn start_of_relative_path(path: &str) -> usize {
        super::start_of_relative_part_lin(path)
    }
    fn process_drive_letter<'a>(path: &'a str, _inner: &mut String) -> Result<&'a str> {
        Ok(remove_win_drive(path))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WinTestOS {}

impl OsGroup for WinTestOS {
    const SEP: char = '\\';

    fn is_absolute(path: &str) -> bool {
        super::is_absolute_win(path)
    }

    fn start_of_relative_path(path: &str) -> usize {
        super::start_of_relative_part_win(path)
    }
    fn process_drive_letter<'a>(path: &'a str, inner: &mut String) -> Result<&'a str> {
        let drive = Self::drive_letter()?;
        Ok(add_win_drive(path, drive, inner))
    }

    fn current() -> anyhow::Result<String> {
        Ok(String::from(r"C:\current"))
    }
    fn home() -> anyhow::Result<String> {
        Ok(String::from(r"C:\User\test\"))
    }
    fn drive_letter() -> Result<char> {
        Ok('C')
    }
}
