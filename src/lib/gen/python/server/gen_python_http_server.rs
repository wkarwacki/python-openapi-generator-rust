use crate::lib::{
    context::Context,
    def::Def,
    desc::Desc,
    gen::{
        gen::{dto_name, Gen},
        lang::Lang,
        python::{lang_python::LangPython, server::templates::Templates},
        templates::Templates as GenTemplates,
    },
    pkg::Pkg,
};
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use itertools::Itertools;
use serde_json::json;
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone)]
pub(crate) struct GenPythonHttpServer {
    pub lang: LangPython,
}

impl GenPythonHttpServer {
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

impl Gen for GenPythonHttpServer {
    fn lang(&self) -> Box<dyn Lang> {
        Box::new(self.lang.clone())
    }
    fn templates(&self) -> HashMap<String, String> {
        Templates {}.default()
    }
    fn src_dir(&self) -> PathBuf {
        "python/server".into()
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
                            Def::Seq(_) =>
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
                                        + "Item",
                                ),
                            _ => {}
                        }
                        names
                    })
                })
                .unique()
                .collect::<Vec<_>>()
                .join("\n");
            let dto = handlebars.render_template(dto_template.as_str(), &json!({"key": dto_name(def_name, &self.lang()), "val": def, "formLike": form_like, "mod": &self.lang().module()})).unwrap();
            ({
                 let dto_path_str = dto_path.as_str();
                 format!("{out_dir}/{dto_path_str}").into()
             }, imports + "\n" + dto.as_str())
        }).collect();
        dtos.insert((out_dir.clone() + "/__init__.py").into(), "".into());
        let type_mapping = self
            .lang
            .gen_cfg
            .type_mapping
            .iter()
            .filter(|(from, _)| defs.iter().find(|(name, _, _)| &name == from).is_some())
            .collect::<HashMap<_, _>>();
        let type_mapping_imports = type_mapping
            .iter()
            .flat_map(|(from, to)| {
                let split = to.split_at(to.rfind('.').unwrap());
                vec![
                    "from ".to_string() + split.0 + " import " + &split.1[1..],
                    "from ".to_string()
                        + self.lang.module().as_str()
                        + "."
                        + self.lang.feature.clone().to_case(Case::Snake).as_str()
                        + "."
                        + self.lang.fmt_name(from).as_str()
                        + " import "
                        + dto_name(from, &self.lang()).as_str(),
                ]
            })
            .join("\n");
        let type_mapping_template = templates.get("typeMapping").unwrap();
        let type_mapping = handlebars
            .render_template(
                type_mapping_template.as_str(),
                &json!({ "typeMapping": type_mapping }),
            )
            .unwrap();
        dtos.insert(
            (out_dir.clone() + "/type_mapping.py").into(),
            type_mapping_imports + "\n" + &type_mapping,
        );

        let trust_mod_template = templates.get("trustMod").unwrap();
        let trust_mod_path = self.lang.clone().gen_cfg.module.unwrap_or("trust".into());
        dtos.insert(
            (trust_mod_path.to_string_lossy().to_string() + "/__init__.py").into(),
            trust_mod_template.clone(),
        );

        dtos.insert(
            (trust_mod_path.to_string_lossy().to_string() + "/py.typed").into(),
            "".into(),
        );

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
                            Def::Seq(_) => names.push(
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
                                    + "Item",
                            ),
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

        let router = {
            let router_template = templates.get("router").unwrap();
            let router = handlebars
                .render_template(
                    router_template.as_str(),
                    &json!({"feature": self.lang.feature.clone(), "ops": &pkg.ops, "useNamespace": &pkg.use_namespace}),
                )
                .unwrap();
            (
                {
                    let out_dir = self.lang.out_dir().to_string_lossy().to_string();
                    let router_path = "router.py".to_string();
                    format!("{out_dir}/{router_path}").into()
                },
                router,
            )
        };
        result.insert(
            router.0,
            imports.clone() + "\nfrom .type_mapping import *\n" + router.1.as_str(),
        );

        let type_mapping = self
            .lang
            .gen_cfg
            .type_mapping
            .iter()
            .filter(|(from, _)| {
                pkg.ops
                    .iter()
                    .flat_map(|(_, ops)| {
                        ops.iter()
                            .flat_map(|op| {
                                op.req
                                    .iter()
                                    .flat_map(|req| req.desc.r#ref())
                                    .chain(op.res.iter().flat_map(|res| res.desc.r#ref()))
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>()
                    })
                    .find(|r#ref| &&self.lang.fmt_class(&r#ref.path, &None) == from)
                    .is_some()
            })
            .collect::<HashMap<_, _>>();
        let type_mapping_imports = type_mapping
            .iter()
            .flat_map(|(from, to)| {
                let split = to.split_at(to.rfind('.').unwrap());
                vec![
                    "from ".to_string() + split.0 + " import " + &split.1[1..],
                    "from ".to_string()
                        + self.lang.module().as_str()
                        + "."
                        + self.lang.feature.clone().to_case(Case::Snake).as_str()
                        + "."
                        + self.lang.fmt_name(from).as_str()
                        + " import "
                        + dto_name(from, &self.lang()).as_str(),
                ]
            })
            .join("\n");

        let service = {
            let service_template = templates.get("service").unwrap();
            let service = handlebars
                .render_template(
                    service_template.as_str(),
                    &json!({"feature": self.lang.feature.clone(), "ops": &pkg.ops, "autoImpl": self.lang.gen_cfg.auto_implement}),
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
        result.insert(
            service.0,
            imports.clone() + "\n" + type_mapping_imports.as_str() + "\n" + service.1.as_str(),
        );

        result
    }
}
