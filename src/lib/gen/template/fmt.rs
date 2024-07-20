use crate::{gen::gen::Gen, lib::desc::Desc};
use handlebars::{
    Context as HbContext, Handlebars, Helper, HelperDef, HelperResult, JsonRender, JsonValue,
    Output, RenderContext, RenderError, ScopedJson,
};
use serde_json::Value;

#[derive(Clone)]
pub(crate) struct FmtClass {
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
                    &class.value().render(),
                    &origin.and_then(|param| param.value().as_str().map(str::to_string)),
                )
                .as_str(),
        )?;
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct FmtEnum {
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

        out.write(self.gen.lang().fmt_enum(&param.value().render()).as_str())
            .unwrap();
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct FmtName {
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

        out.write(self.gen.lang().fmt_name(&param.value().render()).as_str())
            .unwrap();
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct FmtOpt {
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
            .write(self.gen.lang().fmt_opt(&param.value().render()).as_str())
            .unwrap())
    }
}

#[derive(Clone)]
pub(crate) struct FmtSrcIfPresent {
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
            Some(str) => Ok(serde_json::to_value(self.gen.lang().fmt_src(str))
                .unwrap()
                .into()),
            None => Ok(Value::Null.into()),
        }
    }
}

#[derive(Clone)]
pub(crate) struct FmtType {
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
            .and_then(|param| param.value().as_str().map(str::to_string));

        Ok(out
            .write(
                match desc {
                    Desc::Def(def) => {
                        let formatted_name = name.map(|n| n.to_string());
                        self.gen.lang().fmt_type(&def, &formatted_name.as_deref())
                    }
                    Desc::Ref(r#ref) => self.gen.lang().fmt_ref(&r#ref),
                    Desc::TypeParam { param } => param.clone(),
                }
                .as_str(),
            )
            .unwrap())
    }
}

#[derive(Clone)]
pub(crate) struct FmtValue {
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
        let json_value: &JsonValue = h.param(0).unwrap().value();
        Ok(out
            .write(self.gen.lang().fmt_value(json_value).as_str())
            .unwrap())
    }
}
