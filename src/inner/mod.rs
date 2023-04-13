#[cfg(all(test, not(windows)))]
mod test_lin;
#[cfg(all(test, windows))]
mod test_win;

mod drive;
mod envs;
mod path;
mod public;
mod traits;

pub(crate) use envs::expand_envs;
pub(crate) use path::PathInner;
pub use traits::TryExist;
