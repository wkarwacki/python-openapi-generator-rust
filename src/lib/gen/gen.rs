use crate::lib::{
    context::Context,
    def::{Def, Int, Obj, Str},
    desc::Desc,
    ext::Ext,
    gen::lang::{Lang, DTO_NAME_TEMPLATE_NAME},
    op_param::OpParam,
    pkg::Pkg,
    r#ref::Ref,
    util::read,
    var::Var,
};
use convert_case::{Case, Casing};
use dyn_clone::DynClone;
use handlebars::{
    handlebars_helper, Context as HbContext, Handlebars, Helper, HelperDef, HelperResult,
    JsonRender, JsonValue, Output, RenderContext, RenderError, ScopedJson,
};
use serde_json::{json, Value};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

pub trait Gen: DynClone + Send + Sync {
    fn lang(&self) -> Box<dyn Lang>;
    fn dtos(
        &self,
        handlebars: Handlebars,
        pkg: &Pkg,
        context: Context,
        templates: HashMap<String, String>,
    ) -> HashMap<PathBuf, String>;
    fn ops(
        &self,
        handlebars: Handlebars,
        pkg: &Pkg,
        context: Context,
        templates: HashMap<String, String>,
    ) -> HashMap<PathBuf, String>;
    fn src_dir(&self) -> PathBuf;
}

dyn_clone::clone_trait_object!(Gen);

#[derive(Clone)]
struct FmtClass {
    gen: Box<dyn Gen>,
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
struct FmtEnum {
    gen: Box<dyn Gen>,
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
struct FmtName {
    gen: Box<dyn Gen>,
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
struct FmtOpt {
    gen: Box<dyn Gen>,
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
struct FmtType {
    gen: Box<dyn Gen>,
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
struct FmtValue {
    gen: Box<dyn Gen>,
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

#[derive(Clone)]
struct FilterNonconst;

impl HelperDef for FilterNonconst {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let defs: HashMap<String, Def> =
            serde_json::from_value(h.param(0).unwrap().value().clone()).unwrap();
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
struct FilterOpParamsByLoc;

impl HelperDef for FilterOpParamsByLoc {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let op_params: Vec<OpParam> =
            serde_json::from_value(h.param(0).unwrap().value().clone()).unwrap();
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

#[derive(Clone)]
struct HasKey;

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
struct IsAlias;

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
struct IsPrimitive;

#[derive(Clone)]
struct Parents;

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
        Ok(serde_json::to_value(refs).unwrap().into())
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
            let mut value: Value =
                serde_json::to_value(self.context.resolve(r#ref.clone())).unwrap();
            value
                .as_object_mut()
                .unwrap()
                .insert("origin".to_string(), serde_json::to_value(r#ref).unwrap());
            Ok(value.into())
        } else {
            Ok(ScopedJson::Missing)
        }
    }
}

#[derive(Clone)]
struct SortOptionalsLast;

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

#[derive(Clone)]
struct TypeArgs;

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
            Ok(
                serde_json::to_value(vec.iter().map(|(_, desc)| desc).collect::<Vec<_>>())
                    .unwrap()
                    .into(),
            )
        })
        .unwrap_or(Ok(Value::Array(Vec::new()).into()))
    }
}

#[derive(Clone)]
struct TypeParams;

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
            let mut vec: Vec<_> = o
                .vars
                .iter()
                .flat_map(|(_, var)| var.desc.param().map(str::to_string))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            vec.sort();
            Ok(serde_json::to_value(vec).unwrap().into())
        })
        .unwrap_or(Ok(Value::Array(Vec::new()).into()))
    }
}

#[derive(Clone)]
struct ValueDef {}

impl HelperDef for ValueDef {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let val = h.param(0).unwrap().value().render();
        Ok(Value::from(
            serde_json::to_value(match val.parse::<i64>() {
                Ok(_) => Def::Int(Int { null: false }),
                _ => Def::Str(Str { null: false }),
            })
            .unwrap(),
        )
        .into())
    }
}

