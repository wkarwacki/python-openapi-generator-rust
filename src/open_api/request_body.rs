use mime;
use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::desc::Desc;
use crate::open_api::context::Context as OpenApiContext;
use crate::open_api::content::Content;
use crate::open_api::open_api::OpenApi;
use crate::open_api::ref_or::RefOr;
use crate::req::Req;
use crate::util;
use crate::validation::ensure;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RequestBody {
    pub content: Content,
    #[serde(default = "util::r#true", skip_serializing_if = "util::val")]
    pub required: bool,
}

impl RequestBody {
    pub fn of(req: &Req, context: &Context) -> RequestBody {
        RequestBody {
            content: Content::of_req(req, context),
            required: true,
        }
    }
    pub fn req(&self, context: &OpenApiContext) -> Req {
        let entries = self.clone().content.val;
        ensure(|| entries.len() == 1);
        let entry = entries.iter().next().unwrap();
        Req {
            form: {
                let mime = entry.0.clone().val;
                if mime == mime::APPLICATION_JSON { None } else { Some(mime.to_string()) }
            },
            desc: match entry.1.clone().schema {
                RefOr::Ref { r#ref } => Desc::Ref(OpenApi::trust_ref(r#ref)),
                RefOr::Item(schema) => schema.clone().desc("req".to_string(), context)
            }
        }
    }
}
