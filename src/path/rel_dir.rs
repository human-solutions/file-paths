use crate::{inner::PathInner, try_from};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelDir(pub(crate) PathInner);

try_from!(RelDir);
