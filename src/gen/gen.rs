use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fs;


use std::path::PathBuf;

use convert_case::{Case, Casing};
use dyn_clone::DynClone;
use handlebars::{Context as HbContext, Handlebars, handlebars_helper, Helper, HelperDef, HelperResult, JsonRender, JsonValue, Output, RenderContext, RenderError, ScopedJson};
use serde_json::{Error, json, Value};

use crate::context::Context;
use crate::def::{Def, Int, Obj, Str};
use crate::desc::Desc;
use crate::ext::Ext;
use crate::gen::lang::{DTO_NAME_TEMPLATE_NAME, Lang};
use crate::pkg::Pkg;
use crate::r#ref::Ref;
use crate::util::read;

pub trait Gen: DynClone + Send + Sync {
    fn lang(&self) -> Box<dyn Lang>;
    fn dtos(&self, handlebars: Handlebars, pkg: &Pkg, context: Context, templates: HashMap<String, String>) -> HashMap<PathBuf, String>;
    fn ops(&self, handlebars: Handlebars, pkg: &Pkg, context: Context, templates: HashMap<String, String>) -> HashMap<PathBuf, String>;
    fn src_dir(&self) -> PathBuf;
}

dyn_clone::clone_trait_object!(Gen);

#[derive(Clone)]
struct FmtClass {
    gen: Box<dyn Gen>,
}

impl HelperDef for FmtClass {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &HbContext, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        let class = h.param(0).unwrap();
        let origin = h.param(1);

        out.write(self.gen.lang().fmt_class(class.value().render(), origin.and_then(|param| param.value().as_str()).map(|str| str.to_string())).as_str())?;
        Ok(())
    }
}

#[derive(Clone)]
struct FmtEnum {
    gen: Box<dyn Gen>,
}

impl HelperDef for FmtEnum {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &HbContext, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        let param = h.param(0).unwrap();

        out.write(self.gen.lang().fmt_enum(param.value().render()).as_str())?;
        Ok(())
    }
}

#[derive(Clone)]
struct FmtName {
    gen: Box<dyn Gen>,
}

impl HelperDef for FmtName {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &HbContext, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        let param = h.param(0).unwrap();

        out.write(self.gen.lang().fmt_name(param.value().render()).as_str())?;
        Ok(())
    }
}

#[derive(Clone)]
struct FmtOpt {
    gen: Box<dyn Gen>,
}

impl HelperDef for FmtOpt {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &HbContext, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        let param = h.param(0).unwrap();
        Ok(out.write(self.gen.lang().fmt_opt(param.value().render()).as_str()).unwrap())
    }
}

#[derive(Clone)]
struct FmtSrcIfPresent {
    gen: Box<dyn Gen>,
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
            Some(str) => Ok(ScopedJson::from(serde_json::to_value(self.gen.lang().fmt_src(str.to_string())).unwrap())),
            None => Ok(ScopedJson::from(Value::Null))
        }
    }
}

#[derive(Clone)]
struct FmtType {
    gen: Box<dyn Gen>,
}

impl HelperDef for FmtType {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &HbContext, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        let desc: Desc = serde_json::from_value(h.param(0).unwrap().value().clone()).unwrap();
        let name: Option<String> = h.param(1).and_then(|param| param.value().clone().as_str().map(|str| str.to_string()));

        Ok(out.write(match desc {
            Desc::Def(def) => self.gen.lang().fmt_type(def, name),
            Desc::Ref(r#ref) => self.gen.lang().fmt_ref(r#ref),
            Desc::Param { param } => param
        }.as_str())?)
    }
}

#[derive(Clone)]
struct FmtValue {
    gen: Box<dyn Gen>,
}

impl HelperDef for FmtValue {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &HbContext, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        let json_value: JsonValue = h.param(0).unwrap().value().clone();
        Ok(out.write(self.gen.lang().fmt_value(json_value).as_str())?)
    }
}

#[derive(Clone)]
struct IsAlias {
    gen: Box<dyn Gen>,
}

impl HelperDef for IsAlias {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        Ok(ScopedJson::from(Value::Bool(if let Ok(Ref{path: _, src: _}) = serde_json::from_value(h.param(0).unwrap().value().clone()) {
            true
        } else {
            let def: Result<Def, _> = serde_json::from_value(h.param(0).unwrap().value().clone());
            match def {
                Ok(def) => match def {
                    Def::Enum(_) | Def::Obj(_) => false,
                    _ => true
                }
                Err(_) => false
            }
        })))
    }
}

