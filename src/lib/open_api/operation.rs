use crate::lib::{
    context::Context,
    op::Op,
    op_param::OpParam,
    open_api::{
        context::Context as OpenApiContext, parameter::Parameter, ref_or::RefOr,
        request_body::RequestBody, response::Response, status_code::StatusCode,
    },
};
use http::Method;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use typetag::serde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Operation {
    pub operation_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RefOr<Parameter>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RefOr<RequestBody>>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub responses: IndexMap<StatusCode, RefOr<Response>>,
}

impl Operation {
    pub(crate) fn of(
        ops: &Vec<Op>,
        method: Method,
        common_op_params: &Vec<OpParam>,
        context: &Context,
    ) -> Option<Operation> {
        ops.iter()
            .find(|op| op.r#type.iter().any(|r#type| r#type == method.as_str()))
            .map(|op| {
                let mut op_params: Vec<OpParam> = op.params.clone();
                op_params.retain(|op_param| !common_op_params.contains(op_param));
                Operation {
                    tags: vec![],
                    operation_id: op.name.clone(),
                    parameters: op_params
                        .iter()
                        .map(|param| RefOr::Item(Parameter::of(param, context)))
                        .collect(),
                    request_body: op
                        .req
                        .as_ref()
                        .map(|req| RefOr::Item(RequestBody::of(req, context))),
                    responses: op
                        .res
                        .as_ref()
                        .iter()
                        .map(|res| {
                            let status_code = StatusCode::of(method.clone());
                            (
                                status_code.clone(),
                                RefOr::Item(Response::of(res, status_code, context)),
                            )
                        })
                        .collect(),
                }
            })
    }

    pub(crate) fn op(
        &self,
        method: Method,
        path_op_params: Vec<OpParam>,
        context: &OpenApiContext,
    ) -> Op {
        Op {
            name: self.operation_id.clone(),
            r#type: Some(method.to_string()),
            req: self.request_body.as_ref().map(|request_body| {
                request_body
                    .map_item(|request_body| request_body.req(context))
                    .unwrap(context)
            }),
            res: self
                .responses
                .first()
                .as_ref()
                .and_then(|(_, response)| response.map_item(|r| r.res(context)).unwrap(context)),
            params: {
                let mut params: Vec<_> = self
                    .parameters
                    .iter()
                    .map(|parameter| {
                        parameter
                            .map_item(|item| item.op_param(context))
                            .unwrap(context)
                    })
                    .collect();
                let mut op_params = path_op_params;
                op_params.append(&mut params);
                op_params
            },
        }
    }
}
