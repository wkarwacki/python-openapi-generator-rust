use std::collections::HashMap;
use convert_case::{Case, Casing};

use serde::{Deserialize, Serialize};
use std::ops::Not;
use serde_yaml::Value;
use typetag::serde;
use def::DEFS;

use crate::adt::Adt;
use crate::context::Context;
use crate::def;
use crate::def::{Alias, Bool, Const, Dec, Def, Enum, EnumVals, Int, Map, Obj, Seq, Str, Struct};
use crate::desc::Desc;
use crate::open_api::components::COMPONENTS;
use crate::open_api::context::Context as OpenApiContext;
use crate::open_api::open_api::{Discriminator, OpenApi};
use crate::r#ref::Ref;
use crate::var::Var;

pub static SCHEMAS: &str = "schemas";
pub fn schemas_path() -> String {
    format!("/{COMPONENTS}/{SCHEMAS}")
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
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
    pub(crate) fn of_def(def: Def, name: String, default: Option<Value>, context: &Context) -> Schema {
        match def {
            Def::Alias(alias) => {
                Schema::of_ref(&alias.r#ref)
            },
            Def::Bool(_) => {
                Schema {
                    r#type: Some("boolean".to_string()),
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
                    default: default,
                    _ref: None,
                }
            },
            Def::Const(_const) => Schema {
                r#type: Some(match _const.val {
                    Value::Null => unreachable!("null value is not allowed in const"),
                    Value::Bool(_) => "boolean",
                    Value::Number(_) => "double",
                    Value::String(_) => "string",
                    Value::Sequence(_) => "array",
                    Value::Mapping(_) => "object",
                    Value::Tagged(_) => "object"
                }.to_string()),
                properties: HashMap::new(),
                additional_properties: None,
                required: Vec::new(),
                nullable: false,
                all_of: Vec::new(),
                one_of: Vec::new(),
                discriminator: None,
                items: match _const.val.clone() {
                    Value::Sequence(_) => Some(Box::new(Schema {
                        r#type: Some("string".to_string()), // FIXME_LATER parse all kinds of values
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
                        _ref: None,
                    })),
                    _ => None
                },
                r#enum: Vec::new(),
                format: None,
                r#const: Some(_const.val),
                default: default,
                _ref: None,
            },
            Def::Dec(_) => {
                Schema {
                    r#type: Some("double".to_string()),
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
                    default: default,
                    _ref: None,
                }
            }
            Def::Enum(Enum{vals, null: _}) => {
                Schema {
                    r#type: Some("string".to_string()),
                    properties: HashMap::new(),
                    additional_properties: None,
                    required: Vec::new(),
                    nullable: false,
                    all_of: Vec::new(),
                    one_of: Vec::new(),
                    discriminator: None,
                    items: None,
                    r#enum: match vals {
                        EnumVals::Int(vec) => vec.iter().map(|i| i.to_string()).collect(),
                        EnumVals::Str(vec) => vec.clone(),
                      },
                    format: None,
                    r#const: None,
                    default: default,
                    _ref: None,
                }
            }
            Def::Int(_) => {
                Schema {
                    r#type: Some("integer".to_string()),
                    properties: HashMap::new(),
                    additional_properties: None,
                    required: Vec::new(),
                    nullable: false,
                    all_of: Vec::new(),
                    one_of: Vec::new(),
                    discriminator: None,
                    items: None,
                    r#enum: Vec::new(),
                    format: Some("int64".to_string()),
                    r#const: None,
                    default: default,
                    _ref: None,
                }
            }
            Def::Map(map) => {
                Schema {
                    r#type: Some("object".to_string()),
                    properties: HashMap::new(),
                    additional_properties: Some(Box::new(Schema::of_desc(&map.val, "additionalProperties".to_string(), None, context))),
                    required: Vec::new(),
                    nullable: false,
                    all_of: Vec::new(),
                    one_of: Vec::new(),
                    discriminator: None,
                    items: None,
                    r#enum: Vec::new(),
                    format: None,
                    r#const: None,
                    default: default,
                    _ref: None,
                }
            }
            Def::Obj(obj) => {
                Schema {
                    r#type: if obj.mix.is_empty() { Some("object".to_string()) } else { None },
                    properties: if obj.mix.is_empty() { obj.vars.iter().map(|(name, var)| (name.clone(), Schema::of_var(var, name.clone(), context))).collect::<HashMap<_, _>>() } else { HashMap::new() },
                    additional_properties: None,
                    required: {
                        if obj.mix.is_empty() { obj.vars.iter().filter(|(_, var)| !var.opt).map(|(name, _)| name.clone()).collect::<Vec<_>>() } else { Vec::new() }
                    },
                    nullable: false,
                    all_of: {
                        let mut all_of = obj.mix.iter().map(|r#ref| Schema::of_ref(&r#ref)).collect::<Vec<_>>();
                        if !all_of.is_empty() && !obj.vars.is_empty() {
                            all_of.push(Schema {
                                r#type: Some("object".to_string()),
                                properties: obj.vars.iter().map(|(name, var)| (name.clone(), Schema::of_var(var, name.clone(), context))).collect::<HashMap<_, _>>(),
                                additional_properties: None,
                                required: obj.vars.iter().filter(|(_, var)| !var.opt).map(|(name, _)| name.clone()).collect::<Vec<_>>(),
                                nullable: false,
                                all_of: Vec::new(),
                                one_of: Vec::new(),
                                discriminator: None,
                                items: None,
                                r#enum: Vec::new(),
                                format: None,
                                r#const: None,
                                default: None,
                                _ref: None,
                            });
                        }
                        all_of
                    },
                    one_of: Vec::new(),
                    discriminator: obj.adt.as_ref().map(|adt| Discriminator {
                        property_name: adt.var.clone(),
                        mapping: adt.map.iter()
                            .map(|(n, _)| (n.clone(), {
                                let subname = n.to_case(Case::UpperCamel);
                                let schemas_path = schemas_path();
                                format!("#{schemas_path}/{name}{subname}") // FIXME_LATER: src is not taken
                            }))
                            .collect::<HashMap<_, _>>(),
                    }),
                    items: None,
                    r#enum: Vec::new(),
                    format: None,
                    r#const: None,
                    default: default,
                    _ref: None,
                }
            }
            Def::Seq(seq) => {
                Schema {
                    r#type: Some("array".to_string()),
                    properties: HashMap::new(),
                    additional_properties: None,
                    required: Vec::new(),
                    nullable: false,
                    all_of: Vec::new(),
                    one_of: Vec::new(),
                    discriminator: None,
                    items: Some(Box::new(Schema::of_desc(&seq.item, "items".to_string(), None, context))),
                    r#enum: Vec::new(),
                    format: None,
                    r#const: None,
                    default: default,
                    _ref: None,
                }
            }
            Def::Str(_) => {
                Schema {
                    r#type: Some("string".to_string()),
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
                    default: default,
                    _ref: None,
                }
            }
            Def::Struct(_) => {
                Schema {
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
                    default: default,
                    _ref: None,
                }
            }
        }
    }

    pub(crate) fn of_var(var: &Box<Var>, name: String, context: &Context) -> Schema {
        Schema::of_desc(&var.desc, name, None, context)
    }

    pub fn of_desc(desc: &Desc, name: String, default: Option<Value>, context: &Context) -> Schema {
        desc.r#ref()
            .as_ref()
            .map(|r#ref| Schema::of_ref(&r#ref))
            .or_else(|| desc.def().as_ref().map(|def| Schema::of_def(def.clone().clone(), name, default, context)))
            .unwrap()
    }

    fn of_ref(r#ref: &Ref) -> Schema {
        Schema {
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
            _ref: Some(Schema::openapi_path(r#ref)),
        }
    }

    pub fn openapi_path(r#ref: &Ref) -> String {
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
        self.all_of.is_empty() && self.properties.is_empty() && self.discriminator.is_none() && self.additional_properties.is_none() && self._ref.is_none()
    }

    pub(crate) fn def(&self, name: String, context: &OpenApiContext) -> Def {
        match self.clone().r#const {
            None => self.r#type
                .as_ref()
                .map(|r#type| match r#type.as_str() {
                    "array" => Def::Seq(Box::new(Seq {
                        item: self.items.as_ref().unwrap().clone().desc(name.clone(), context),
                        null: self.nullable
                    })),
                    "boolean" => Def::Bool(Bool {
                        null: self.nullable
                    }),
                    "double" | "number" => Def::Dec(Dec {
                        null: self.nullable
                    }),
                    "integer" => Def::Int(Int {
                        null: self.nullable
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
                                            null: self.nullable
                                        })),
                                        val: additional_properties.desc(name.clone(), context),
                                        null: self.nullable
                                    }
                                    ))
                                })
                                .unwrap_or({
                                    let vars = self
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
                                        .collect::<HashMap<_, _>>();
                                    Def::Obj(Obj {
                                        ext: None,
                                        mix: Vec::new(),
                                        vars: vars,
                                        adt: self.adt(name.clone(), context),
                                        null: self.nullable
                                    })
                                })
                        }
                    }
                    "string" => if self.r#enum.is_empty() {Def::Str(Str {
                        null: self.nullable
                    })} else {
                        Def::Enum(Enum{vals: EnumVals::Str(self.r#enum.clone()), null: self.nullable})
                    },
                    _ => unreachable!(),
                })
                .unwrap_or_else(|| {
                    if self.empty() {
                        Def::Struct(Struct {})
                    } else {
                        self.clone()._ref.map(|r#ref| Def::Alias(Alias{r#ref: OpenApi::trust_ref(r#ref)})).unwrap_or_else(|| Def::Obj(Obj {
                            ext: None,
                            mix: self
                                .all_of
                                .iter()
                                .flat_map(|schema| schema.clone().desc(name.clone(), context).r#ref().cloned())
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
                            null: self.nullable
                        }))
                    }
                }),
            Some(value) => Def::Const(Const {
                val: value,
                desc: None
            }),
        }
    }

    pub fn adt(&self, name: String, context: &OpenApiContext) -> Option<Adt> {
        self.clone().discriminator.map(|discriminator| Adt {
            var: discriminator.property_name.clone(),
            map: discriminator
                .mapping
                .iter()
                .map(|(n, path)| {
                    (n.clone(), {
                        let schema: Schema = context.resolve(path.clone());
                        let sub_def = schema
                            .with_mapped_all_of()
                            .def(name.clone(), context);
                        sub_def.obj()
                            .map(|obj| Obj {
                                ext: None,
                                mix: obj
                                    .mix
                                    .iter()
                                    .filter(|&mix| {
                                        let defs = DEFS;
                                        mix.path
                                            != format!("{defs}.{name}")
                                    }) // FIXME_LATER: src is not taken
                                    .map(|_ref| _ref.clone())
                                    .collect::<Vec<_>>(),
                                vars: obj.clone().vars,
                                adt: obj.clone().adt,
                                null: self.nullable
                            }).unwrap()
                    })
                })
                .collect::<HashMap<_, _>>(),
        })
    }

    pub fn desc(self, name: String, context: &OpenApiContext) -> Desc {
        self._ref
            .as_ref()
            .map(|r| Desc::Ref(OpenApi::trust_ref(r.clone())))
            .unwrap_or_else(|| {
                Desc::Def(self.def(name, context))
            })
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
            nullable: false,
            all_of: {
                let mut all_of = schema
                    .all_of
                    .iter()
                    .map(|s| s.with_mapped_all_of())
                    .collect::<Vec<_>>();
                all_of.append(&mut all_of.iter().flat_map(|ss| ss.all_of.clone()).collect());
                all_of
            },
            one_of: Vec::new(),
            discriminator: schema.discriminator,
            items: schema.items,
            r#enum: schema.r#enum,
            format: None,
            r#const: None,
            default: None,
            _ref: schema._ref,
        }
    }
}
