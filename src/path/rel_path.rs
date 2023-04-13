use crate::{inner::PathInner, try_from};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelPath(pub(crate) PathInner);

try_from!(RelPath);
