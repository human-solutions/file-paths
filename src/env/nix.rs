use anyhow::Result;

// https://github.com/rust-lang/rust/blob/2682b88c526d493edeb2d3f2df358f44db69b73f/library/std/src/sys/unix/os.rs#L595
pub fn home_dir() -> Result<String> {
    #[cfg(test)]
    return Ok(String::from("/home/test"));

    #[cfg(not(test))]
    {
        use anyhow::bail;
        use std::env;
        use std::ffi::{CStr, OsString};
        use std::mem;
        use std::os::unix::ffi::OsStringExt;
        use std::ptr;

        #[allow(unreachable_code)]
        let os_str = env::var_os("HOME")
            .and_then(|h| if h.is_empty() { None } else { Some(h) })
            .or_else(|| unsafe { fallback() });

        if let Some(os_str) = os_str {
            match os_str.into_string() {
                Ok(s) => return Ok(s),
                Err(s) => bail!(
                    "invalid characters in user home directory: {}",
                    s.to_string_lossy()
                ),
            }
        } else {
            bail!("could not resolve the user's home directory")
        }

        #[cfg(any(target_os = "android", target_os = "ios", target_os = "emscripten"))]
        unsafe fn fallback() -> Option<OsString> {
            None
        }
        #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "emscripten")))]
        unsafe fn fallback() -> Option<OsString> {
            let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
                n if n < 0 => 512 as usize,
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
}
