use crate::lib::{
    context::Context,
    def::Def,
    desc::Desc,
    gen::{
        gen::{dto_name, Gen},
        lang::Lang,
        python::lang_python::LangPython,
    },
    pkg::Pkg,
};
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use itertools::Itertools;
use serde_json::json;
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone)]
pub(crate) struct GenPythonHttpClient {
    pub lang: LangPython,
}

impl GenPythonHttpClient {
    fn descs_from_inline_ops(&self, pkg: &Pkg) -> Vec<(String, Desc, Option<String>)> {
        pkg.ops
            .iter()
            .flat_map(|(_, ops)| ops)
            .flat_map(|op| {
                let req_dto = op.req.clone().map(|req| {
                    (
                        self.lang
                            .fmt_class((op.name.clone() + "Req").as_str(), &None),
                        req.desc,
                        req.form,
                    )
                });
                let res_dto = op.res.clone().map(|res| {
                    (
                        self.lang
                            .fmt_class((op.name.clone() + "Res").as_str(), &None),
                        res.desc,
                        res.form,
                    )
                });
                let mut vec = Vec::new();
                if let Some(tuple) = req_dto {
                    tuple.1.clone().def().iter().for_each(|def| match def {
                        Def::Obj(_) | Def::Seq(_) | Def::Map(_) => vec.push(tuple.clone()),
                        _ => {}
                    });
                }
                if let Some(tuple) = res_dto {
                    tuple.1.clone().def().iter().for_each(|def| match def {
                        Def::Obj(_) | Def::Seq(_) | Def::Map(_) => vec.push(tuple.clone()),
                        _ => {}
                    });
                }
                vec
            })
            .filter(|(_, desc, _)| desc.def().is_some())
            .collect()
    }
}

impl Gen for GenPythonHttpClient {
    fn lang(&self) -> Box<dyn Lang> {
        Box::new(self.lang.clone())
    }
    fn src_dir(&self) -> PathBuf {
        "python/client".into()
    }

