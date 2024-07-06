use crate::lib::{desc::Desc, r#ref::Ref};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ext {
    #[serde(flatten)]
    pub r#ref: Ref,
    pub args: HashMap<String, Desc>,
}
