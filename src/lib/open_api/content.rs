use crate::lib::{
    context::Context,
    open_api::{media_type::MediaType, mime::Mime},
    req::Req,
    res::Res,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Content {
    #[serde(flatten)]
    pub val: HashMap<Mime, MediaType>,
}

impl Content {
    pub(crate) fn of_req(req: &Req, context: &Context) -> Content {
        let mut val = HashMap::new();
        val.insert(
            Mime::of(req.form.clone()),
            MediaType::of(&req.desc, context),
        );
        Content { val: val }
    }
    pub(crate) fn of_res(res: &Res, context: &Context) -> Content {
        let mut val = HashMap::new();
        val.insert(
            Mime::of(res.form.clone()),
            MediaType::of(&res.desc, context),
        );
        Content { val: val }
    }
}

impl Default for Content {
    fn default() -> Self {
        Content {
            val: HashMap::new(),
        }
    }
}
