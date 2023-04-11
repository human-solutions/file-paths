use anyhow::Result;

pub fn home_dir() -> Result<String> {
    #[cfg(test)]
    return Ok(String::from(r"C:\User\test\"));
    #[cfg(not(test))]
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
