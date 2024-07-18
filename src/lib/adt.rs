use crate::lib::def::Obj;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Adt {
    pub var: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub map: HashMap<String, Obj>,
}
