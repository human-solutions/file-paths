use std::borrow::Cow;

use crate::Result;

use crate::error::{ensure, Context};
use crate::{ext::CharExt, os::env, SLASH};

use crate::os::OsGroup;

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
pub(crate) fn expand<OS: OsGroup>(path: &str) -> Result<Cow<str>> {
    let start = Start::from(path);

    let path: Cow<str> = match start {
        Start::Current => prefix_current_dir::<OS>(&path[1..])?,
        Start::Home => prefix_home_dir::<OS>(&path[1..])?,
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
            let mut key = (if start_curly { "${" } else { "%" }).to_string();

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

                        ensure(end - start > 0, || {
                            format!("empty environment variable in path: {path}")
                        })?;

                        expanded.extend(env::var(&key[start..end])?.drain(..));
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

fn prefix_current_dir<P: OsGroup>(path: &str) -> Result<Cow<str>> {
    let mut cwd = P::current().context("could not resolve the current working directory")?;
    if !cwd.ends_with(SLASH) && !path.starts_with(SLASH) {
        cwd.push(P::SEP);
    }
    cwd.push_str(path);
    Ok(Cow::Owned(cwd))
}

fn prefix_home_dir<P: OsGroup>(path: &str) -> Result<Cow<str>> {
    let mut home = P::home().context("could not resolve the current working directory")?;
    if !home.ends_with(SLASH) && !path.starts_with(SLASH) {
        home.push(P::SEP);
    }
    home.push_str(path);
    Ok(Cow::Owned(home))
}
