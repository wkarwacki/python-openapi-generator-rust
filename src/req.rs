use serde::{Deserialize, Serialize};
use crate::desc::Desc;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Req {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(flatten)]
    pub desc: Desc
}
