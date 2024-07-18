use crate::lib::{
    context::Context,
    def,
    def::{Def, Def::Obj},
    open_api::schema::{schemas_path, Schema},
    var::Var,
};
use convert_case::{Case, Casing};
use def::DEFS;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static COMPONENTS: &str = "components";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Components {
    pub schemas: HashMap<String, Schema>,
}

impl Components {
    pub(crate) fn is_empty(&self) -> bool {
        self.schemas.is_empty()
    }
    pub(crate) fn of(defs: HashMap<String, Def>, context: &Context) -> Components {
        let with_synth_adt_refs_schemas: HashMap<_, _> = defs
            .iter()
            .flat_map(|(name, def)| {
                def.obj()
                    .iter()
                    .flat_map(|obj| {
                        obj.adt
                            .iter()
                            .flat_map(|adt| adt.clone().map)
                            .map(|(subname, subtype)| {
                                (
                                    name.to_string()
                                        + subname.clone().to_case(Case::UpperCamel).as_str(),
                                    Schema {
                                        all_of: {
                                            let mut vec = Vec::new();
                                            vec.push(Schema {
                                                _ref: Some({
                                                    let schemas_path = schemas_path();
                                                    format!("#{schemas_path}/{name}")
                                                }),
                                                ..Default::default()
                                            });
                                            vec.push(Schema::of_def(
                                                Obj(subtype),
                                                &subname,
                                                None,
                                                context,
                                            ));
                                            vec
                                        },
                                        ..Default::default()
                                    },
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let with_synth_param_subtypes_schemas: HashMap<_, _> = defs
            .iter()
            .flat_map(|(def_name, def)| {
                def.obj()
                    .iter()
                    .flat_map(|obj| {
                        obj.ext.iter().map(|ext| {
                            let supertype = defs
                                .iter()
                                .find(|(d_name, _d)| {
                                    ext.r#ref.path.clone()
                                        == (DEFS.to_string() + "." + d_name.as_str())
                                })
                                .unwrap()
                                .1
                                .obj()
                                .unwrap(); // FIXME_LATER: only path is taken
                            let vars_for_args: Vec<_> = ext
                                .args
                                .iter()
                                .flat_map(|(arg_name, arg)| {
                                    supertype
                                        .vars
                                        .iter()
                                        .filter(|(_var_name, var)| {
                                            var.desc
                                                .param()
                                                .as_ref()
                                                .map(|param| {
                                                    param.to_string() == arg_name.to_string()
                                                })
                                                .unwrap_or(false)
                                        })
                                        .map(|(var_name, var)| {
                                            (
                                                var_name,
                                                Var {
                                                    desc: arg.clone(),
                                                    opt: var.opt,
                                                },
                                            )
                                        })
                                })
                                .collect();
                            let mut required: Vec<_> = vars_for_args
                                .iter()
                                .filter(|(_, var)| !var.opt)
                                .map(|(var_name, _)| var_name.to_string())
                                .collect();
                            required.sort();
                            let supertype_schema = Schema {
                                r#type: Some("object".to_string()),
                                properties: {
                                    vars_for_args
                                        .iter()
                                        .map(|(var_name, var)| {
                                            (
                                                var_name.to_string(),
                                                Schema::of_var(
                                                    &Box::new(var.clone()),
                                                    var_name,
                                                    context,
                                                ),
                                            )
                                        })
                                        .collect()
                                },
                                required: required,
                                ..Default::default()
                            };
                            (def_name.clone(), supertype_schema)
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut schemas: HashMap<_, _> = defs
            .clone()
            .iter()
            .filter(|(_name, def)| {
                def.obj()
                    .iter()
                    .flat_map(|obj| obj.vars.iter().flat_map(|(_, var)| var.desc.param()))
                    .collect::<Vec<_>>()
                    .is_empty()
            })
            .map(|(name, def)| {
                (
                    name.clone(),
                    Schema::of_def(def.clone(), name, None, context),
                )
            })
            .collect();

        schemas.extend(with_synth_adt_refs_schemas.clone());
        schemas.extend(with_synth_param_subtypes_schemas.clone());

        Components { schemas: schemas }
    }
}
