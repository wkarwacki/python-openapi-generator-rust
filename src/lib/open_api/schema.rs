use crate::lib::{
    adt::Adt,
    context::Context,
    def,
    def::{Alias, Bool, Const, Dec, Def, Enum, EnumVals, Int, Map, Obj, Seq, Str, Struct},
    desc::Desc,
    open_api::{
        components::COMPONENTS,
        context::Context as OpenApiContext,
        open_api::{Discriminator, OpenApi},
    },
    r#ref::Ref,
    var::Var,
};
use convert_case::{Case, Casing};
use def::DEFS;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{collections::HashMap, ops::Not};
use typetag::serde;

pub static SCHEMAS: &str = "schemas";
pub(crate) fn schemas_path() -> String {
    format!("/{COMPONENTS}/{SCHEMAS}")
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub properties: HashMap<String, Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Box<Schema>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    pub nullable: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub all_of: Vec<Schema>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub one_of: Vec<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<Discriminator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub r#enum: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#const: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _ref: Option<String>,
}

impl Schema {
    pub(crate) fn of_def(
        def: Def,
        name: &str,
        default: Option<Value>,
        context: &Context,
    ) -> Schema {
        match def {
            Def::Alias(alias) => Schema::of_ref(&alias.r#ref),
            Def::Bool(_) => Schema {
                r#type: Some("boolean".to_string()),
                default: default,
                ..Default::default()
            },
            Def::Const(_const) => Schema {
                r#type: Some(
                    match _const.val {
                        Value::Null => unreachable!("null value is not allowed in const"),
                        Value::Bool(_) => "boolean",
                        Value::Number(_) => "double",
                        Value::String(_) => "string",
                        Value::Sequence(_) => "array",
                        Value::Mapping(_) => "object",
                        Value::Tagged(_) => "object",
                    }
                    .to_string(),
                ),
                items: match _const.val.clone() {
                    Value::Sequence(_) => Some(Box::new(Schema {
                        r#type: Some("string".to_string()), // FIXME_LATER parse all kinds of values
                        ..Default::default()
                    })),
                    _ => None,
                },
                r#const: Some(_const.val),
                default: default,
                ..Default::default()
            },
            Def::Dec(_) => Schema {
                r#type: Some("double".to_string()),
                default: default,
                ..Default::default()
            },
            Def::Enum(Enum { vals, null: _ }) => Schema {
                r#type: Some("string".to_string()),
                r#enum: match vals {
                    EnumVals::Int(vec) => vec.iter().map(i64::to_string).collect(),
                    EnumVals::Str(vec) => vec.clone(),
                },
                default: default,
                ..Default::default()
            },
            Def::Int(_) => Schema {
                r#type: Some("integer".to_string()),
                format: Some("int64".to_string()),
                default: default,
                ..Default::default()
            },
            Def::Map(map) => Schema {
                r#type: Some("object".to_string()),
                additional_properties: Some(Box::new(Schema::of_desc(
                    &map.val,
                    "additionalProperties",
                    None,
                    context,
                ))),
                default: default,
                ..Default::default()
            },
            Def::Obj(obj) => {
                Schema {
                    r#type: if obj.mix.is_empty() {
                        Some("object".to_string())
                    } else {
                        None
                    },
                    properties: if obj.mix.is_empty() {
                        obj.vars
                            .iter()
                            .map(|(name, var)| (name.clone(), Schema::of_var(var, name, context)))
                            .collect()
                    } else {
                        HashMap::new()
                    },
                    required: {
                        if obj.mix.is_empty() {
                            obj.vars
                                .iter()
                                .filter(|(_, var)| !var.opt)
                                .map(|(name, _)| name.clone())
                                .collect()
                        } else {
                            Vec::new()
                        }
                    },
                    all_of: {
                        let mut all_of: Vec<_> = obj.mix.iter().map(Schema::of_ref).collect();
                        if !all_of.is_empty() && !obj.vars.is_empty() {
                            all_of.push(Schema {
                                r#type: Some("object".to_string()),
                                properties: obj
                                    .vars
                                    .iter()
                                    .map(|(name, var)| {
                                        (name.clone(), Schema::of_var(var, name, context))
                                    })
                                    .collect(),
                                required: obj
                                    .vars
                                    .iter()
                                    .filter(|(_, var)| !var.opt)
                                    .map(|(name, _)| name.clone())
                                    .collect(),
                                ..Default::default()
                            });
                        }
                        all_of
                    },
                    discriminator: obj.adt.as_ref().map(|adt| Discriminator {
                        property_name: adt.var.clone(),
                        mapping: adt
                            .map
                            .iter()
                            .map(|(n, _)| {
                                (n.clone(), {
                                    let subname = n.to_case(Case::UpperCamel);
                                    let schemas_path = schemas_path();
                                    format!("#{schemas_path}/{name}{subname}") // FIXME_LATER: src is not taken
                                })
                            })
                            .collect(),
                    }),
                    default: default,
                    ..Default::default()
                }
            }
            Def::Seq(seq) => Schema {
                r#type: Some("array".to_string()),
                items: Some(Box::new(Schema::of_desc(&seq.item, "items", None, context))),
                default: default,
                ..Default::default()
            },
            Def::Str(_) => Schema {
                r#type: Some("string".to_string()),
                default: default,
                ..Default::default()
            },
            Def::Struct(_) => Schema {
                default: default,
                ..Default::default()
            },
        }
    }

    pub(crate) fn of_var(var: &Box<Var>, name: &str, context: &Context) -> Schema {
        Schema::of_desc(&var.desc, name, None, context)
    }

    pub(crate) fn of_desc(
        desc: &Desc,
        name: &str,
        default: Option<Value>,
        context: &Context,
    ) -> Schema {
        desc.r#ref()
            .as_ref()
            .cloned()
            .map(Schema::of_ref)
            .or_else(|| {
                desc.def()
                    .as_ref()
                    .map(|&def| Schema::of_def(def.clone(), name, default, context))
            })
            .unwrap()
    }

    fn of_ref(r#ref: &Ref) -> Schema {
        Schema {
            _ref: Some(Schema::openapi_path(r#ref)),
            ..Default::default()
        }
    }

    pub(crate) fn openapi_path(r#ref: &Ref) -> String {
        let (src, path) = r#ref.src_and_path();
        let mut result = path;
        result.remove(0);
        result.insert(0, "#");
        result.insert(1, COMPONENTS);
        result.insert(2, SCHEMAS);
        src.iter().for_each(|s| result.insert(0, s.as_str()));
        result.join("/")
    }

    fn empty(&self) -> bool {
        self.all_of.is_empty()
            && self.properties.is_empty()
            && self.discriminator.is_none()
            && self.additional_properties.is_none()
            && self._ref.is_none()
    }

    pub(crate) fn def(&self, name: String, context: &OpenApiContext) -> Def {
        match self.clone().r#const {
            None => self
                .r#type
                .as_ref()
                .map(|r#type| match r#type.as_str() {
                    "array" => Def::Seq(Box::new(Seq {
                        item: self
                            .items
                            .as_ref()
                            .unwrap()
                            .clone()
                            .desc(name.clone(), context),
                        null: self.nullable,
                    })),
                    "boolean" => Def::Bool(Bool {
                        null: self.nullable,
                    }),
                    "double" | "number" => Def::Dec(Dec {
                        null: self.nullable,
                    }),
                    "integer" => Def::Int(Int {
                        null: self.nullable,
                    }),
                    "object" => {
                        if self.empty() {
                            Def::Struct(Struct {})
                        } else {
                            self.additional_properties
                                .clone()
                                .map(|additional_properties| {
                                    Def::Map(Box::new(Map {
                                        key: Desc::Def(Def::Str(Str {
                                            null: self.nullable,
                                        })),
                                        val: additional_properties.desc(name.clone(), context),
                                        null: self.nullable,
                                    }))
                                })
                                .unwrap_or({
                                    let vars: HashMap<_, _> = self
                                        .properties
                                        .iter()
                                        .map(|(n, schema)| {
                                            (
                                                n.clone(),
                                                Box::new(Var {
                                                    desc: schema.clone().desc(n.clone(), context),
                                                    opt: !self.required.contains(n),
                                                }),
                                            )
                                        })
                                        .collect();
                                    Def::Obj(Obj {
                                        ext: None,
                                        mix: Vec::new(),
                                        vars: vars,
                                        adt: self.adt(name.clone(), context),
                                        null: self.nullable,
                                    })
                                })
                        }
                    }
                    "string" => {
                        if self.r#enum.is_empty() {
                            Def::Str(Str {
                                null: self.nullable,
                            })
                        } else {
                            Def::Enum(Enum {
                                vals: EnumVals::Str(self.r#enum.clone()),
                                null: self.nullable,
                            })
                        }
                    }
                    _ => unreachable!(),
                })
                .unwrap_or_else(|| {
                    if self.empty() {
                        Def::Struct(Struct {})
                    } else {
                        self.clone()
                            ._ref
                            .map(|r#ref| {
                                Def::Alias(Alias {
                                    r#ref: OpenApi::trust_ref(r#ref),
                                })
                            })
                            .unwrap_or_else(|| {
                                Def::Obj(Obj {
                                    ext: None,
                                    mix: self
                                        .all_of
                                        .iter()
                                        .flat_map(|schema| {
                                            schema
                                                .clone()
                                                .desc(name.clone(), context)
                                                .r#ref()
                                                .cloned()
                                        })
                                        .collect(),
                                    vars: self
                                        .properties
                                        .iter()
                                        .map(|(n, schema)| {
                                            (
                                                n.clone(),
                                                Box::new(Var {
                                                    desc: schema.clone().desc(n.clone(), context),
                                                    opt: !self.required.contains(n),
                                                }),
                                            )
                                        })
                                        .collect(),
                                    adt: self.adt(name, context),
                                    null: self.nullable,
                                })
                            })
                    }
                }),
            Some(value) => Def::Const(Const {
                val: value,
                desc: None,
            }),
        }
    }

    pub(crate) fn adt(&self, name: String, context: &OpenApiContext) -> Option<Adt> {
        self.clone().discriminator.map(|discriminator| Adt {
            var: discriminator.property_name.clone(),
            map: discriminator
                .mapping
                .iter()
                .map(|(n, path)| {
                    (n.clone(), {
                        let schema: Schema = context.resolve(path.clone());
                        let sub_def = schema.with_mapped_all_of().def(name.clone(), context);
                        sub_def
                            .obj()
                            .map(|obj| Obj {
                                ext: None,
                                mix: obj
                                    .mix
                                    .iter()
                                    .filter(|&mix| {
                                        let defs = DEFS;
                                        mix.path != format!("{defs}.{name}")
                                    }) // FIXME_LATER: src is not taken
                                    .map(|_ref| _ref.clone())
                                    .collect(),
                                vars: obj
                                    .clone()
                                    .vars
                                    .iter()
                                    .filter(|(&ref name, _)| {
                                        name.clone() != discriminator.property_name
                                    })
                                    .map(|(name, var)| (name.clone(), var.clone()))
                                    .collect(),
                                adt: obj.clone().adt,
                                null: self.nullable,
                            })
                            .unwrap()
                    })
                })
                .collect(),
        })
    }

    pub(crate) fn desc(self, name: String, context: &OpenApiContext) -> Desc {
        self._ref
            .as_ref()
            .map(|r| Desc::Ref(OpenApi::trust_ref(r.clone())))
            .unwrap_or_else(|| Desc::Def(self.def(name, context)))
    }

    pub(crate) fn with_mapped_all_of(&self) -> Schema {
        let schema = self.clone();
        Schema {
            r#type: schema.r#type,
            properties: {
                let mut properties = schema.properties.clone();
                schema
                    .all_of
                    .iter()
                    .flat_map(|s| {
                        let mut properties: HashMap<_, _> = s
                            .properties
                            .iter()
                            .map(|(name, ss)| (name.clone(), ss.with_mapped_all_of()))
                            .clone()
                            .collect();
                        s.all_of
                            .iter()
                            .flat_map(|ss| ss.with_mapped_all_of().properties)
                            .for_each(|entry| {
                                properties.insert(entry.0, entry.1);
                                ()
                            });
                        properties
                    })
                    .for_each(|(key, value)| {
                        properties.insert(key, value);
                    });
                properties
            },
            additional_properties: schema.additional_properties,
            required: schema
                .all_of
                .iter()
                .flat_map(|s| s.required.clone())
                .collect(),
            all_of: {
                let mut all_of: Vec<_> = schema
                    .all_of
                    .iter()
                    .map(Schema::with_mapped_all_of)
                    .collect();
                all_of.append(&mut all_of.iter().flat_map(|ss| ss.all_of.clone()).collect());
                all_of
            },
            discriminator: schema.discriminator,
            items: schema.items,
            r#enum: schema.r#enum,
            _ref: schema._ref,
            ..Default::default()
        }
    }
}
