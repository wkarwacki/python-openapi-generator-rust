use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::desc::Desc;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Meta {
    #[serde(flatten)]
    pub value: HashMap<String, Desc>
}

impl Default for Meta{
    fn default() -> Self {
        Meta {
            value: HashMap::default()
        }
    }
}

impl Meta {
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}
