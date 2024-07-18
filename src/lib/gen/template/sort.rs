use crate::lib::{op_param::OpParam, var::Var};
use handlebars::{Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct SortOptionalsLast;

impl HelperDef for SortOptionalsLast {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let vars: Result<HashMap<String, Box<Var>>, _> =
            serde_json::from_value(h.param(0).unwrap().value().clone());
        let value = match vars {
            Ok(vars) => {
                let mut vec: Vec<_> = vars.iter().collect();
                vec.sort_by(|(name0, var0), (name1, var1)| {
                    if var0.opt && !var1.opt {
                        std::cmp::Ordering::Greater
                    } else if !var0.opt && var1.opt {
                        std::cmp::Ordering::Less
                    } else {
                        name0.cmp(name1)
                    }
                });
                Value::Object(
                    vec.iter()
                        .map(|(&ref name, var)| (name.clone(), serde_json::to_value(var).unwrap()))
                        .collect(),
                )
            }
            _ => {
                let mut op_params: Vec<OpParam> =
                    serde_json::from_value(h.param(0).unwrap().value().clone())
                        .unwrap_or(Vec::new());
                op_params.sort_by(|op_param0, op_param1| {
                    if op_param0.default.is_some() && op_param1.default.is_none() {
                        std::cmp::Ordering::Greater
                    } else if op_param0.default.is_none() && op_param1.default.is_some() {
                        std::cmp::Ordering::Less
                    } else {
                        op_param0.name.cmp(&op_param1.name.clone())
                    }
                });
                Value::Array(
                    op_params
                        .iter()
                        .map(|op_param| serde_json::to_value(op_param).unwrap())
                        .collect(),
                )
            }
        };

        Ok(value.into())
    }
}