#[derive(Clone)]
struct IsPrimitive {
    gen: Box<dyn Gen>,
}

#[derive(Clone)]
struct Parents {
    context: Context,
}

impl HelperDef for Parents {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let obj: Obj = serde_json::from_value(h.param(0).unwrap().value().clone()).unwrap();
        let mut refs = obj.mix;
        obj.ext.iter().for_each(|ext| {
            refs.insert(0, ext.clone().r#ref);
        });
        Ok(ScopedJson::from(serde_json::to_value(refs).unwrap()))
    }
}

#[derive(Clone)]
struct Resolve {
    context: Context,
}

impl HelperDef for Resolve {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        if let Ok(Desc::Ref(r#ref)) = serde_json::from_value(h.param(0).unwrap().value().clone()) {
            let mut value: Value = serde_json::to_value(self.context.resolve(r#ref.clone())).unwrap();
            value.as_object_mut().unwrap().insert("origin".to_string(), serde_json::to_value(r#ref).unwrap());
            Ok(ScopedJson::from(value))
        } else {
            Ok(ScopedJson::Missing)
        }
    }
}

#[derive(Clone)]
struct TypeArgs {
    context: Context,
}

impl HelperDef for TypeArgs {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let ext: Result<Ext, _> = serde_json::from_value(h.param(0).unwrap().value().clone());
        ext.map(|e| {
            let mut vec: Vec<_> = e.args.into_iter().collect();
            vec.sort_by(|(name0, _), (name1, _)| name0.cmp(name1));
            Ok(ScopedJson::from(serde_json::to_value(vec.iter().map(|(_, desc)| desc).collect::<Vec<_>>()).unwrap()))
        }).unwrap_or(Ok(ScopedJson::from(Value::Array(Vec::new()))))
    }
}

#[derive(Clone)]
struct TypeParams {
    context: Context,
}

impl HelperDef for TypeParams {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let obj: Result<Obj, _> = serde_json::from_value(h.param(0).unwrap().value().clone());
        obj.map(|o| {
            let mut vec: Vec<_> = o.vars.iter().flat_map(|(_, var)| var.desc.param().map(str::to_string)).collect::<HashSet<_>>().into_iter().collect();
            vec.sort();
            Ok(ScopedJson::from(serde_json::to_value(vec).unwrap()))
        }).unwrap_or(Ok(ScopedJson::from(Value::Array(Vec::new()))))
    }
}

#[derive(Clone)]
struct ValueDef {
    context: Context,
}

impl HelperDef for ValueDef {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let val = h.param(0).unwrap().value().render();
        Ok(ScopedJson::from(Value::from(serde_json::to_value(match val.parse::<i64>() {
            Ok(_) => Def::Int(Int { null: false }),
            _ => Def::Str(Str { null: false })
        }).unwrap())))
    }
}

#[derive(Clone)]
struct Json;

impl HelperDef for Json {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &HbContext, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        let param = h.param(0).unwrap();

        out.write(param.value().to_string().as_str())?;
        Ok(())
    }
}

#[derive(Clone)]
struct Add;

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
                Ok(ScopedJson::from(other.as_array().map(|vec| vec.clone()).map(Value::from).or(other.as_str().map(Value::from)).unwrap()))
            }
            _ => {
                let result = param.as_array()
                    .map(|v| {
                        let mut vec = v.clone();
                        let other = h.param(1).unwrap().value().as_array().unwrap();
                        vec.append(&mut other.clone());
                        serde_json::to_value(vec)
                    }).or(param.as_str().map(|str| {
                    let mut string = str.to_string();
                    let other = h.param(1).unwrap().value().as_str();
                    other.iter().for_each(|o| string.push_str(o));
                    serde_json::to_value(string)
                })).unwrap();
                Ok(ScopedJson::from(result.unwrap()))
            }
        }
    }
}

