use serde::{Deserialize, Serialize};
use crate::lib::carrier::Carrier;
use crate::lib::desc::Desc;
use crate::lib::meta::Meta;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Res {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(flatten)]
    pub desc: Desc,
    #[serde(default)]
    #[serde(skip_serializing_if = "Carrier::is_default")]
    pub carrier: Carrier,
    #[serde(skip_serializing_if = "Meta::is_empty")]
    #[serde(default)]
    pub meta: Meta
}
