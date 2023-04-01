#![allow(unused_imports)]

use anyhow::Result;

#[cfg(unix)]
mod nix;
#[cfg(target_os = "windows")]
mod win;

#[cfg(unix)]
use nix as dir;
#[cfg(target_os = "windows")]
use win as dir;

#[cfg(not(test))]
use crate::ext::PathBufExt;

pub(crate) fn current_dir() -> Result<String> {
    #[cfg(not(test))]
    return Ok(std::env::current_dir()?.try_to_string()?);
    #[cfg(test)]
    Ok(String::from("/var/test"))
}

pub(crate) fn home_dir() -> Result<String> {
    #[cfg(not(test))]
    return Ok(dir::home_dir()
        .ok_or(anyhow::anyhow!(
            "could not resolve the user's home directory"
        ))?
        .try_to_string()?);
    #[cfg(test)]
    Ok(String::from("/home/test"))
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
