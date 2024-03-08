use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::desc::Desc;

use crate::r#ref::Ref;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ext {
    #[serde(flatten)]
    pub r#ref: Ref,
    pub args: HashMap<String, Desc>
}
