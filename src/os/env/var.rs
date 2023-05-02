use crate::Result;

pub(crate) fn var(key: &str) -> Result<String> {
    #[cfg(not(test))]
    {
        std::env::var(key)
            .map_err(|e| format!("environment variable '{key}' is not defined: {e}").into())
    }
    #[cfg(test)]
    {
        if key == "FAIL" {
            return Err(format!("environment variable '{key}' is not defined").into());
        }
        let key = key.to_lowercase();
        Ok(format!("={key}="))
    }
}
