use std::ffi::c_void;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::slice;

use windows_sys as windows;

use windows::Win32;
use windows::Win32::UI::Shell;

pub fn known_folder(folder_id: windows::core::GUID) -> Option<PathBuf> {
    unsafe {
        let mut path_ptr: windows::core::PWSTR = std::ptr::null_mut();
        let result = Shell::SHGetKnownFolderPath(
            &folder_id,
            0,
            Win32::Foundation::HANDLE::default(),
            &mut path_ptr,
        );
        if result == 0 {
            let len = windows::Win32::Globalization::lstrlenW(path_ptr) as usize;
            let path = slice::from_raw_parts(path_ptr, len);
            let ostr: OsString = OsStringExt::from_wide(path);
            windows::Win32::System::Com::CoTaskMemFree(path_ptr as *const c_void);
            Some(PathBuf::from(ostr))
        } else {
            windows::Win32::System::Com::CoTaskMemFree(path_ptr as *const c_void);
            None
        }
    }
}

pub fn home_dir() -> Option<PathBuf> {
    known_folder(Shell::FOLDERID_Profile)
}
