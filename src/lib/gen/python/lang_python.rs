use crate::{
    lib::{
        context::Context,
        def::{Def, Enum, EnumVals},
        desc::Desc,
        gen::{
            gen::dto_name,
            lang::{inner, Lang},
        },
        r#ref::Ref,
    },
    GenCfg,
};
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct LangPython {
    pub gen_cfg: GenCfg,
    pub feature: String,
    pub handlebars: Handlebars<'static>,
}

impl LangPython {
    fn reserved_names() -> Vec<&'static str> {
        vec![
            "False", "None", "True", "and", "as", "assert", "break", "class", "continue", "def",
            "del", "elif", "else", "except", "finally", "for", "form", "from", "global", "if",
            "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return",
            "try", "while", "with", "yield",
        ]
    }

    pub(crate) fn dto_name_template() -> String {
        "{{val}}Dto".to_string()
    }
}

impl Lang for LangPython {
    fn handlebars(&self) -> Handlebars {
        self.clone().handlebars
    }

    fn out_dir(&self) -> PathBuf {
        self.pkg_name()
            .clone()
            .replace(".", "/")
            .to_case(Case::Snake)
            .into()
    }

    fn module(&self) -> String {
        self.gen_cfg
            .module
            .clone()
            .map(|path| {
                path.iter()
                    .map(|os_str| os_str.to_string_lossy().to_string())
                    .reduce(|string0, string1| string0 + "." + string1.as_str())
                    .unwrap()
                    + "."
            })
            .unwrap_or("trust".to_string())
    }

    fn pkg_name(&self) -> String {
        self.module() + "." + self.feature.as_str()
    }

    fn fmt_class(&self, class: &str, _origin: &Option<String>) -> String {
        Ref {
            src: None,
            path: class.to_string(),
        }
        .class_name()
        .to_case(Case::UpperCamel)
    }

    fn fmt_enum(&self, val: &str) -> String {
        match val.parse::<i64>() {
            Ok(_) => "_".to_string() + val.to_case(Case::UpperSnake).as_str(),
            _ => val.to_case(Case::UpperSnake),
        }
    }

    fn fmt_name(&self, name: &str) -> String {
        let formatted = name.split(".").last().unwrap().to_case(Case::Snake);

        if LangPython::reserved_names().contains(&formatted.as_str()) {
            "_".to_string() + &formatted
        } else {
            formatted
        }
    }

    fn fmt_opt(&self, str: &str) -> String {
        str.to_string() + " | None"
    }

    fn fmt_ref(&self, r#ref: &Ref) -> String {
        // FIXME_LATER: such implementation is strongly coupled with current python gens
        dto_name(
            r#ref.class_name().to_case(Case::Pascal).as_str(),
            &(Box::new(self.clone()) as Box<dyn Lang>),
        )
    }

    fn fmt_src(&self, src: &str) -> String {
        PathBuf::from(src)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_case(Case::Snake)
    }

    fn fmt_type(&self, def: &Def, name: &Option<&str>) -> String {
        match def {
            Def::Alias(alias) => self.fmt_ref(&alias.r#ref),
            Def::Bool(_) => "bool".to_string(),
            Def::Const(r#const) => r#const
                .desc
                .clone()
                .map(|desc| match *desc {
                    Desc::Def(def) => self.fmt_type(&def, name),
                    Desc::Ref(r#ref) => self.fmt_ref(&r#ref),
                    Desc::TypeParam { .. } => unimplemented!("Type parameter not supported yet."),
                })
                .unwrap_or("const".to_string()),
            Def::Dec(_) => "float".to_string(),
            Def::Enum(Enum { vals, null: _ }) => match vals {
                EnumVals::Int { .. } => "int".to_string(),
                EnumVals::Str { .. } => name.unwrap_or("str").to_string(),
            },
            Def::Int(_) => "int".to_string(),
            Def::Map(map) => {
                let key = inner(
                    &map.key,
                    "Key",
                    name,
                    &(Box::new(self.clone()) as Box<dyn Lang>),
                );
                let val = inner(
                    &map.val,
                    "Value",
                    name,
                    &(Box::new(self.clone()) as Box<dyn Lang>),
                );
                "dict[".to_string() + key.as_str() + ", " + val.as_str() + "]"
            }
            Def::Obj(_) => name.unwrap().to_string(),
            Def::Seq(seq) => {
                let item = inner(
                    &seq.item,
                    "Item",
                    name,
                    &(Box::new(self.clone()) as Box<dyn Lang>),
                );
                "list[".to_string() + item.as_str() + "]"
            }
            Def::Str(_) => name
                .and_then(|n| self.gen_cfg.type_mapping.get(n))
                .unwrap_or(&"str".to_string())
                .clone(),
            Def::Struct(_) => "Any".to_string(),
        }
    }

    fn fmt_value(&self, json_value: &Value, desc: &Option<Desc>, name: &Option<&str>, context: &Context) -> String {
        let def = desc.as_ref().map(|d| match d {
            Desc::Def(def) => def.clone(),
            Desc::Ref(r#ref) => context.resolve(&r#ref),
            _ => unreachable!(),
        });
        match json_value {
            Value::Bool(val) => if val.clone() { "True" } else { "False" }.to_string(),
            Value::Number(val) => val.to_string(),
            Value::String(val) => def.as_ref().map(|d| match d {
                Def::Enum(Enum { vals, null: _ }) => match vals {
                    EnumVals::Int(_) => self.fmt_type(d, name) + "." + self.fmt_enum(val).as_str(),
                    EnumVals::Str(_) => self.fmt_type(d, name) + "." + self.fmt_enum(val).as_str()
                },
                _ => val.to_string(),
            }).unwrap_or(val.clone()),
            Value::Array(val) => {
                let item = def.map(|d| match d {
                    Def::Seq(seq) => seq.item,
                    _ => unreachable!()
                });
                let name = match item {
                    Some(Desc::Ref(ref r#ref)) => Some(self.fmt_ref(r#ref)),
                    _ => None
                };
                "[".to_string()
                    + val
                        .iter()
                        .map(|json_value| self.fmt_value(json_value, &item, &name.as_deref(), context))
                        .collect::<Vec<String>>()
                        .join(", ")
                        .as_str()
                    + "]"
            }
            Value::Object(val) => val
                .iter()
                .map(|(key, json_value)| key.clone() + ": " + self.fmt_value(json_value, &None, &None, context).as_str())
                .collect::<Vec<String>>()
                .join(", "),
            Value::Null => "None".to_string(),
        }
    }

    fn stub_impl(&self, def: &Def, context: &Context) -> String {
        match def {
            Def::Alias(alias) => self.stub_impl(&context.resolve(&alias.r#ref), context),
            Def::Bool(_) => "False".to_string(),
            Def::Const(r#const) => serde_json::to_string(&r#const.clone().val).unwrap(),
            Def::Dec(_) => "0.0".to_string(),
            Def::Enum(Enum { vals, null: _ }) => match vals {
                EnumVals::Int(vals) => vals.first().unwrap().to_string(),
                EnumVals::Str(vals) => "\"".to_string() + vals.first().unwrap() + "\"",
            },
            Def::Int(_) => "0".to_string(),
            Def::Str(_) => "\"str\"".to_string(),
            Def::Struct(_) => "{}".to_string(),
            _ => unreachable!(),
        }
    }
}
