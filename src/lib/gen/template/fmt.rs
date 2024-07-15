use crate::{gen::gen::Gen, lib::desc::Desc};
use handlebars::{
    Context as HbContext, Handlebars, Helper, HelperDef, HelperResult, JsonRender, JsonValue,
    Output, RenderContext, RenderError, ScopedJson,
};
use serde_json::Value;

#[derive(Clone)]
pub struct FmtClass {
    pub gen: Box<dyn Gen>,
}

impl HelperDef for FmtClass {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &HbContext,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let class = h.param(0).unwrap();
        let origin = h.param(1);

        out.write(
            self.gen
                .lang()
                .fmt_class(
                    class.value().render(),
                    origin
                        .and_then(|param| param.value().as_str())
                        .map(|str| str.to_string()),
                )
                .as_str(),
        )?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct FmtEnum {
    pub gen: Box<dyn Gen>,
}

impl HelperDef for FmtEnum {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &HbContext,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();

        out.write(self.gen.lang().fmt_enum(param.value().render()).as_str())?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct FmtName {
    pub gen: Box<dyn Gen>,
}

impl HelperDef for FmtName {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &HbContext,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();

        out.write(self.gen.lang().fmt_name(param.value().render()).as_str())?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct FmtOpt {
    pub gen: Box<dyn Gen>,
}

impl HelperDef for FmtOpt {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &HbContext,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();
        Ok(out
            .write(self.gen.lang().fmt_opt(param.value().render()).as_str())
            .unwrap())
    }
}

#[derive(Clone)]
pub struct FmtSrcIfPresent {
    pub gen: Box<dyn Gen>,
}

impl HelperDef for FmtSrcIfPresent {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let param = h.param(0).unwrap();
        match param.value().as_str() {
            Some(str) => Ok(
                serde_json::to_value(self.gen.lang().fmt_src(str.to_string()))
                    .unwrap()
                    .into(),
            ),
            None => Ok(Value::Null.into()),
        }
    }
}

#[derive(Clone)]
pub struct FmtType {
    pub gen: Box<dyn Gen>,
}

impl HelperDef for FmtType {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &HbContext,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let desc: Desc = serde_json::from_value(h.param(0).unwrap().value().clone()).unwrap();
        let name: Option<String> = h
            .param(1)
            .and_then(|param| param.value().clone().as_str().map(|str| str.to_string()));

        Ok(out.write(
            match desc {
                Desc::Def(def) => self.gen.lang().fmt_type(def, name),
                Desc::Ref(r#ref) => self.gen.lang().fmt_ref(r#ref),
                Desc::TypeParam { param } => param,
            }
            .as_str(),
        )?)
    }
}

#[derive(Clone)]
pub struct FmtValue {
    pub gen: Box<dyn Gen>,
}

impl HelperDef for FmtValue {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &HbContext,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let json_value: JsonValue = h.param(0).unwrap().value().clone();
        Ok(out.write(self.gen.lang().fmt_value(json_value).as_str())?)
    }
}