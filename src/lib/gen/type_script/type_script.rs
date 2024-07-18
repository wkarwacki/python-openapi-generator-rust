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
pub(crate) struct TypeScript {
    pub gen_cfg: GenCfg,
    pub feature: String,
}

impl TypeScript {
    fn pkg_name(&self) -> String {
        self.gen_cfg.module.clone().map(|path| path.iter().map(|os_str| os_str.to_string_lossy().to_string()).reduce(|string0, string1| string0 + "." + string1.as_str()).unwrap() + ".").unwrap_or("".to_string()) + self.feature.as_str()
    }
}

impl Gen for TypeScript {
    fn clone_box(&self) -> Box<dyn Gen> {
        Box::new(self.clone())
    }

    fn out_dir(&self) -> PathBuf {
        self.pkg_name().clone().replace(".", "/").to_case(Case::Camel).into()
    }

    fn templates_dir(&self) -> PathBuf {
        "type_script".into()
    }

    fn fmt_class(&self, class: String, _origin: Option<String>) -> String {
        Ref{src: None, path: class }.class_name().to_case(Case::UpperCamel)
    }

    fn fmt_enum(&self, val: String) -> String {
        match val.parse::<i64>() {
            Ok(_) => {
                "_".to_string() + val.as_str().to_case(Case::UpperSnake).as_str()
            }, // TIDY: string interpolation | TIDY: unify with fmt_name ?
            _ => val.to_case(Case::UpperSnake)
        }
    }

    fn fmt_name(&self, name: String) -> String {
        let reserved_names = vec!(
             "from"// TODO: take list from somewhere
        );

        let formatted = name.split(".").last().unwrap().to_case(Case::Camel);

        if reserved_names.contains(&formatted.as_str()) {
            "_".to_string() + &formatted
        } else {
            formatted
        }
    }

    fn fmt_opt(&self, string: String) -> String {
        string + " | None"
    }

    fn fmt_ref(&self, r#ref: Ref) -> String {
        r#ref.clone().src.map(|src| self.fmt_src(src)+ ".").unwrap_or("".to_string()).to_case(Case::Camel)  + r#ref.class_name().to_case(Case::Camel).as_str() + "." + r#ref.class_name().as_str() + "Dto" // TODO: encapsulate Dto addition | TIDY: interpolation
    }

    fn fmt_src(&self, src: String) -> String {
        Path::with_extension(Path::new(self.pkg_name().clone().as_str()), "").to_string_lossy().to_string() + "." + PathBuf::from(src).file_stem().unwrap().to_str().unwrap().to_case(Case::Flat).as_str() // TODO: encapsulate Dto addition | TIDY: interpolation
    }

