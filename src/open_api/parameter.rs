use serde::{Deserialize, Serialize};
use std::ops::Not;

use crate::context::Context;
use crate::op_param::OpParam;
use crate::open_api::parameter::Parameter::{Cookie, Header, Path, Query};

use crate::open_api::schema::Schema;
use crate::open_api::context::Context as OpenApiContext;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ParameterVal {
    pub name: String,
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    pub required: bool,
    pub schema: Schema,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "in", rename_all = "camelCase")]
pub enum Parameter {
    Cookie {
        #[serde(flatten)]
        val: ParameterVal,
    },
    Header {
        #[serde(flatten)]
        val: ParameterVal,
    },
    Path {
        #[serde(flatten)]
        val: ParameterVal,
    },
    Query {
        #[serde(flatten)]
        val: ParameterVal,
    },
}

impl Parameter {
    pub fn of(op_param: &OpParam, context: &Context) -> Parameter {
        match op_param.clone().loc.unwrap_or("query".to_string()).as_str() {
            "cookie" => Cookie{val: ParameterVal {
                name: op_param.name.clone(),
                required: op_param.default.is_none(),
                schema: Schema::of_desc(&op_param.desc, "CookieParam".to_string(), op_param.clone().default, context),
            }},
            "header" => Header{val: ParameterVal {
                name: op_param.name.clone(),
                required: op_param.default.is_none(),
                schema: Schema::of_desc(&op_param.desc, "HeaderParam".to_string(), op_param.clone().default, context),
            }},
            "path" => Path{val: ParameterVal {
                name: op_param.name.clone(),
                required: op_param.default.is_none(),
                schema: Schema::of_desc(&op_param.desc, "PathParam".to_string(), op_param.clone().default, context),
            }},
            "query" => Query{val: ParameterVal {
                name: op_param.name.clone(),
                required: op_param.default.is_none(),
                schema: Schema::of_desc(&op_param.desc, "QueryParam".to_string(), op_param.clone().default, context),
            }},
            _ => unimplemented!()
        }
    }

    pub fn op_param(&self, context: &OpenApiContext) -> OpParam {
        match self {
            Cookie{val} => OpParam {
                name: val.name.clone(),
                loc: Some("cookie".to_string()),
                desc: val.clone().schema.desc("param".to_string(), context),
                default: val.clone().schema.default
            },
            Header{val} => OpParam {
                name: val.name.clone(),
                loc: Some("header".to_string()),
                desc: val.clone().schema.desc("param".to_string(), context),
                default: val.clone().schema.default
            },
            Path{val} => OpParam {
                name: val.name.clone(),
                loc: Some("path".to_string()),
                desc: val.clone().schema.desc("param".to_string(), context),
                default: val.clone().schema.default
            },
            Query{val} => OpParam {
                name: val.name.clone(),
                loc: Some("query".to_string()),
                desc: val.clone().schema.desc("param".to_string(), context),
                default: val.clone().schema.default
            },
        }
    }
}