use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::lib::def::Def;

use crate::lib::op::Op;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Pkg {
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub ops: HashMap<String, Vec<Op>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub defs: HashMap<String, Def>,
}