    fn dtos(
        &self,
        handlebars: &Handlebars,
        pkg: &Pkg,
        context: &Context,
        templates: &HashMap<String, String>,
    ) -> HashMap<PathBuf, String> {
        let out_dir = self.lang.out_dir().to_string_lossy().to_string();
        let mut defs: Vec<(String, Def, bool)> = Vec::new();
        let descs_from_inline_ops: Vec<_> = self.descs_from_inline_ops(pkg);
        defs.extend(descs_from_inline_ops.iter().flat_map(|(name, desc, form)| {
            desc.def().map(|def| {
                (
                    name.clone(),
                    def.clone(),
                    form.clone()
                        .map(|f| f == "multipart/form-data")
                        .unwrap_or(false),
                )
            })
        }));
        let form_ops: Vec<_> = descs_from_inline_ops
            .iter()
            .filter(|(_, _, form)| {
                form.clone()
                    .map(|f| f == "multipart/form-data")
                    .unwrap_or(false)
            })
            .collect();
        defs.extend(pkg.defs.iter().map(|(def_name, def)| {
            (def_name.clone(), def.clone(), {
                let form_refs = form_ops
                    .iter()
                    .flat_map(|(_, desc, _)| {
                        desc.r#ref()
                            .map(|r#ref| r#ref.path.clone().rsplit_once('.').unwrap().1.to_string())
                    })
                    .collect::<Vec<_>>();
                form_refs.contains(def_name) // FIXME: take src into account as well
            })
        }));
        let mut dtos: HashMap<PathBuf, _> = defs.iter().map(|(def_name, def, form_like)| {
            let dto_template = templates.get("dtoFile").unwrap();
            let dto_path = def_name.to_case(Case::Snake).to_string() + ".py";
            let imports = context
                .def_refs(def)
                .iter()
                .flat_map(|(src, refs)| {
                    refs.iter().flat_map(move |r#ref| {
                        let mut names: Vec<String> = Default::default();
                        names.push("from ".to_string()
                            + self.lang.module().as_str()
                            + "."
                            + match &src {
                            None => self.lang.feature.clone().to_case(Case::Snake),
                            Some(src) => self.lang.fmt_src(src.as_str()),
                        }
                            .as_str()
                            + "."
                            + r#ref.class_name().to_case(Case::Snake).as_str()
                            + " import "
                            + dto_name(self.lang.fmt_class(r#ref.class_name().as_str(), &None).as_str(), &self.lang())
                            .as_str());
                        match context.resolve(r#ref) {
                            Def::Obj(obj) => {
                                obj.adt.iter().for_each(|_| names.push("from ".to_string()
                                    + self.lang.module().as_str()
                                    + "."
                                    + match &src {
                                    None => self.lang.feature.clone().to_case(Case::Snake),
                                    Some(src) => self.lang.fmt_src(src.as_str()),
                                }
                                    .as_str()
                                    + "."
                                    + r#ref.class_name().to_case(Case::Snake).as_str()
                                    + " import "
                                    + dto_name(self.lang.fmt_class(r#ref.class_name().as_str(), &None).as_str(), &self.lang())
                                    .as_str()
                                + "Base"))
                            }
                            _ => {}
                        }
                        names
                    })
                })
                .unique()
                .collect::<Vec<_>>()
                .join("\n");
            let dto = handlebars.render_template(dto_template.as_str(), &json!({"key": dto_name(def_name, &self.lang()), "val": def, "formLike": form_like, "imports": imports})).unwrap();
            ({
                 let dto_path_str = dto_path.as_str();
                 format!("{out_dir}/{dto_path_str}").into()
             }, dto)
        }).collect();
        dtos.insert((out_dir.clone() + "/__init__.py").into(), "".into());
        let trust_mod_template = templates.get("trustMod").unwrap();
        dtos.insert(
            (self.lang.module() + "/__init__.py").into(),
            trust_mod_template.clone(),
        );

        self.lang.gen_cfg.module.iter().for_each(|path| {
            path.iter().fold(None, |path: Option<String>, os_str| {
                let p = match path {
                    None => os_str.to_string_lossy().to_string(),
                    Some(str) => {
                        str.to_string() + "/" + os_str.to_string_lossy().to_string().as_str()
                    }
                };
                dtos.insert((p.clone() + "/__init__.py").into(), "".into());
                Some(p)
            });
        });

        dtos
    }

    fn ops(
        &self,
        handlebars: &Handlebars,
        pkg: &Pkg,
        context: &Context,
        templates: &HashMap<String, String>,
    ) -> HashMap<PathBuf, String> {
        let mut result = HashMap::new();

        let mut imports_vec = Vec::new();
        pkg.ops
            .iter()
            .flat_map(|(_, ops)| ops.iter().flat_map(|op| context.op_refs(op)))
            .flat_map(|(src, refs)| {
                refs.iter()
                    .flat_map(|r#ref| {
                        let mut names: Vec<String> = Default::default();
                        names.push(
                            "from ".to_string()
                                + self.lang.module().as_str()
                                + "."
                                + match &src {
                                    None => self.lang.feature.clone().to_case(Case::Snake),
                                    Some(src) => self.lang.fmt_src(src.as_str()),
                                }
                                .as_str()
                                + "."
                                + r#ref.class_name().to_case(Case::Snake).as_str()
                                + " import "
                                + dto_name(
                                    self.lang
                                        .fmt_class(r#ref.class_name().as_str(), &None)
                                        .as_str(),
                                    &self.lang(),
                                )
                                .as_str(),
                        );
                        match context.resolve(r#ref) {
                            Def::Obj(obj) => obj.adt.iter().for_each(|_| {
                                names.push(
                                    "from ".to_string()
                                        + self.lang.module().as_str()
                                        + "."
                                        + match &src {
                                            None => self.lang.feature.clone().to_case(Case::Snake),
                                            Some(src) => self.lang.fmt_src(src.as_str()),
                                        }
                                        .as_str()
                                        + "."
                                        + r#ref.class_name().to_case(Case::Snake).as_str()
                                        + " import "
                                        + dto_name(
                                            self.lang
                                                .fmt_class(r#ref.class_name().as_str(), &None)
                                                .as_str(),
                                            &self.lang(),
                                        )
                                        .as_str()
                                        + "Base",
                                )
                            }),
                            _ => {}
                        }
                        names
                    })
                    .collect::<Vec<_>>()
            })
            .for_each(|import| imports_vec.push(import));
        self.descs_from_inline_ops(pkg)
            .iter()
            .flat_map(|(name, desc, _)| {
                desc.refs()
                    .iter()
                    .map(|r#ref| r#ref.class_name())
                    .chain(Some(name.clone()).into_iter())
                    .collect::<Vec<_>>()
            })
            .map(|name| {
                "from ".to_string()
                    + self.lang.module().as_str()
                    + "."
                    + self.lang.feature.clone().to_case(Case::Snake).as_str()
                    + "."
                    + name.to_case(Case::Snake).as_str()
                    + " import "
                    + dto_name(
                        self.lang.fmt_class(name.as_str(), &None).as_str(),
                        &self.lang(),
                    )
                    .as_str()
            })
            .for_each(|import| imports_vec.push(import));

        let imports = imports_vec
            .iter()
            .unique()
            .map(String::clone)
            .collect::<Vec<_>>()
            .join("\n");

        let service = {
            let service_template = templates.get("service").unwrap();
            let service = handlebars
                .render_template(
                    service_template.as_str(),
                    &json!({"feature": self.lang.feature.clone(), "ops": &pkg.ops}),
                )
                .unwrap();
            (
                {
                    let out_dir = self.lang.out_dir().to_string_lossy().to_string();
                    let service_path = "service.py".to_string();
                    format!("{out_dir}/{service_path}").into()
                },
                service,
            )
        };
        result.insert(service.0, imports.clone() + "\n" + service.1.as_str());

        result
    }
}
