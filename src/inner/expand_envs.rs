use std::borrow::Cow;

use anyhow::{ensure, Context, Result};

use crate::{ext::CharExt, SEP, SLASH};

enum Start {
    Home,
    Current,
    None,
}

impl Start {
    fn from(path: &str) -> Self {
        if path == "~" || path.starts_with("~/") || path.starts_with("~\\") {
            Self::Home
        } else if path == "." || path.starts_with("./") || path.starts_with(".\\") {
            Self::Current
        } else {
            Self::None
        }
    }
}

pub(crate) fn expand_envs<'a>(path: &'a str) -> Result<Cow<str>> {
    let start = Start::from(&path);

    let path: Cow<str> = match start {
        Start::Current => prefix_current_dir(&path[1..])?,
        Start::Home => prefix_home_dir(&path[1..])?,
        Start::None if !path.contains(['$', '%']) => return Ok(Cow::Borrowed(path)),
        Start::None => Cow::Borrowed(path),
    };

    let mut chars = path.chars().peekable();

    // set to true because no character also counts.
    let mut prev_slash = true;

    let mut expanded = String::new();

    while let Some(ch) = chars.next() {
        let start_curly = ch == '$' && prev_slash && chars.next_if_eq(&'{').is_some();
        let start_prcnt = ch == '%' && prev_slash;

        if start_curly || start_prcnt {
            let mut key = start_curly.then_some("${").unwrap_or("%").to_string();

            while let Some(ch) = chars.next() {
                key.push(ch);

                let end_curly = start_curly && ch == '}';
                let end_prcnt = start_prcnt && ch == '%';

                if end_curly || end_prcnt {
                    // a valid env var end is either with a slash or nothing.
                    let valid_end = chars.peek().map(|c| c.is_slash()).unwrap_or(true);
                    if valid_end {
                        let start = if start_curly { 2 } else { 1 };
                        let end = key.len() - 1;

                        ensure!(
                            end - start > 0,
                            "empty environment variable in path: {path}"
                        );

                        expanded.extend(env_var(&key[start..end])?.drain(..));
                        key.clear();
                    }
                    break;
                }

                if ch.is_slash() || !ch.is_allowed_in_environment_var() {
                    break;
                }
            }
            expanded.extend(key.drain(..));
        } else {
            expanded.push(ch);
        }

        prev_slash = ch.is_slash();
    }
    Ok(Cow::Owned(expanded))
}

fn prefix_current_dir<'a>(path: &'a str) -> Result<Cow<'a, str>> {
    let mut cwd = current_dir().context("could not resolve the current working directory")?;
    if !cwd.ends_with(SLASH) && !path.starts_with(SLASH) {
        cwd.push(SEP);
    }
    cwd.extend(path.chars());
    Ok(Cow::Owned(cwd))
}

fn prefix_home_dir<'a>(path: &'a str) -> Result<Cow<'a, str>> {
    let mut home = home_dir().context("could not resolve the current working directory")?;
    if !home.ends_with(SLASH) && !path.starts_with(SLASH) {
        home.push(SEP);
    }
    home.extend(path.chars());
    Ok(Cow::Owned(home))
}

#[cfg(not(test))]
use crate::ext::PathBufExt;

fn current_dir() -> Result<String> {
    #[cfg(not(test))]
    return Ok(std::env::current_dir()?.try_to_string()?);
    #[cfg(test)]
    Ok(String::from("/var/test"))
}

fn home_dir() -> Result<String> {
    #[cfg(not(test))]
    return Ok(dirs_sys::home_dir()
        .ok_or(anyhow::anyhow!(
            "could not resolve the user's home directory"
        ))?
        .try_to_string()?);
    #[cfg(test)]
    Ok(String::from("/home/test"))
}

fn env_var(key: &str) -> Result<String> {
    #[cfg(not(test))]
    return std::env::var(&key).context(format!("environment variable '{key}' is not defined"));
    #[cfg(test)]
    {
        if key == "FAIL" {
            anyhow::bail!("environment variable '{key}' is not defined")
        }
        let key = key.to_lowercase();
        return Ok(format!("={key}="));
    }
}

#[test]
fn exp_envs() {
    assert_eq!(exp_ok("$HI"), "$HI");

    assert_eq!(exp_ok("${HI}"), "=hi=");
    assert_eq!(exp_ok("/${HI}"), "/=hi=");
    assert_eq!(exp_ok("/${HI}/"), "/=hi=/");

    assert_eq!(exp_ok("%HI%"), "=hi=");
    assert_eq!(exp_ok("/%HI%"), "/=hi=");
    assert_eq!(exp_ok("/%HI%/"), "/=hi=/");

    assert_eq!(exp_ok("."), "/var/test/");
    assert_eq!(exp_ok("./"), "/var/test/");
    assert_eq!(exp_ok("./dir"), "/var/test/dir");

    assert_eq!(exp_ok("~"), "/home/test/");
    assert_eq!(exp_ok("~/"), "/home/test/");
    assert_eq!(exp_ok("~/dir"), "/home/test/dir");

    // not expanded
    assert_eq!(exp_ok("/s$HI$"), "/s$HI$");
    assert_eq!(exp_ok("/%$HI"), "/%$HI");
    assert_eq!(exp_ok("/${HI"), "/${HI");
    assert_eq!(exp_ok("/${H-}"), "/${H-}");
    assert_eq!(exp_ok("/${H}s"), "/${H}s");
    assert_eq!(exp_ok("/%H%s"), "/%H%s");
    assert_eq!(exp_ok("/$"), "/$");

    assert_eq!(exp_ok("/dir1/./dir2"), "/dir1/./dir2");
    assert_eq!(exp_ok("dir1/./dir2"), "dir1/./dir2");

    assert_eq!(exp_ok("/dir1/~/dir2"), "/dir1/~/dir2");
    assert_eq!(exp_ok("dir1/~/dir2"), "dir1/~/dir2");

    // errors
    assert_eq!(exp_err("/%%"), "empty environment variable in path: /%%");

    assert_eq!(exp_err("/${}"), "empty environment variable in path: /${}");

    assert_eq!(
        exp_err("/${FAIL}"),
        "environment variable 'FAIL' is not defined"
    );
}

#[cfg(test)]
fn exp_ok(path: &str) -> String {
    expand_envs(path.into()).unwrap().into_owned()
}

#[cfg(test)]
fn exp_err(path: &str) -> String {
    expand_envs(path.into()).unwrap_err().to_string()
}
