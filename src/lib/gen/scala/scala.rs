/*use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use crate::lib::context::Context;
use crate::lib::def::{Def, Enum};
use crate::lib::desc::Desc;
use crate::lib::gen::gen::Gen;
use crate::lib::GenCfg;
use crate::lib::pkg::Pkg;
use crate::lib::r#ref::Ref;

#[derive(Clone)]
pub(crate) struct Scala {
    pub gen_cfg: GenCfg,
    pub feature: String,
}

impl Scala {
    fn pkg_name(&self) -> String {
        self.gen_cfg.module.clone().map(|path| path.iter().map(|os_str| os_str.to_string_lossy().to_string()).reduce(|string0, string1| string0 + "." + string1.as_str()).unwrap() + ".").unwrap_or("".to_string()) + self.feature.as_str()
    }
}

impl Gen for Scala {
    fn clone_box(&self) -> Box<dyn Gen> {
        Box::new(self.clone())
    }

    fn out_dir(&self) -> PathBuf {
        self.pkg_name().clone().replace(".", "/").to_case(Case::Flat).into()
    }

    fn templates_dir(&self) -> PathBuf {
        "scala".into()
    }

    fn fmt_class(&self, class: String, origin: Option<String>) -> String {
        self.fmt_ref(Ref{src: origin, path: Ref{src: None, path: class }.class_name().to_case(Case::UpperCamel)})
    }

    fn fmt_enum(&self, val: String) -> String {
        match val.parse::<i64>() {
            Ok(_) => "`".to_string() + val.as_str() + "`", // TIDY: string interpolation | TIDY: unify with fmt_name ?
            _ => val
        }.to_case(Case::UpperSnake)
    }

    fn fmt_name(&self, name: String) -> String {
        let reserved_names = vec!(
            "fun", "type", "val", "var" // TODO: take list from somewhere
        );

        let formatted = name.split(".").last().unwrap().to_case(Case::Camel);

        if reserved_names.contains(&formatted.as_str()) {
            "`".to_string() + &formatted + "`"
        } else {
            formatted
        }
    }

    fn fmt_opt(&self, string: String) -> String {
        "Option[".to_string() + string.as_str() + "]"
    }

    fn fmt_ref(&self, r#ref: Ref) -> String {
        r#ref.clone().src.map(|src| self.fmt_src(src) + ".").unwrap_or("".to_string()) +  r#ref.class_name().as_str() + "Dto" // TODO: encapsulate Dto addition | TIDY: interpolation
    }

    fn fmt_src(&self, src: String) -> String {
        Path::with_extension(Path::new(self.pkg_name().clone().as_str()), "").to_string_lossy().to_string() + "." + PathBuf::from(src).file_stem().unwrap().to_str().unwrap().to_case(Case::Flat).as_str() // TODO: encapsulate Dto addition | TIDY: interpolation
    }

    fn fmt_type(&self, def: Def, name: Option<String>) -> String {
        match def {
            Def::Bool(_) => "Boolean".to_string(),
            Def::Const(_const) => todo!(),
            Def::Dec(_) => "Double".to_string(),
            Def::Enum{vals, null: _} => match vals {
                Enum::Int { .. } => "Int".to_string(),
                Enum::Str { .. } => name.unwrap_or("String".to_string())
            },
            Def::Int(_) => "Long".to_string(),
            Def::Map(map) => {
                let key = match map.key {
                    Desc::Def(def) => self.fmt_type(def.clone(), name.clone().map(|n| n + ".Key").or(Some("Key".to_string()))).replace(".Key.Key", ".Key"),
                    Desc::Ref(r#ref) => self.fmt_ref(r#ref),
                    _ => todo!()
                };
                let val = match map.val {
                    Desc::Def(def) => self.fmt_opt(self.fmt_type(def.clone(), name.clone().map(|n| n + ".Value").or(Some("Value".to_string()))).replace(".Value.Value", ".Value")),
                    Desc::Ref(r#ref) => self.fmt_ref(r#ref),
                    _ => todo!()
                }; // TIDY: merge both above into one fn
                "Map[".to_string() + key.as_str() + ", " + val.as_str() + "]" // TIDY: string interpolation
            }
            Def::Obj(_) => name.unwrap(),
            Def::Seq(seq) => {
                let item = match seq.item {
                    Desc::Def(def) => self.fmt_type(def.clone(), name.clone().map(|string| string + ".Item").or(Some("Item".to_string()))).replace(".Item.Item", ".Item"),
                    Desc::Ref(r#ref) => self.fmt_ref(r#ref),
                    _ => todo!()
                }; // TIDY: merge also this into one fn
                "Seq[".to_string() + item.as_str() + "]" // TIDY: string interpolation
            }
            Def::Str(_) => name.and_then(|n| self.gen_cfg.type_mapping.get(n.as_str())).unwrap_or(&"String".to_string()).clone(),
            Def::Struct(_) => "io.circe.Json".to_string(), // TODO: Json reference to variable
        }
    }

    fn dtos(&self, handlebars: Handlebars, pkg: &Pkg, _context: Context) -> HashMap<PathBuf, String> {
        pkg.defs.iter().map(|(def_name, def)| {
            let dto_template_path = self.templates_path().to_string_lossy().to_string() + "/dto_file.hbs";
            let mut dto_template = String::new();
            File::open(dto_template_path)
                .unwrap()
                .read_to_string(&mut dto_template)
                .unwrap();
            let dto_path = def_name.to_string() + "Dto.scala"; // TIDY: string interpolation
            let dto = handlebars.render_template(dto_template.as_str(), &(def_name.clone().as_mut_str().to_owned() + "Dto", def, self.pkg_name().clone())).unwrap(); // TIDY: pass treemap instead of tuple
            ({
                 let out_dir = self.out_dir().to_string_lossy().to_string();
                 let dto_path_str = dto_path.as_str();
                 format!("{out_dir}/{dto_path_str}").into()
             }, dto)
        }).collect()
    }

    fn ops(&self, handlebars: Handlebars, pkg: &Pkg, _context: Context) -> HashMap<PathBuf, String> {
        let routes = {
            let routes_template_path = self.templates_path().to_string_lossy().to_string() + "/routes.hbs";
            let mut routes_template = String::new();
            File::open(routes_template_path)
                .unwrap()
                .read_to_string(&mut routes_template)
                .unwrap();
            let routes = handlebars.render_template(routes_template.as_str(), &(self.feature.clone(), &pkg.ops, self.pkg_name().clone())).unwrap(); // TIDY: pass treemap instead of tuple
            ({
                 let out_dir = self.out_dir().to_string_lossy().to_string();
                 let routes_path = self.clone().feature.to_case(Case::Pascal) + "Routes.scala";
                 format!("{out_dir}/{routes_path}").into()
             }, routes)
        };

        let service = {
            let service_template_path = self.templates_path().to_string_lossy().to_string() + "/service.hbs";
            let mut service_template = String::new();
            File::open(service_template_path)
                .unwrap()
                .read_to_string(&mut service_template)
                .unwrap();
            let service = handlebars.render_template(service_template.as_str(), &(self.feature.clone(), &pkg.ops, self.pkg_name().clone())).unwrap(); // TIDY: pass treemap instead of tuple
            ({
                 let out_dir = self.out_dir().to_string_lossy().to_string();
                 let service_path = self.feature.clone().to_case(Case::Pascal) + "Service.scala";
                 format!("{out_dir}/{service_path}").into()
             }, service)
        };

        let mut result = HashMap::new();
        result.insert(routes.0, routes.1);
        result.insert(service.0, service.1);
        result
    }

    fn templates(&self) -> HashMap<String, PathBuf> {
        HashMap::from([
            ("companion".to_string(), "companion.hbs".into()),
            ("dto".to_string(), "dto.hbs".into()),
            ("type_args".to_string(), "type_args.hbs".into()),
            ("type_params".to_string(), "type_params.hbs".into()),
            ("var".to_string(), "var.hbs".into())
        ])
    }
}*/
