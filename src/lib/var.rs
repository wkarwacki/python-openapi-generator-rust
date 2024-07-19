use crate::lib::desc::Desc;
use serde::{Deserialize, Serialize};
use std::ops::Not;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Var {
    #[serde(flatten)]
    pub desc: Desc,
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub opt: bool,
}
