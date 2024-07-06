use crate::lib::{def::Def, op::Op};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Pkg {
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub ops: HashMap<String, Vec<Op>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub defs: HashMap<String, Def>,
}
