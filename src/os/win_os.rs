use super::OsGroup;
use crate::ext::PathBufExt;
use anyhow::Result;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct WinOS {}

impl OsGroup for WinOS {
    const SEP: char = '\\';

    fn home() -> Result<String> {
        home_dir()
    }

    fn current() -> Result<String> {
        Ok(std::env::current_dir()?.try_to_string()?)
    }

    fn drive_letter() -> Result<char> {
        use anyhow::bail;

        let cwd = std::env::current_dir()?.try_to_string()?;
        match crate::os::drive::win_drive(&cwd) {
            Some(drive) => Ok(drive),
            None => bail!("could not extract drive letter from {cwd}"),
        }
    }

    fn is_absolute(path: &str) -> bool {
        super::is_absolute_win(path)
    }

    fn start_of_relative_path(path: &str) -> usize {
        super::start_of_relative_part_win(path)
    }

    fn process_drive_letter<'a>(path: &'a str, inner: &mut String) -> Result<&'a str> {
        let drive = Self::drive_letter()?;
        Ok(super::drive::add_win_drive(path, drive, inner))
    }
}

pub fn home_dir() -> Result<String> {
    unsafe {
        use anyhow::bail;
        use std::ffi::c_void;
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use std::slice;
        use windows_sys as windows;

        use windows::Win32;
        use windows::Win32::UI::Shell;

        let mut path_ptr: windows::core::PWSTR = std::ptr::null_mut();
        let result = Shell::SHGetKnownFolderPath(
            &Shell::FOLDERID_Profile,
            0,
            Win32::Foundation::HANDLE::default(),
            &mut path_ptr,
        );
        if result == 0 {
            let len = windows::Win32::Globalization::lstrlenW(path_ptr) as usize;
            let path = slice::from_raw_parts(path_ptr, len);
            let os_str: OsString = OsStringExt::from_wide(path);
            windows::Win32::System::Com::CoTaskMemFree(path_ptr as *const c_void);
            match os_str.into_string() {
                Ok(mut s) => {
                    if !s.ends_with(['\\']) {
                        s.push('\\');
                    }
                    Ok(s)
                }
                Err(s) => bail!(
                    "invalid characters in user home directory: {}",
                    s.to_string_lossy()
                ),
            }
        } else {
            windows::Win32::System::Com::CoTaskMemFree(path_ptr as *const c_void);
            bail!("could not resolve the user's home directory")
        }
    }
}
