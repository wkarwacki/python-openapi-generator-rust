use crate::{
    gen::template::r#fn::ToFlatCase,
    lib::{
        context::Context,
        gen::{
            lang::{Lang, DTO_NAME_TEMPLATE_NAME},
            template::{filter, fmt, proc, prop, r#fn, sort},
        },
        pkg::Pkg,
        util::read,
    },
};
use dyn_clone::DynClone;
use filter::{FilterNonconst, FilterOpParamsByLoc};
use fmt::{FmtClass, FmtEnum, FmtName, FmtOpt, FmtSrcIfPresent, FmtType, FmtValue};
use handlebars::Handlebars;
use proc::{Parents, Resolve, TypeArgs, TypeParams, ValueDef};
use prop::{HasKey, IsAlias};
use r#fn::{Add, Json};
use serde_json::json;
use sort::SortOptionalsLast;
use std::{collections::HashMap, fs, path::PathBuf};

pub(crate) trait Gen: DynClone + Send + Sync {
    fn lang(&self) -> Box<dyn Lang>;
    fn dtos(
        &self,
        handlebars: &Handlebars,
        pkg: &Pkg,
        context: &Context,
        templates: &HashMap<String, String>,
    ) -> HashMap<PathBuf, String>;
    fn ops(
        &self,
        handlebars: &Handlebars,
        pkg: &Pkg,
        context: &Context,
        templates: &HashMap<String, String>,
    ) -> HashMap<PathBuf, String>;
    fn src_dir(&self) -> PathBuf;
}

dyn_clone::clone_trait_object!(Gen);

pub(crate) fn go(
    pkg: &Pkg,
    gen: &Box<dyn Gen>,
    templates_path: &Option<PathBuf>,
    context: &Context,
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
    handlebars.register_helper("toFlatCase", Box::new(ToFlatCase {}));
    handlebars.register_helper("typeParams", Box::new(TypeParams {}.clone()));
    handlebars.register_helper("valueDef", Box::new(ValueDef {}.clone()));

    handlebars_misc_helpers::setup_handlebars(&mut handlebars);
    handlebars.set_strict_mode(false);

    let mut merged_templates: HashMap<_, _> = templates(&default_templates_path(gen));
    let templates: HashMap<String, String> = templates_path
        .as_ref()
        .map(templates)
        .unwrap_or(Default::default());
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

    let dtos = gen.dtos(&handlebars, pkg, context, &merged_templates);

    if pkg.ops.is_empty() {
        dtos
    } else {
        let ops = gen.ops(&handlebars, pkg, context, &merged_templates);
        let mut result = dtos;
        ops.iter().for_each(|op| {
            result.insert(op.0.clone(), op.1.clone());
        });
        result
    }
}

pub(crate) fn dto_name(str: &str, lang: &Box<dyn Lang>) -> String {
    lang.handlebars()
        .render(DTO_NAME_TEMPLATE_NAME, &json!({ "val": str }))
        .unwrap()
}

fn templates(path: &PathBuf) -> HashMap<String, String> {
    fs::read_dir(path)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            (
                path.file_stem().unwrap().to_string_lossy().to_string(),
                template(&path),
            )
        })
        .collect()
}

fn template(path: &PathBuf) -> String {
    read(path)
}

fn default_templates_path(gen: &Box<dyn Gen>) -> PathBuf {
    ("src/lib/gen/".to_string()
        + gen.src_dir().to_string_lossy().to_string().as_str()
        + "/templates")
        .into()
}