pub fn go(pkg: &Pkg, gen: Box<dyn Gen>, templates_path: Option<PathBuf>, context: Context) -> HashMap<PathBuf, String> {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("json", Box::new(Json {}.clone()));
    handlebars.register_helper("add", Box::new(Add {}.clone()));

    handlebars.register_helper("fmtClass", Box::new(FmtClass { gen: gen.clone() }.clone()));
    handlebars.register_helper("fmtEnum", Box::new(FmtEnum { gen: gen.clone() }.clone()));
    handlebars.register_helper("fmtName", Box::new(FmtName { gen: gen.clone() }.clone()));
    handlebars.register_helper("fmtOpt", Box::new(FmtOpt { gen: gen.clone() }.clone()));
    handlebars.register_helper("fmtSrcIfPresent", Box::new(FmtSrcIfPresent { gen: gen.clone() }.clone()));
    handlebars.register_helper("fmtType", Box::new(FmtType { gen: gen.clone() }.clone()));
    handlebars.register_helper("fmtValue", Box::new(FmtValue { gen: gen.clone() }.clone()));

    handlebars_helper!(filter_nonconst: |defs: HashMap<String, Def>| defs.iter().filter(|(name, def)| match def{
        Def::Const(_) => false,
        _ => true
    }).map(|(name, def)| serde_json::to_value((name, def)).unwrap()).collect::<Vec<_>>()); // TIDY: make object (i.e. hashmap) out of it instead of Vec<Pair>, otherwise before and after filtering are different types | TIDY: make it handle nulls, otherwise double if is needed everywhere
    handlebars.register_helper("filterNonconst", Box::new(filter_nonconst));
    handlebars.register_helper("isAlias", Box::new(IsAlias { gen: gen.clone() }.clone()));
    handlebars_helper!(has_key: |json_value: JsonValue, key: String| match json_value {
        Value::Object(map) => map.contains_key(key.as_str()),
        _ => false
    });
    handlebars.register_helper("hasKey", Box::new(has_key));

    handlebars.register_helper("parents", Box::new(Parents { context: context.clone() }.clone()));
    handlebars.register_helper("resolve", Box::new(Resolve { context: context.clone() }.clone()));
    handlebars.register_helper("typeArgs", Box::new(TypeArgs { context: context.clone() }.clone()));
    handlebars_helper!(to_flat_case: |string: String| string.to_case(Case::Flat));
    handlebars.register_helper("toFlatCase", Box::new(to_flat_case));
    handlebars.register_helper("typeParams", Box::new(TypeParams { context: context.clone() }.clone()));
    handlebars.register_helper("valueDef", Box::new(ValueDef { context: context.clone() }.clone()));

    handlebars_misc_helpers::setup_handlebars(&mut handlebars);
    handlebars.set_strict_mode(false);

    let mut merged_templates: HashMap<_, _> = templates(default_templates_path(gen.clone()));
    let templates = templates_path.map(templates).unwrap_or(HashMap::new());
    merged_templates.extend(templates);


    merged_templates.iter().for_each(|(name, template)| {
        handlebars.register_template_string(name, template.clone()).unwrap();
    });

    handlebars.register_template(DTO_NAME_TEMPLATE_NAME, gen.lang().handlebars().get_template(DTO_NAME_TEMPLATE_NAME).unwrap().clone());

    let dtos = gen.dtos(handlebars.clone(), pkg, context.clone(), merged_templates.clone());

    if pkg.ops.is_empty() {
        dtos
    } else {
        let ops = gen.ops(handlebars, pkg, context, merged_templates);
        let mut result = dtos;
        ops.iter().for_each(|op| {
            result.insert(op.0.clone(), op.1.clone());
        });
        result
    }
}

pub fn dto_name(string: String, lang: Box<dyn Lang>) -> String {
    lang.handlebars().render(DTO_NAME_TEMPLATE_NAME, &json!({"val": string})).unwrap()
}

fn templates(path: PathBuf) -> HashMap<String, String> {
    fs::read_dir(path).unwrap().map(|entry| {
        let path = entry.unwrap().path().clone();
        (path.file_stem().unwrap().to_string_lossy().to_string(), template(path))
    }).collect()
}

fn template(path: PathBuf) -> String {
    read(path)
}

fn default_templates_path(gen: Box<dyn Gen>) -> PathBuf {
    PathBuf::from("src/gen/".to_string() + gen.src_dir().to_string_lossy().to_string().as_str() + "/templates")
}
