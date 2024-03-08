use serde::{Deserialize, Serialize};
use crate::desc::Desc;


use serde_yaml::Value;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OpParam {
    pub loc: Option<String>,
    pub name: String,
    #[serde(flatten)]
    pub desc: Desc,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>
}
