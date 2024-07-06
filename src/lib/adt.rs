use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::lib::def::Obj;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Adt {
    pub var: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub map: HashMap<String, Obj>,
}
