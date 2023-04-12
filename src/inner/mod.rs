#[cfg(all(test, not(windows)))]
mod test_lin;
#[cfg(all(test, windows))]
mod test_win;

mod envs;
mod path;

pub(crate) use envs::expand_envs;
pub(crate) use path::PathInner;
