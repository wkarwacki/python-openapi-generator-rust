use crate::lib::{
    context::Context,
    op::Op,
    op_param::OpParam,
    open_api::{
        context::Context as OpenApiContext, operation::Operation, parameter::Parameter,
        ref_or::RefOr,
    },
};
use http::method::Method;
use serde::{Deserialize, Serialize};
use std::convert::identity;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Path {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RefOr<Parameter>>,
}

impl Path {
    pub(crate) fn operations(&self) -> Vec<Operation> {
        vec![
            self.get.clone(),
            self.put.clone(),
            self.post.clone(),
            self.delete.clone(),
            self.patch.clone(),
        ]
        .into_iter()
        .filter_map(identity)
        .collect()
    }
    pub(crate) fn of(ops: &Vec<Op>, context: &Context) -> Path {
        let common_op_params: Vec<_> = ops
            .iter()
            .fold(None as Option<Vec<OpParam>>, |vec, op| match vec {
                Some(vec) => Some(
                    vec.iter()
                        .cloned()
                        .filter(|param: &OpParam| op.params.iter().any(|p| p == param))
                        .collect(),
                ),
                None => Some(op.params.clone()),
            })
            .iter()
            .flatten()
            .cloned()
            .collect();
        Path {
            summary: None,
            description: None,
            get: Operation::of(ops, Method::GET, &common_op_params, context),
            put: Operation::of(ops, Method::PUT, &common_op_params, context),
            post: Operation::of(ops, Method::POST, &common_op_params, context),
            delete: Operation::of(ops, Method::DELETE, &common_op_params, context),
            patch: Operation::of(ops, Method::PATCH, &common_op_params, context),
            parameters: common_op_params
                .iter()
                .map(|op_param| RefOr::Item(Parameter::of(op_param, context)))
                .collect(),
        }
    }

    pub(crate) fn ops(&self, context: &OpenApiContext) -> Vec<Op> {
        let mut ops = vec![];
        let path_op_params: Vec<_> = self
            .parameters
            .iter()
            .map(|parameter| {
                parameter
                    .map_item(|item| item.op_param(context))
                    .unwrap(context)
            })
            .collect();
        if let Some(get) = &self.get {
            ops.push(get.op(Method::GET, path_op_params.clone(), context));
        }
        if let Some(put) = &self.put {
            ops.push(put.op(Method::PUT, path_op_params.clone(), context));
        }
        if let Some(post) = &self.post {
            ops.push(post.op(Method::POST, path_op_params.clone(), context));
        }
        if let Some(delete) = &self.delete {
            ops.push(delete.op(Method::DELETE, path_op_params.clone(), context));
        }
        if let Some(patch) = &self.patch {
            ops.push(patch.op(Method::PATCH, path_op_params.clone(), context));
        }
        ops
    }
}
