use std::collections::HashMap;
use std::path::PathBuf;

use convert_case::{Case, Casing};
use handlebars::Handlebars;
use serde_json::json;

use crate::context::Context;
use crate::def::Def;
use crate::gen::gen::{dto_name, Gen};
use crate::gen::lang::Lang;
use crate::gen::python::lang_python::LangPython;
use crate::pkg::Pkg;

#[derive(Clone)]
pub struct GenPythonHttpClient {
    pub lang: LangPython
}

impl Gen for GenPythonHttpClient {
    fn lang(&self) -> Box<dyn Lang> {
        Box::new(self.lang.clone())
    }
    fn src_dir(&self) -> PathBuf {
        "python/client".into()
    }
    fn dtos(&self, handlebars: Handlebars, pkg: &Pkg, context: Context, templates: HashMap<String, String>) -> HashMap<PathBuf, String> {
        let out_dir = self.lang.out_dir().to_string_lossy().to_string();
        let mut defs: Vec<(String, Def, bool)> = Vec::new();
        defs.extend(pkg.defs.iter().map(|(def_name, def)| (def_name.clone(), def.clone(), false)));
        defs.extend(pkg.ops.iter().flat_map(|(_, ops)| ops).filter(|op| op.req.clone().and_then(|req| req.form.map(|form| form == "multipart/form-data")).unwrap_or(false))
            .flat_map(|op| op.req.iter().collect::<Vec<_>>().iter().flat_map(|req| req.desc.def().map(|def| (op.name.clone(), def.clone(), true))).collect::<Vec<_>>()));
        let dtos: HashMap<PathBuf, _> = defs.iter().map(|(def_name, def, form_like)| {
            let dto_template = templates.get("dtoFile").unwrap();
            let dto_path = def_name.to_case(Case::Snake).to_string() + ".py";
            let dto = handlebars.render_template(dto_template.as_str(), &json!({"key": dto_name(def_name.clone().as_mut_str().to_owned(), self.lang()), "val": def, "formLike": form_like})).unwrap();
            ({
                 let dto_path_str = dto_path.as_str();
                 format!("{out_dir}/{dto_path_str}").into()
             }, dto)
        }).collect();
        let imports = context.defs().iter().flat_map(|(src, defs)| defs.iter().map(move |def| "from trust.".to_string() + match src {
            None => {self.lang.feature.clone().to_case(Case::Snake)}
            Some(src) => {self.lang.fmt_src(src.clone())}
        }.as_str() + " import " + def.to_case(Case::Snake).as_str())).collect::<Vec<_>>().join("\n");
        let mut dtos_with_imports: HashMap<_, _> = dtos.iter().map(|(path, content)| (path.clone(), imports.clone() + "\n" + content)).collect();
        dtos_with_imports.insert((out_dir.clone() + "/__init__.py").into(), "".into());
        let trust_mod_template = templates.get("trustMod").unwrap();
        dtos_with_imports.insert("__init__.py".into(), trust_mod_template.clone());
        dtos_with_imports
    }

    fn ops(&self, handlebars: Handlebars, pkg: &Pkg, context: Context, templates: HashMap<String, String>) -> HashMap<PathBuf, String> {
        let mut result = HashMap::new();

        let dtos = self.dtos(handlebars.clone(), pkg, context, templates.clone());
        let imports = dtos.clone().keys()
            .map(|path| path.file_stem().unwrap().to_string_lossy().to_string())
            .filter(|path| path != "__init__")
            .map(|path| {
                "from trust.".to_string() + self.lang.feature.clone().to_case(Case::Snake).as_str() + " import " + path.to_case(Case::Snake).as_str()
            }).collect::<Vec<_>>().join("\n");

        let service = {
            let service_template = templates.get("service").unwrap();
            let service = handlebars.render_template(service_template.as_str(), &json!({"feature": self.lang.feature.clone(), "ops": &pkg.ops})).unwrap();
            ({
                 let out_dir = self.lang.out_dir().to_string_lossy().to_string();
                 let service_path = "service.py".to_string();
                 format!("{out_dir}/{service_path}").into()
             }, service)
        };
        result.insert(service.0, imports.clone() + "\n" + service.1.as_str());

        result
    }
}