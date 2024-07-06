use serde::{Deserialize, Serialize};
use crate::lib::op_param::OpParam;
use crate::lib::req::Req;
use crate::lib::res::Res;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Op {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req: Option<Req>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res: Option<Res>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<OpParam>
}
