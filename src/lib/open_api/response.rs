use crate::lib::{
    carrier::Carrier,
    context::Context,
    desc::Desc,
    meta::Meta,
    open_api::{
        content::Content, context::Context as OpenApiContext, header::Header, open_api::OpenApi,
        ref_or::RefOr, schema::Schema, status_code::StatusCode,
    },
    res::Res,
    validation::ensure,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Response {
    pub description: String,
    pub content: Option<Content>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, RefOr<Header>>,
}

impl Response {
    pub(crate) fn of(res: &Res, status_code: StatusCode, context: &Context) -> Response {
        Response {
            description: status_code.val.to_string(),
            content: Some(Content::of_res(res, context)),
            headers: res
                .meta
                .value
                .iter()
                .map(|(name, meta)| {
                    (
                        name.clone(),
                        match meta {
                            Desc::Def(def) => RefOr::Item(Header {
                                schema: Schema::of_def(def.clone(), name, None, context),
                            }),
                            Desc::Ref(r#ref) => RefOr::Ref {
                                r#ref: Schema::openapi_path(r#ref),
                            },
                            Desc::TypeParam { .. } => unimplemented!(),
                        },
                    )
                })
                .collect(),
        }
    }
    pub(crate) fn res(&self, context: &OpenApiContext) -> Option<Res> {
        self.clone().content.map(|content| {
            let entries = content.val;
            ensure(|| entries.len() == 1);
            let entry = entries.iter().next().unwrap();
            Res {
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
                    RefOr::Item(schema) => schema.clone().desc("res".to_string(), context),
                },
                carrier: if entry
                    .1
                    .clone()
                    .schema
                    .map_item(|schema| {
                        schema
                            .clone()
                            .format
                            .map(|format| format == "binary")
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
                {
                    Carrier::Stream
                } else {
                    Carrier::Batch
                },
                meta: Meta {
                    value: self
                        .headers
                        .iter()
                        .map(|(name, ref_or_header)| {
                            (
                                name.clone(),
                                match ref_or_header.clone() {
                                    RefOr::Ref { r#ref } => Desc::Ref(OpenApi::trust_ref(r#ref)),
                                    RefOr::Item(header) => {
                                        Desc::Def(header.schema.def(name.clone(), context))
                                    }
                                },
                            )
                        })
                        .collect(),
                },
            }
        })
    }
}
