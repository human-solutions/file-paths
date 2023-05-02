use crate::ext::PathBufExt;
use crate::os::OsGroup;
use crate::Result;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LinOS {}

impl OsGroup for LinOS {
    const SEP: char = '/';
    const SEP_STR: &'static str = "/";

    fn home() -> Result<String> {
        home_dir()
    }

    fn current() -> Result<String> {
        std::env::current_dir()?.try_to_string()
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
        Ok(super::drive::remove_win_drive(path))
    }
}

// https://github.com/rust-lang/rust/blob/2682b88c526d493edeb2d3f2df358f44db69b73f/library/std/src/sys/unix/os.rs#L595
pub fn home_dir() -> Result<String> {
    use std::env;
    use std::ffi::{CStr, OsString};
    use std::mem;
    use std::os::unix::ffi::OsStringExt;
    use std::ptr;

    #[allow(unreachable_code)]
    let os_str = env::var_os("HOME")
        .and_then(|h| if h.is_empty() { None } else { Some(h) })
        .or_else(|| unsafe { fallback() });

    return if let Some(os_str) = os_str {
        os_str
            .into_string()
            .map_err(|s| format!("invalid characters in user home directory: {s:?}").into())
    } else {
        Err("could not resolve the user's home directory".into())
    };

    #[cfg(any(target_os = "android", target_os = "ios", target_os = "emscripten"))]
    unsafe fn fallback() -> Option<OsString> {
        None
    }
    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "emscripten")))]
    unsafe fn fallback() -> Option<OsString> {
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut passwd: libc::passwd = mem::zeroed();
        let mut result = ptr::null_mut();
        match libc::getpwuid_r(
            libc::getuid(),
            &mut passwd,
            buf.as_mut_ptr(),
            buf.capacity(),
            &mut result,
        ) {
            0 if !result.is_null() => {
                let ptr = passwd.pw_dir as *const _;
                let bytes = CStr::from_ptr(ptr).to_bytes();
                if bytes.is_empty() {
                    None
                } else {
                    Some(OsStringExt::from_vec(bytes.to_vec()))
                }
            }
            _ => None,
        }
    }
}
