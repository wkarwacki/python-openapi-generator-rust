use crate::lib::{def::Def, r#ref::Ref};
use handlebars::{Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use serde_json::Value;

#[derive(Clone)]
pub(crate) struct HasKey;

impl HelperDef for HasKey {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let value = h.param(0).unwrap().value().clone();
        let key: String = serde_json::from_value(h.param(1).unwrap().value().clone()).unwrap();
        Ok(Value::Bool(match value {
            Value::Object(map) => map.contains_key(key.as_str()),
            _ => false,
        })
        .into())
    }
}

#[derive(Clone)]
pub(crate) struct IsAlias;

impl HelperDef for IsAlias {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        Ok(Value::Bool(
            if let Ok(Ref { path: _, src: _ }) =
                serde_json::from_value(h.param(0).unwrap().value().clone())
            {
                true
            } else {
                let def: Result<Def, _> =
                    serde_json::from_value(h.param(0).unwrap().value().clone());
                match def {
                    Ok(def) => match def {
                        Def::Enum(_) | Def::Obj(_) => false,
                        _ => true,
                    },
                    Err(_) => false,
                }
            },
        )
        .into())
    }
}

#[derive(Clone)]
pub(crate) struct IsPrimitive;
