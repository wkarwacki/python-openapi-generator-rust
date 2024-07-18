use crate::lib::desc::Desc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Meta {
    #[serde(flatten)]
    pub value: HashMap<String, Desc>,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            value: HashMap::default(),
        }
    }
}

impl Meta {
    pub(crate) fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}
