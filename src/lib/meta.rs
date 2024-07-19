use crate::lib::desc::Desc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Meta {
    #[serde(flatten)]
    pub value: HashMap<String, Desc>,
}

impl Meta {
    pub(crate) fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}
