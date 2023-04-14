use anyhow::Result;

pub(crate) fn var(key: &str) -> Result<String> {
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
