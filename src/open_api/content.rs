use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::context::Context;
use crate::open_api::media_type::MediaType;
use crate::open_api::mime::Mime;
use crate::req::Req;
use crate::res::Res;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Content {
    #[serde(flatten)]
    pub val: HashMap<Mime, MediaType>
}

impl Content {
    pub fn of_req(req: &Req, context: &Context) -> Content {
        let mut val = HashMap::new();
        val.insert(Mime::of(req.form.clone()), MediaType::of(req.desc.clone(), context));
        Content {
            val: val
        }
    }
    pub fn of_res(res: &Res, context: &Context) -> Content {
        let mut val = HashMap::new();
        val.insert(Mime::of(res.form.clone()), MediaType::of(res.desc.clone(), context));
        Content {
            val: val
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Content {
            val: HashMap::new()
        }
    }
}
