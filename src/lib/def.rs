use crate::lib::{adt::Adt, desc::Desc, ext::Ext, r#ref::Ref, var::Var};
use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{collections::HashMap, ops::Not};

pub static DEFS: &str = "defs";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub(crate) enum Def {
    Alias(Alias),
    Bool(Bool),
    Const(Const),
    Dec(Dec),
    Enum(Enum),
    Int(Int),
    Map(Box<Map>),
    Obj(Obj),
    Seq(Box<Seq>),
    Str(Str),
    Struct(Struct),
}

impl Def {
    pub(crate) fn obj(&self) -> Option<&Obj> {
        match self {
            Def::Obj(obj) => Some(obj),
            _ => None,
        }
    }

    pub(crate) fn refs(&self) -> Vec<Ref> {
        match self {
            Def::Alias(alias) => vec![alias.r#ref.clone()],
            Def::Map(map) => map.key.refs().into_iter().chain(map.val.refs()).collect(),
            Def::Obj(obj) => obj
                .vars
                .iter()
                .flat_map(|(_name, var)| var.desc.refs())
                .collect(),
            Def::Seq(seq) => seq.item.refs(),
            _ => Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Alias {
    #[serde(flatten)]
    pub r#ref: Ref,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Bool {
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Const {
    pub val: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<Box<Desc>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Dec {
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Enum {
    pub vals: EnumVals,
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    pub null: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum EnumVals {
    Int(Vec<i64>),
    Str(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Int {
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool,
}

// TODO_LATER: for http gen, validation, whether a key is string-like
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Map {
    pub key: Desc,
    pub val: Desc,
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Obj {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mix: Vec<Ref>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub vars: HashMap<String, Box<Var>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adt: Option<Adt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Seq {
    pub item: Desc,
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Str {
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Struct {}
