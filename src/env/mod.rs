use anyhow::Result;

#[cfg(unix)]
mod nix;
#[cfg(target_os = "windows")]
mod win;

#[cfg(unix)]
pub(crate) use nix::home_dir;
#[cfg(target_os = "windows")]
pub(crate) use win::home_dir;

#[cfg(not(test))]
use crate::ext::PathBufExt;

pub(crate) fn current_dir() -> Result<String> {
    #[cfg(not(test))]
    return Ok(std::env::current_dir()?.try_to_string()?);
    #[cfg(all(not(windows), test))]
    return Ok(String::from("/var/test"));
    #[cfg(all(windows, test))]
    return Ok(String::from(r"C:\current"));
}

pub(crate) fn env_var(key: &str) -> Result<String> {
    #[cfg(not(test))]
    {
        use anyhow::Context;
        return std::env::var(&key).context(format!("environment variable '{key}' is not defined"));
    }
    #[cfg(test)]
    {
        if key == "FAIL" {
            anyhow::bail!("environment variable '{key}' is not defined")
        }
        let key = key.to_lowercase();
        return Ok(format!("={key}="));
    }
}
