use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum PathError {
    Error(String),
    Context(String, Box<Self>),
}
impl Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error(err) => write!(f, "{err}"),
            Self::Context(ctx, err) => write!(f, "{ctx}: {err}"),
        }
    }
}
impl Error for PathError {}
pub(crate) fn ensure<E: Into<PathError>, F: FnOnce() -> E>(
    cond: bool,
    err: F,
) -> crate::Result<()> {
    if cond {
        Ok(())
    } else {
        Err(err().into())
    }
}

impl From<&str> for PathError {
    fn from(value: &str) -> Self {
        Self::Error(value.to_string())
    }
}
impl From<String> for PathError {
    fn from(value: String) -> Self {
        Self::Error(value)
    }
}

impl From<std::io::Error> for PathError {
    fn from(value: std::io::Error) -> Self {
        Self::Error(value.to_string())
    }
}

impl From<PathError> for String {
    fn from(value: PathError) -> Self {
        value.to_string()
    }
}

pub(crate) trait Context<T> {
    fn context<S: AsRef<str>>(self, msg: S) -> crate::Result<T>;
}

impl<T> Context<T> for crate::Result<T> {
    fn context<S: AsRef<str>>(self, msg: S) -> crate::Result<T> {
        match self {
            Ok(me) => Ok(me),
            Err(e) => Err(PathError::Context(msg.as_ref().to_string(), Box::new(e))),
        }
    }
}
