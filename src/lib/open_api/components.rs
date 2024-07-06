use std::collections::HashMap;

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use def::DEFS;

use crate::lib::context::Context;
use crate::lib::def;
use crate::lib::def::Def;
use crate::lib::def::Def::Obj;
use crate::lib::open_api::schema::{Schema, schemas_path};
use crate::lib::var::Var;

pub static COMPONENTS: &str = "components";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Components {
    pub schemas: HashMap<String, Schema>,
}

impl Default for Components {
    fn default() -> Self {
        Components {
            schemas: HashMap::new(),
        }
    }
}

impl Components {
    pub fn is_empty(&self) -> bool {
        self.schemas.is_empty()
    }
    pub fn of(defs: HashMap<String, Def>, context: &Context) -> Components {
        let with_synth_adt_refs_schemas = defs
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
                                        r#type: None,
                                        properties: HashMap::new(),
                                        additional_properties: None,
                                        required: Vec::new(),
                                        nullable: false,
                                        all_of: {
                                            let mut vec = Vec::new();
                                            vec.push(Schema {
                                                r#type: None,
                                                properties: HashMap::new(),
                                                additional_properties: None,
                                                required: Vec::new(),
                                                nullable: false,
                                                all_of: Vec::new(),
                                                one_of: Vec::new(),
                                                discriminator: None,
                                                items: None,
                                                r#enum: Vec::new(),
                                                format: None,
                                                r#const: None,
                                                default: None,
                                                _ref: Some({
                                                    let schemas_path = schemas_path();
                                                    format!("#{schemas_path}/{name}")
                                                }
                                                ),
                                            });
                                            vec.push(Schema::of_def(
                                                Obj(subtype),
                                                subname.clone(),
                                                None,
                                                context,
                                            ));
                                            vec
                                        },
                                        one_of: Vec::new(),
                                        discriminator: None,
                                        items: None,
                                        r#enum: Vec::new(),
                                        format: None,
                                        r#const: None,
                                        default: None,
                                        _ref: None,
                                    },
                                )
                            })
                            .collect::<HashMap<_, _>>()
                    })
                    .collect::<HashMap<_, _>>()
            })
            .collect::<HashMap<_, _>>();

        let with_synth_param_subtypes_schemas = defs
            .iter()
            .flat_map(|(def_name, def)| def.obj().iter().flat_map(|obj| obj.ext.iter().map(|ext| {
                let supertype = defs.iter().find(|(d_name, _d)| ext.r#ref.path.clone() == (DEFS.to_string() + "." + d_name.as_str())).unwrap().1.obj().unwrap(); // FIXME_LATER: only path is taken
                let vars_for_args = ext.args.iter()
                    .flat_map(|(arg_name, arg)| supertype.vars.iter().filter(|(_var_name, var)| var.desc.param().as_ref().map(|param| param.to_string() == arg_name.to_string()).unwrap_or(false)).map(|(var_name, var)| (var_name, Var {
                        desc: arg.clone(),
                        opt: var.opt,
                    }))).collect::<Vec<_>>();
                let mut required: Vec<_> = vars_for_args.iter().filter(|(_, var)| !var.opt).map(|(var_name, _)| var_name.to_string()).collect();
                required.sort();
                let supertype_schema = Schema {
                    r#type: Some("object".to_string()),
                    properties: {
                        vars_for_args.iter().map(|(var_name, var)| (var_name.to_string(), Schema::of_var(&Box::new(var.clone()), var_name.to_string(), context))).collect()
                    },
                    all_of: Vec::new(),
                    one_of: Vec::new(),
                    additional_properties: None,
                    required: required,
                    nullable: false,
                    discriminator: None,
                    items: None,
                    r#enum: Vec::new(),
                    format: None,
                    r#const: None,
                    default: None,
                    _ref: None,
                };
                (def_name.clone(), supertype_schema)
            })).collect::<Vec<_>>())
            .collect::<HashMap<_, _>>();

        let mut schemas = defs
            .clone()
            .iter()
            .filter(|(_name, def)| def.obj().iter().flat_map(|obj| obj.vars.iter().flat_map(|(_, var)| var.desc.param())).collect::<Vec<_>>().is_empty())
            .map(|(name, def)| {
                (
                    name.clone(),
                    Schema::of_def(def.clone(), name.clone(), None, context),
                )
            })
            .collect::<HashMap<_, _>>();

        schemas.extend(with_synth_adt_refs_schemas.clone());
        schemas.extend(with_synth_param_subtypes_schemas.clone());

        Components { schemas: schemas }
    }
}
