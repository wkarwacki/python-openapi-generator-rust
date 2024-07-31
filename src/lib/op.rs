use crate::lib::{op_param::OpParam, r#ref::Ref, req::Req, res::Res};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Op {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req: Option<Req>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res: Option<Res>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<OpParam>,
}

impl Op {
    pub(crate) fn refs(&self) -> Vec<Ref> {
        let mut refs = Vec::new();
        self.req.iter().for_each(|req| refs.extend(req.desc.refs()));
        self.res.iter().for_each(|res| refs.extend(res.desc.refs()));
        self.params
            .iter()
            .for_each(|param| refs.extend(param.desc.refs()));
        refs
    }
}
