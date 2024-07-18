use convert_case::{Case, Casing};
use handlebars::{
    Context as HbContext, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
    RenderError, ScopedJson,
};
use serde_json::Value;

#[derive(Clone)]
pub(crate) struct Json;

impl HelperDef for Json {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &HbContext,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();

        out.write(param.value().to_string().as_str()).unwrap();
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct Add;

impl HelperDef for Add {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let param = h.param(0).unwrap().value();
        match param {
            Value::Null => {
                let other = h.param(1).unwrap().value();
                Ok(other
                    .as_array()
                    .map(Vec::clone)
                    .map(Value::from)
                    .or(other.as_str().map(Value::from))
                    .unwrap()
                    .into())
            }
            _ => {
                let result = param
                    .as_array()
                    .map(|v| {
                        let mut vec = v.clone();
                        let other = h.param(1).unwrap().value().as_array().unwrap();
                        vec.append(&mut other.clone());
                        serde_json::to_value(vec)
                    })
                    .or(param.as_str().map(|str| {
                        let string = str.to_string();
                        let other = h.param(1).unwrap().value().as_str();
                        let joined = string + other.unwrap_or("");
                        serde_json::to_value(joined)
                    }))
                    .unwrap();
                Ok(result.unwrap().into())
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct ToFlatCase {}

impl HelperDef for ToFlatCase {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let string: String = serde_json::from_value(h.param(0).unwrap().value().clone()).unwrap();
        Ok(Value::from(string.to_case(Case::Flat)).into())
    }
}
