use crate::lib::{carrier::Carrier, desc::Desc, meta::Meta};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Res {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(flatten)]
    pub desc: Desc,
    #[serde(default)]
    #[serde(skip_serializing_if = "Carrier::is_default")]
    pub carrier: Carrier,
    #[serde(skip_serializing_if = "Meta::is_empty")]
    #[serde(default)]
    pub meta: Meta,
}
