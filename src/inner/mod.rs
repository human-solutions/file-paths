#[cfg(all(test, not(windows)))]
mod test_lin;
#[cfg(all(test, windows))]
mod test_win;

mod path;
mod public;
mod traits;

pub(crate) use path::PathInner;
pub use traits::TryExist;
