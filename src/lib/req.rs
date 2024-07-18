use crate::lib::desc::Desc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Req {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(flatten)]
    pub desc: Desc,
}