    fn fmt_type(&self, def: Def, name: Option<String>) -> String {
        match def {
            Def::Bool(_) => "boolean".to_string(),
            Def::Const(_const) => todo!(),
            Def::Dec(_) => "number".to_string(),
            Def::Enum{vals, null: _} => match vals {
                Enum::Int { .. } => "number".to_string(),
                Enum::Str { .. } => name.unwrap_or("string".to_string())
            },
            Def::Int(_) => "number".to_string(),
            Def::Map(map) => {
                let key = match map.key {
                    Desc::Def(def) => self.fmt_type(def.clone(), name.clone().map(|n| n + ".Key").or(Some("Key".to_string()))).replace(".Key.Key", ".Key"),
                    Desc::Ref(r#ref) => self.fmt_ref(r#ref),
                    _ => todo!()
                };
                let val = match map.val {
                    Desc::Def(def) => self.fmt_opt(self.fmt_type(def.clone(), name.clone().map(|n| n + "Value").or(Some("Value".to_string())))),
                    Desc::Ref(r#ref) => self.fmt_ref(r#ref),
                    _ => todo!()
                }; // TIDY: merge both above into one fn
                "{ [key: ".to_string() + key.as_str() + "]: " + val.as_str() + "; ]" // TIDY: string interpolation
            }
            Def::Obj(_) => name.unwrap(),
            Def::Seq(seq) => {
                let item = match seq.item {
                    Desc::Def(def) => self.fmt_type(def.clone(), name.clone().map(|string| string + "Item").or(Some("Item".to_string()))),
                    Desc::Ref(r#ref) => self.fmt_ref(r#ref),
                    _ => todo!()
                }; // TIDY: merge also this into one fn
                "Array<".to_string() + item.as_str() + ">" // TIDY: string interpolation
            }
            Def::Str(_) => name.and_then(|n| self.gen_cfg.type_mapping.get(n.as_str())).unwrap_or(&"string".to_string()).clone(),
            Def::Struct(_) => "any".to_string(), // TODO: Json reference to variable
        }
    }

    fn dtos(&self, handlebars: Handlebars, pkg: &Pkg, _context: Context) -> HashMap<PathBuf, String> {
        let out_dir = self.out_dir().to_string_lossy().to_string();
        let dtos: HashMap<PathBuf, _> = pkg.defs.iter().map(|(def_name, def)| {
            let dto_template_path = self.templates_path().to_string_lossy().to_string() + "/dto_file.hbs";
            let mut dto_template = String::new();
            File::open(dto_template_path)
                .unwrap()
                .read_to_string(&mut dto_template)
                .unwrap();
            let dto_path = def_name.to_case(Case::Kebab).to_string() + ".ts"; // TIDY: string interpolation
            let dto = handlebars.render_template(dto_template.as_str(), &(def_name.clone().as_mut_str().to_owned() + "Dto", def, self.pkg_name().clone())).unwrap(); // TIDY: pass treemap instead of tuple
            ({
                 let dto_path_str = dto_path.as_str();
                 format!("{out_dir}/{dto_path_str}").into()
             }, dto)
        }).collect();
        let imports = dtos.clone().keys().map(|path| {
            "from trust.".to_string() + self.feature.clone().to_case(Case::Camel).as_str() + " import " + path.file_stem().unwrap().to_string_lossy().to_string().to_case(Case::Camel).as_str()
        }).collect::<Vec<_>>().join("\n");
        let mut dtos_with_imports: HashMap<_, _> = dtos.iter().map(|(path, content)| (path.clone(), imports.clone() + "\n" + content)).collect();
        dtos_with_imports.insert((out_dir.clone() + "/__init__.ts").into(), "".into());
        let trust_mod_template_path = self.templates_path().to_string_lossy().to_string() + "/trust_mod.hbs";
        let mut trust_mod_template = String::new();
        File::open(trust_mod_template_path)
            .unwrap()
            .read_to_string(&mut trust_mod_template)
            .unwrap();
        dtos_with_imports.insert("__init__.ts".into(), "".into());
        dtos_with_imports
    }

    fn ops(&self, handlebars: Handlebars, pkg: &Pkg, context: Context) -> HashMap<PathBuf, String> {
        let router = {
            let router_template_path = self.templates_path().to_string_lossy().to_string() + "/router.hbs";
            let mut router_template = String::new();
            File::open(router_template_path)
                .unwrap()
                .read_to_string(&mut router_template)
                .unwrap();
            let router = handlebars.render_template(router_template.as_str(), &(self.feature.clone(), &pkg.ops, self.pkg_name().clone())).unwrap(); // TIDY: pass treemap instead of tuple
            ({
                 let out_dir = self.out_dir().to_string_lossy().to_string();
                 let router_path = "router.ts".to_string();
                 format!("{out_dir}/{router_path}").into()
             }, router)
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
                 let service_path = "service.ts".to_string();
                 format!("{out_dir}/{service_path}").into()
             }, service)
        };

        let dtos = self.dtos(handlebars, pkg, context);
        let imports = dtos.clone().keys()
            .map(|path| path.file_stem().unwrap().to_string_lossy().to_string())
            .filter(|path| path != "__init__")
            .map(|path| {
            "from trust.".to_string() + self.feature.clone().to_case(Case::Camel).as_str() + " import " + path.to_case(Case::Camel).as_str()
        }).collect::<Vec<_>>().join("\n");

        let mut result = HashMap::new();
        result.insert(router.0, imports.clone() + "\n" + router.1.as_str());
        result.insert(service.0, imports.clone() + "\n" + service.1.as_str());
        result
    }

    fn templates(&self) -> HashMap<String, PathBuf> {
        HashMap::from([
            ("dto".to_string(), "dto.hbs".into()),
            ("type_args".to_string(), "type_args.hbs".into()),
            ("type_params".to_string(), "type_params.hbs".into()),
            ("union".to_string(), "union.hbs".into()),
            ("var".to_string(), "var.hbs".into())
        ])
    }
}*/
