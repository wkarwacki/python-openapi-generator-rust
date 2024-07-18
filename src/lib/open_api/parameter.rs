use crate::lib::{
    context::Context,
    op_param::OpParam,
    open_api::{
        context::Context as OpenApiContext,
        parameter::Parameter::{Cookie, Header, Path, Query},
        schema::Schema,
    },
    util,
};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::ops::Not;

trait ParameterVal {
    fn name(&self) -> String;
    fn required(&self) -> bool;
    fn schema(&self) -> Schema;
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct ParameterValDefault {
    pub name: String,
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    pub required: bool,
    pub schema: Schema,
}

impl ParameterVal for ParameterValDefault {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn required(&self) -> bool {
        self.required.clone()
    }
    fn schema(&self) -> Schema {
        self.schema.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct ParameterValPath {
    pub name: String,
    #[serde(default = "util::r#true", skip_serializing_if = "util::val")]
    pub required: bool,
    pub schema: Schema,
}

impl ParameterVal for ParameterValPath {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn required(&self) -> bool {
        self.required.clone()
    }
    fn schema(&self) -> Schema {
        self.schema.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "in", rename_all = "camelCase")]
pub(crate) enum Parameter {
    Cookie(ParameterValDefault),
    Header(ParameterValDefault),
    Path(ParameterValPath),
    Query(ParameterValDefault),
}

impl Parameter {
    pub(crate) fn of(op_param: &OpParam, context: &Context) -> Parameter {
        match op_param.clone().loc.unwrap_or("query".to_string()).as_str() {
            "cookie" => Cookie(ParameterValDefault {
                name: op_param.name.clone(),
                required: op_param.default.is_none(),
                schema: Schema::of_desc(
                    &op_param.desc,
                    "CookieParam",
                    op_param.clone().default,
                    context,
                ),
            }),
            "header" => Header(ParameterValDefault {
                name: op_param.name.clone(),
                required: op_param.default.is_none(),
                schema: Schema::of_desc(
                    &op_param.desc,
                    "HeaderParam",
                    op_param.clone().default,
                    context,
                ),
            }),
            "path" => Path(ParameterValPath {
                name: op_param.name.clone(),
                required: op_param.default.is_none(),
                schema: Schema::of_desc(
                    &op_param.desc,
                    "PathParam",
                    if op_param.clone().default == Some(Value::Null) {
                        None
                    } else {
                        op_param.clone().default
                    },
                    context,
                ),
            }),
            "query" => Query(ParameterValDefault {
                name: op_param.name.clone(),
                required: op_param.default.is_none(),
                schema: Schema::of_desc(
                    &op_param.desc,
                    "QueryParam",
                    op_param.clone().default,
                    context,
                ),
            }),
            _ => unimplemented!(),
        }
    }

    // TIDY: extract processing for all below methods
    pub(crate) fn op_param(&self, context: &OpenApiContext) -> OpParam {
        match self {
            Cookie(val) => OpParam {
                name: val.name.clone(),
                loc: Some("cookie".to_string()),
                desc: val.clone().schema.desc("param".to_string(), context),
                default: if val.required {
                    None
                } else {
                    Some(val.clone().schema.default.unwrap_or(Value::Null))
                },
            },
            Header(val) => OpParam {
                name: val.name.clone(),
                loc: Some("header".to_string()),
                desc: val.clone().schema.desc("param".to_string(), context),
                default: if val.required {
                    None
                } else {
                    Some(val.clone().schema.default.unwrap_or(Value::Null))
                },
            },
            Path(val) => OpParam {
                name: val.name.clone(),
                loc: Some("path".to_string()),
                desc: val.clone().schema.desc("param".to_string(), context),
                default: if val.required {
                    None
                } else {
                    Some(val.clone().schema.default.unwrap_or(Value::Null))
                },
            },
            Query(val) => OpParam {
                name: val.name.clone(),
                loc: Some("query".to_string()),
                desc: val.clone().schema.desc("param".to_string(), context),
                default: if val.required {
                    None
                } else {
                    Some(val.clone().schema.default.unwrap_or(Value::Null))
                },
            },
        }
    }
}
