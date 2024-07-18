use crate::lib::{def::Def, op_param::OpParam};
use handlebars::{Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct FilterNonconst;

impl HelperDef for FilterNonconst {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let defs: HashMap<String, Def> =
            serde_json::from_value(h.param(0).unwrap().value().clone())
                .unwrap_or(Default::default());
        Ok(Value::Array(
            defs.iter()
                .filter(|(_name, def)| match def {
                    Def::Const(_) => false,
                    _ => true,
                })
                .map(|(name, def)| serde_json::to_value((name, def)).unwrap())
                .collect::<Vec<_>>(),
        )
        .into())
    }
}

#[derive(Clone)]
pub(crate) struct FilterOpParamsByLoc;

impl HelperDef for FilterOpParamsByLoc {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let op_params: Vec<OpParam> = serde_json::from_value(h.param(0).unwrap().value().clone())
            .unwrap_or(Default::default());
        let loc: String = serde_json::from_value(h.param(1).unwrap().value().clone()).unwrap();
        Ok(Value::Array(
            op_params
                .iter()
                .filter(|&op_param| op_param.clone().loc.map(|l| l == loc).unwrap_or(false))
                .map(|op_param| serde_json::to_value(op_param).unwrap())
                .collect::<Vec<_>>(),
        )
        .into())
    }
}
