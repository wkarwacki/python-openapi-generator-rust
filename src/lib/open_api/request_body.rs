use crate::lib::{
    context::Context,
    desc::Desc,
    open_api::{
        content::Content, context::Context as OpenApiContext, open_api::OpenApi, ref_or::RefOr,
    },
    req::Req,
    util,
    validation::ensure,
};
use mime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct RequestBody {
    pub content: Content,
    #[serde(default = "util::r#true", skip_serializing_if = "util::val")]
    pub required: bool,
}

impl RequestBody {
    pub(crate) fn of(req: &Req, context: &Context) -> RequestBody {
        RequestBody {
            content: Content::of_req(req, context),
            required: true,
        }
    }
    pub(crate) fn req(&self, context: &OpenApiContext) -> Req {
        let entries = self.clone().content.val;
        ensure(|| entries.len() == 1);
        let entry = entries.iter().next().unwrap();
        Req {
            form: {
                let mime = entry.0.clone().val;
                if mime == mime::APPLICATION_JSON {
                    None
                } else {
                    Some(mime.to_string())
                }
            },
            desc: match entry.1.clone().schema {
                RefOr::Ref { r#ref } => Desc::Ref(OpenApi::trust_ref(r#ref)),
                RefOr::Item(schema) => schema.clone().desc("req".to_string(), context),
            },
        }
    }
}
