use crate::{inner::PathInner, iter::Segments, try_from};
use anyhow::Result;
use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    str::Chars,
};

pub trait PushSeg: DerefMut<Target = PathInner> {
    fn push(&mut self, segment: &str) -> Result<()> {
        self.deref_mut().push_segment(segment)
    }
}

pub struct AnyPath(PathInner);

impl Deref for AnyPath {
    type Target = PathInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AnyPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PushSeg for AnyPath {}

impl AnyPath {
    pub fn segments(&self) -> Segments {
        self.0.segments()
    }

    pub fn chars(&self) -> Chars {
        self.0.chars()
    }
}

try_from!(AnyPath);