#[derive(Clone)]
struct Json;

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
                Ok(other
                    .as_array()
                    .map(|vec| vec.clone())
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
struct ToFlatCase {}

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

pub fn go(
    pkg: &Pkg,
    gen: Box<dyn Gen>,
    templates_path: Option<PathBuf>,
    context: Context,
) -> HashMap<PathBuf, String> {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("json", Box::new(Json {}.clone()));
    handlebars.register_helper("add", Box::new(Add {}.clone()));

    handlebars.register_helper("fmtClass", Box::new(FmtClass { gen: gen.clone() }.clone()));
    handlebars.register_helper("fmtEnum", Box::new(FmtEnum { gen: gen.clone() }.clone()));
    handlebars.register_helper("fmtName", Box::new(FmtName { gen: gen.clone() }.clone()));
    handlebars.register_helper("fmtOpt", Box::new(FmtOpt { gen: gen.clone() }.clone()));
    handlebars.register_helper(
        "fmtSrcIfPresent",
        Box::new(FmtSrcIfPresent { gen: gen.clone() }.clone()),
    );
    handlebars.register_helper("fmtType", Box::new(FmtType { gen: gen.clone() }.clone()));
    handlebars.register_helper("fmtValue", Box::new(FmtValue { gen: gen.clone() }.clone()));

    handlebars.register_helper("filterNonconst", Box::new(FilterNonconst {}.clone()));
    handlebars.register_helper(
        "filterOpParamsByLoc",
        Box::new(FilterOpParamsByLoc {}.clone()),
    );
    handlebars.register_helper("isAlias", Box::new(IsAlias {}.clone()));
    handlebars.register_helper("hasKey", Box::new(HasKey {}.clone()));

    handlebars.register_helper("parents", Box::new(Parents {}.clone()));
    handlebars.register_helper(
        "resolve",
        Box::new(
            Resolve {
                context: context.clone(),
            }
            .clone(),
        ),
    );
    handlebars.register_helper("sortOptionalsLast", Box::new(SortOptionalsLast {}.clone()));
    handlebars.register_helper("typeArgs", Box::new(TypeArgs {}.clone()));
    handlebars_helper!(to_flat_case: |string: String| string.to_case(Case::Flat));
    handlebars.register_helper("toFlatCase", Box::new(to_flat_case));
    handlebars.register_helper("typeParams", Box::new(TypeParams {}.clone()));
    handlebars.register_helper("valueDef", Box::new(ValueDef {}.clone()));

    handlebars_misc_helpers::setup_handlebars(&mut handlebars);
    handlebars.set_strict_mode(false);

    let mut merged_templates: HashMap<_, _> = templates(default_templates_path(gen.clone()));
    let templates = templates_path.map(templates).unwrap_or(HashMap::new());
    merged_templates.extend(templates);

    merged_templates.iter().for_each(|(name, template)| {
        handlebars
            .register_template_string(name, template.clone())
            .unwrap();
    });

    handlebars.register_template(
        DTO_NAME_TEMPLATE_NAME,
        gen.lang()
            .handlebars()
            .get_template(DTO_NAME_TEMPLATE_NAME)
            .unwrap()
            .clone(),
    );

    let dtos = gen.dtos(
        handlebars.clone(),
        pkg,
        context.clone(),
        merged_templates.clone(),
    );

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
    lang.handlebars()
        .render(DTO_NAME_TEMPLATE_NAME, &json!({ "val": string }))
        .unwrap()
}

fn templates(path: PathBuf) -> HashMap<String, String> {
    fs::read_dir(path)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path().clone();
            (
                path.file_stem().unwrap().to_string_lossy().to_string(),
                template(path),
            )
        })
        .collect()
}

fn template(path: PathBuf) -> String {
    read(path)
}

fn default_templates_path(gen: Box<dyn Gen>) -> PathBuf {
    ("src/lib/gen/".to_string()
        + gen.src_dir().to_string_lossy().to_string().as_str()
        + "/templates")
        .into()
}
